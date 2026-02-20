use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex; // Required for state management

#[macro_use]
extern crate lazy_static;

const SECRET_KEY: u8 = 0x53;

fn decode(data: &[u8]) -> String {
    let decoded: Vec<u8> = data.iter().map(|&b| b ^ SECRET_KEY).collect();
    String::from_utf8(decoded).unwrap_or_else(|_| "??".to_string())
}

lazy_static! {
    // Tracks the current path in memory
    static ref CURRENT_DIR: Mutex<String> = Mutex::new("/".to_string());

    static ref FILES: HashMap<&'static str, Vec<u8>> = {
        let mut m = HashMap::new();
        // Encrypted: "1. WASM Terminal (Rust)\n2. Encrypted Portfolio"
        m.insert("projects.txt", vec![68, 11, 85, 34, 18, 32, 26, 85, 39, 18, 31, 24, 20, 25, 20, 23, 85, 107, 37, 30, 36, 11, 85, 99, 10, 85, 23, 27, 20, 31, 10, 33, 11, 18, 27, 85, 33, 26, 31, 11, 25, 26, 23, 20, 26]);
        // Encrypted: "GitHub: @sangeeth\nEmail: hello@sangeeth.dev"
        m.insert("contact.txt", vec![26, 24, 37, 21, 38, 17, 13, 11, 85, 11, 36, 42, 45, 18, 16, 16, 37, 21, 10, 22, 24, 20, 24, 27, 107, 85, 21, 18, 27, 27, 26, 11, 11, 30, 25, 20, 18, 18, 37, 21, 107, 27, 18, 35]);
        m
    };
}

#[wasm_bindgen]
pub fn run_command(input: &str) -> String {
    let mut dir = CURRENT_DIR.lock().unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() { return "".to_string(); }

    let cmd = parts[0];
    let args = &parts[1..];

    match cmd {
        "pwd" => dir.clone(),

        "cd" => {
            if args.is_empty() || args[0] == "/" || args[0] == "~" {
                *dir = "/".to_string();
                "".to_string()
            } else if args[0] == "home" || args[0] == "/home" {
                *dir = "/home".to_string();
                "".to_string()
            } else {
                format!("cd: no such directory: {}", args[0])
            }
        },

        "ls" => {
            if *dir == "/" {
                "home/    etc/    bin/".to_string()
            } else if *dir == "/home" {
                let mut keys: Vec<&str> = FILES.keys().cloned().collect();
                keys.sort();
                keys.join("    ")
            } else {
                "".to_string()
            }
        },

        "cat" => {
            if args.is_empty() { return "Usage: cat <filename>".to_string(); }
            // Only allow cat if we are in the home directory where files live
            if *dir == "/home" {
                match FILES.get(args[0]) {
                    Some(content_bytes) => decode(content_bytes),
                    None => format!("cat: {}: No such file", args[0])
                }
            } else {
                format!("cat: {}: No such file in this directory", args[0])
            }
        },

        "whoami" => decode(&[36, 26, 25, 20, 18, 18, 37, 21, 40, 11, 38, 18, 36, 37]),
        
        "help" => "Available commands: ls, cd, pwd, cat, whoami, clear".to_string(),

        "clear" => "CLEARED".to_string(),

        _ => format!("command not found: {}. Type 'help' for info.", cmd),
    }
}