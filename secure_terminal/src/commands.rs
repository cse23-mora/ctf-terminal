/// Command execution module
/// Handles all shell command processing and execution

use crate::encryption::{decode, base64_encode};
use crate::filesystem::FileSystem;
use js_sys::Date;
use wasm_bindgen::prelude::JsValue;

/// Executes a shell command on the virtual filesystem
///
/// # Arguments
/// * `fs` - Mutable reference to the filesystem
/// * `input` - The command string to execute
///
/// # Returns
/// Command output as a string
pub fn execute_command(fs: &mut FileSystem, input: &str) -> String {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return "".to_string();
    }

    let cmd = parts[0];
    let args = &parts[1..];
    let now = Date::now();

    match cmd {
        "pwd" => handle_pwd(fs),
        "cd" => handle_cd(fs, args),
        "ls" => handle_ls(fs, args),
        "cat" => handle_cat(fs, args),
        "mkdir" => handle_mkdir(fs, args, now),
        "touch" => handle_touch(fs, args, now),
        "rm" => handle_rm(fs, args),
        "date" => handle_date(now),
        "echo" => handle_echo(args),
        "whoami" => handle_whoami(),
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

/// rm - Remove file or empty directory
fn handle_rm(fs: &mut FileSystem, args: &[&str]) -> String {
    if args.is_empty() {
        return "Usage: rm <filename>".to_string();
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
    decode(&[36, 26, 25, 20, 18, 18, 37, 21, 40, 11, 38, 18, 36, 37])
}

/// help - Display available commands
fn handle_help() -> String {
    "Available commands: ls, cd, pwd, cat, mkdir, touch, rm, date, echo, whoami, downld, clear"
        .to_string()
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
