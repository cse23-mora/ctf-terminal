/// Command execution module
/// Handles all shell command processing and execution

use crate::encryption::{decode, base64_encode};
use crate::filesystem::FileSystem;
use crate::SudoState;
use crate::TerminalState;
use js_sys::Date;
use wasm_bindgen::prelude::JsValue;

const SUDO_PASSWORD: &str = "sangeeth"; // Default sudo password
const THEMES: [&str; 4] = ["matrix", "sunset", "dracula", "light"];

/// Executes a shell command on the virtual filesystem
///
/// # Arguments
/// * `fs` - Mutable reference to the filesystem
/// * `sudo` - Mutable reference to sudo state
/// * `term` - Mutable reference to terminal session state
/// * `input` - The command string to execute
///
/// # Returns
/// Command output as a string
pub fn execute_command(
    fs: &mut FileSystem,
    sudo: &mut SudoState,
    term: &mut TerminalState,
    input: &str,
) -> String {
    // Check if we're waiting for a password
    if sudo.waiting_for_password {
        // Verify the password
        if input.trim() == SUDO_PASSWORD {
            sudo.waiting_for_password = false;
            sudo.authenticated = true;
            
            // Execute the pending command
            if let Some(cmd_str) = sudo.pending_command.take() {
                return execute_command(fs, sudo, term, &cmd_str);
            }
            return "[sudo] authenticated successfully".to_string();
        } else {
            return "[sudo] Sorry, try again.".to_string();
        }
    }

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return "".to_string();
    }

    let cmd = parts[0];
    let args = &parts[1..];
    let now = Date::now();

    // Internal command used by frontend to refresh prompt without polluting history
    if cmd == "__pwd__" {
        return handle_pwd(fs);
    }
    if cmd == "__ls__" {
        return handle_ls(fs, &[]);
    }

    term.history.push(input.trim().to_string());

    // Handle sudo command: sudo <command> [args...]
    if cmd == "sudo" && !args.is_empty() {
        let actual_cmd = args[0];
        let actual_args = &args[1..];
        
        // Build the full command to execute after password
        let full_cmd = format!("{} {}", actual_cmd, actual_args.join(" "));
        
        // Ask for password
        sudo.waiting_for_password = true;
        sudo.pending_command = Some(full_cmd);
        return "[sudo] password: ".to_string();
    }

    match cmd {
        "pwd" => handle_pwd(fs),
        "cd" => handle_cd(fs, args),
        "ls" => handle_ls(fs, args),
        "cat" => handle_cat(fs, args),
        "mkdir" => handle_mkdir(fs, args, now),
        "touch" => handle_touch(fs, args, now),
        "rm" => handle_rm(fs, sudo, args),
        "date" => handle_date(now),
        "echo" => handle_echo(args),
        "whoami" => handle_whoami(),
        "newtab" => handle_newtab(args),
        "history" => handle_history(term),
        "theme" => handle_theme(term, args),
        "help" => handle_help(),
        "clear" => "CLEARED".to_string(),
        "downld" => handle_download(fs, args),
        _ => format!("command not found: {}. Type 'help' for info.", cmd),
    }
}

/// pwd - Print working directory
fn handle_pwd(fs: &FileSystem) -> String {
    fs.current_path.clone()
}

/// cd - Change directory
fn handle_cd(fs: &mut FileSystem, args: &[&str]) -> String {
    if args.is_empty() {
        fs.current_path = "/home".to_string();
        return "".to_string();
    }

    let target = fs.resolve_path(args[0]);
    if fs.exists(&target) && fs.is_dir(&target) {
        fs.current_path = target;
        "".to_string()
    } else {
        format!("cd: {}: No such file or directory", args[0])
    }
}

/// ls - List directory contents
fn handle_ls(fs: &FileSystem, args: &[&str]) -> String {
    let dir_path = if args.is_empty() {
        fs.current_path.clone()
    } else {
        fs.resolve_path(args[0])
    };

    if !fs.exists(&dir_path) {
        return format!("ls: cannot access '{}': No such file or directory", args[0]);
    }

    if !fs.is_dir(&dir_path) {
        return format!("ls: {}: Not a directory", args[0]);
    }

    let entries = fs.list_directory(&dir_path);
    entries.join("    ")
}

/// cat - Display file contents
fn handle_cat(fs: &FileSystem, args: &[&str]) -> String {
    if args.is_empty() {
        return "Usage: cat <filename>".to_string();
    }

    let target = fs.resolve_path(args[0]);

    if !fs.exists(&target) {
        return format!("cat: {}: No such file", args[0]);
    }

    if fs.is_dir(&target) {
        return format!("cat: {}: Is a directory", args[0]);
    }

    if let Some(content) = fs.get_file_content(&target) {
        decode(content)
    } else {
        "Error reading file".to_string()
    }
}

/// mkdir - Create directory
fn handle_mkdir(fs: &mut FileSystem, args: &[&str], now: f64) -> String {
    if args.is_empty() {
        return "Usage: mkdir <directory>".to_string();
    }

    let target = fs.resolve_path(args[0]);

    if fs.exists(&target) {
        format!("mkdir: cannot create directory '{}': File exists", args[0])
    } else {
        fs.create_dir(&target, now);
        "".to_string()
    }
}

/// touch - Create empty file or update timestamp
fn handle_touch(fs: &mut FileSystem, args: &[&str], now: f64) -> String {
    if args.is_empty() {
        return "Usage: touch <filename>".to_string();
    }

    let target = fs.resolve_path(args[0]);

    if !fs.exists(&target) {
        fs.create_file(&target, Vec::new(), now);
    }

    "".to_string()
}

/// rm - Remove file or empty directory (requires sudo authentication)
fn handle_rm(fs: &mut FileSystem, sudo: &SudoState, args: &[&str]) -> String {
    if args.is_empty() {
        return "Usage: rm <filename>".to_string();
    }

    // Check if user is authenticated with sudo
    if !sudo.authenticated {
        return "rm: Permission denied. Use 'sudo <password> rm <filename>' or 'sudo <password>' first.".to_string();
    }

    let target = fs.resolve_path(args[0]);

    if !fs.exists(&target) {
        return format!("rm: cannot remove '{}': No such file or directory", args[0]);
    }

    if fs.is_dir(&target) {
        if !fs.delete(&target) {
            return format!("rm: cannot remove '{}': Directory not empty", args[0]);
        }
    } else {
        fs.delete(&target);
    }

    "".to_string()
}

/// date - Display current date/time
fn handle_date(now: f64) -> String {
    let date = Date::new(&JsValue::from_f64(now));
    date.to_string().into()
}

/// echo - Print arguments
fn handle_echo(args: &[&str]) -> String {
    args.join(" ")
}

/// whoami - Display current user (encrypted)
fn handle_whoami() -> String {
    decode(&[18, 33, 48, 59, 31, 58, 61, 38, 43, 12, 0, 50, 61, 52, 54, 54, 39, 59])
}

/// newtab - Ask frontend to open a URL in a new tab
fn handle_newtab(args: &[&str]) -> String {
    if args.is_empty() {
        return "Usage: newtab [http(s)://]<host-or-url>\nExample: newtab openai.com".to_string();
    }

    let raw = args.join(" ").trim().to_string();
    if raw.chars().any(char::is_whitespace) {
        return format!("newtab: invalid URL '{}'", raw);
    }

    let normalized = if raw.starts_with("http://") || raw.starts_with("https://") {
        raw
    } else if raw.contains("://") {
        return "newtab: only http/https URLs are allowed".to_string();
    } else {
        format!("https://{}", raw)
    };

    format!("NEWTAB:{}", normalized)
}

/// history - Show command history from backend session
fn handle_history(term: &TerminalState) -> String {
    if term.history.is_empty() {
        return "No history yet".to_string();
    }

    term.history
        .iter()
        .enumerate()
        .map(|(i, cmd)| format!("{}  {}", i + 1, cmd))
        .collect::<Vec<String>>()
        .join("\n")
}

/// theme - Set terminal theme (frontend applies, backend validates)
fn handle_theme(term: &mut TerminalState, args: &[&str]) -> String {
    if args.is_empty() {
        return format!(
            "Usage: theme <name>\nAvailable themes: {}",
            THEMES.join(", ")
        );
    }

    let name = args[0];
    if !THEMES.contains(&name) {
        return format!(
            "theme: unknown theme '{}'. Available themes: {}",
            name,
            THEMES.join(", ")
        );
    }

    term.theme = name.to_string();
    format!("THEME:{}", name)
}

/// help - Display available commands
fn handle_help() -> String {
    "Available commands: ls, cd, pwd, cat, mkdir, touch, rm, sudo, date, echo, whoami, history, theme, newtab, downld, clear".to_string()
}

/// downld - Download file (returns base64 encoded content)
fn handle_download(fs: &FileSystem, args: &[&str]) -> String {
    if args.is_empty() {
        return "Usage: download <filename>".to_string();
    }

    let target = fs.resolve_path(args[0]);

    if !fs.exists(&target) {
        return format!("download: {}: No such file", args[0]);
    }

    if fs.is_dir(&target) {
        return format!("download: {}: Is a directory", args[0]);
    }

    if let Some(content) = fs.get_file_content(&target) {
        let filename = args[0].split('/').last().unwrap_or("file");
        let decrypted = decode(content);
        let decrypted_bytes = decrypted.as_bytes();
        let b64 = base64_encode(decrypted_bytes);
        format!("DOWNLOAD:{}:{}", filename, b64)
    } else {
        "Error reading file".to_string()
    }
}
