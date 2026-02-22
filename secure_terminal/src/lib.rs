use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use js_sys::Date;

#[macro_use]
extern crate lazy_static;

const SECRET_KEY: u8 = 0x53;

fn decode(data: &[u8]) -> String {
    let decoded: Vec<u8> = data.iter().map(|&b| b ^ SECRET_KEY).collect();
    String::from_utf8(decoded).unwrap_or_else(|_| "??".to_string())
}

#[derive(Clone, PartialEq)]
enum FileType {
    File,
    Directory,
}

#[derive(Clone)]
struct FileNode {
    ftype: FileType,
    content: Vec<u8>,
    #[allow(dead_code)]
    timestamp: f64,
}

struct FileSystem {
    nodes: HashMap<String, FileNode>,
    current_path: String,
}

impl FileSystem {
    fn new() -> Self {
        let mut fs = FileSystem {
            nodes: HashMap::new(),
            current_path: "/home".to_string(),
        };

        // Initialize Root and Home
        fs.create_dir("/", 0.0);
        fs.create_dir("/home", 0.0);
        fs.create_dir("/bin", 0.0);
        fs.create_dir("/etc", 0.0);

        // Initialize default files
        // Encrypted: "1. WASM Terminal (Rust)\n2. Encrypted Portfolio"
        let projects_content = vec![68, 11, 85, 34, 18, 32, 26, 85, 39, 18, 31, 24, 20, 25, 20, 23, 85, 107, 37, 30, 36, 11, 85, 99, 10, 85, 23, 27, 20, 31, 10, 33, 11, 18, 27, 85, 33, 26, 31, 11, 25, 26, 23, 20, 26];
        fs.create_file("/home/projects.txt", projects_content, 0.0);

        // Encrypted: "GitHub: @sangeeth\nEmail: hello@sangeeth.dev"
        let contact_content = vec![26, 24, 37, 21, 38, 17, 13, 11, 85, 11, 36, 42, 45, 18, 16, 16, 37, 21, 10, 22, 24, 20, 24, 27, 107, 85, 21, 18, 27, 27, 26, 11, 11, 30, 25, 20, 18, 18, 37, 21, 107, 27, 18, 35];
        fs.create_file("/home/contact.txt", contact_content, 0.0);

        fs
    }

    fn create_dir(&mut self, path: &str, time: f64) {
        self.nodes.insert(path.to_string(), FileNode {
            ftype: FileType::Directory,
            content: Vec::new(),
            timestamp: time,
        });
    }

    fn create_file(&mut self, path: &str, content: Vec<u8>, time: f64) {
        self.nodes.insert(path.to_string(), FileNode {
            ftype: FileType::File,
            content,
            timestamp: time,
        });
    }

    fn resolve_path(&self, path: &str) -> String {
        if path.starts_with("/") {
            // Absolute
            self.normalize_path(path)
        } else {
            // Relative
            let mut full = self.current_path.clone();
            if !full.ends_with('/') { full.push('/'); }
            full.push_str(path);
            self.normalize_path(&full)
        }
    }

    fn normalize_path(&self, path: &str) -> String {
        let mut stack = Vec::new();
        for part in path.split('/') {
            if part == "" || part == "." { continue; }
            if part == ".." {
                stack.pop();
            } else {
                stack.push(part);
            }
        }
        let res = "/".to_string() + &stack.join("/");
        if res.is_empty() { "/".to_string() } else { res }
    }

    fn exists(&self, path: &str) -> bool {
        self.nodes.contains_key(path)
    }

    fn is_dir(&self, path: &str) -> bool {
        if let Some(node) = self.nodes.get(path) {
            node.ftype == FileType::Directory
        } else {
            false
        }
    }
}

lazy_static! {
    static ref FS: Mutex<FileSystem> = Mutex::new(FileSystem::new());
}

#[wasm_bindgen]
pub fn run_command(input: &str) -> String {
    let mut fs = FS.lock().unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() { return "".to_string(); }

    let cmd = parts[0];
    let args = &parts[1..];
    let now = Date::now();

    match cmd {
        "pwd" => fs.current_path.clone(),

        "cd" => {
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
        },

        "ls" => {
            // List contents of current directory
            // Naive implementation: iterate all keys and check if they start with current_path
            let current = if fs.current_path == "/" { "/".to_string() } else { fs.current_path.clone() + "/" };
            let mut entries = Vec::new();
            
            for (path, node) in &fs.nodes {
                if path.starts_with(&current) {
                    // Check if direct child (no extra slashes)
                    let relative = &path[current.len()..];
                    if !relative.contains('/') && !relative.is_empty() {
                        let suffix = if node.ftype == FileType::Directory { "/" } else { "" };
                        entries.push(format!("{}{}", relative, suffix));
                    }
                }
            }
            entries.sort();
            entries.join("    ")
        },

        "cat" => {
            if args.is_empty() { return "Usage: cat <filename>".to_string(); }
            let target = fs.resolve_path(args[0]);
            if let Some(node) = fs.nodes.get(&target) {
                if node.ftype == FileType::File {
                    decode(&node.content)
                } else {
                    format!("cat: {}: Is a directory", args[0])
                }
            } else {
                format!("cat: {}: No such file", args[0])
            }
        },

        "mkdir" => {
            if args.is_empty() { return "Usage: mkdir <directory>".to_string(); }
            let target = fs.resolve_path(args[0]);
            if fs.exists(&target) {
                format!("mkdir: cannot create directory '{}': File exists", args[0])
            } else {
                fs.create_dir(&target, now);
                "".to_string()
            }
        },

        "touch" => {
            if args.is_empty() { return "Usage: touch <filename>".to_string(); }
            let target = fs.resolve_path(args[0]);
            if !fs.exists(&target) {
                fs.create_file(&target, Vec::new(), now);
            } else {
                // Update timestamp (simplified: just do nothing for now or update it)
            }
            "".to_string()
        },

        "rm" => {
            if args.is_empty() { return "Usage: rm <filename>".to_string(); }
            let target = fs.resolve_path(args[0]);
            if fs.exists(&target) {
                // Check if directory is not empty? 
                // For simplicity, allow deleting anything or check if dir
                if fs.is_dir(&target) {
                    // Simple check: does any other key start with this path?
                    let prefix = target.clone() + "/";
                    let has_children = fs.nodes.keys().any(|k| k.starts_with(&prefix));
                    if has_children {
                        format!("rm: cannot remove '{}': Directory not empty", args[0])
                    } else {
                        fs.nodes.remove(&target);
                        "".to_string()
                    }
                } else {
                    fs.nodes.remove(&target);
                    "".to_string()
                }
            } else {
                format!("rm: cannot remove '{}': No such file or directory", args[0])
            }
        },

        "date" => {
            let date = Date::new(&JsValue::from_f64(now));
            date.to_string().into()
        },

        "echo" => {
            args.join(" ")
        },

        "whoami" => decode(&[36, 26, 25, 20, 18, 18, 37, 21, 40, 11, 38, 18, 36, 37]),

        "help" => "Available commands: ls, cd, pwd, cat, mkdir, touch, rm, date, echo, whoami, downld, clear".to_string(),

        "clear" => "CLEARED".to_string(),

        "downld" => {
            if args.is_empty() { 
                return "Usage: download <filename>".to_string(); 
            }
            let target = fs.resolve_path(args[0]);
            if let Some(node) = fs.nodes.get(&target) {
                if node.ftype == FileType::File {
                    // Decrypt the content first, then encode as base64 for transfer
                    let filename = args[0].split('/').last().unwrap_or("file");
                    let decrypted = decode(&node.content);
                    let decrypted_bytes = decrypted.as_bytes();
                    let b64 = base64_encode(decrypted_bytes);
                    format!("DOWNLOAD:{}:{}", filename, b64)
                } else {
                    format!("download: {}: Is a directory", args[0])
                }
            } else {
                format!("download: {}: No such file", args[0])
            }
        },

        _ => format!("command not found: {}. Type 'help' for info.", cmd),
    }
}

fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in input.chunks(3) {
        let b1 = chunk[0];
        let b2 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b3 = if chunk.len() > 2 { chunk[2] } else { 0 };
        
        let n = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);
        
        result.push(TABLE[((n >> 18) & 63) as usize] as char);
        result.push(TABLE[((n >> 12) & 63) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(TABLE[((n >> 6) & 63) as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(TABLE[(n & 63) as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}