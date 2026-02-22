/// Secure Terminal - WebAssembly-based encrypted terminal emulator
///
/// This module provides a virtual filesystem with encryption capabilities,
/// exposed as WebAssembly functions for use in web applications.

mod commands;
mod encryption;
mod filesystem;

use wasm_bindgen::prelude::*;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

use filesystem::FileSystem;

/// Sudo session state - tracks authentication and password prompts
pub struct SudoState {
    pub authenticated: bool,
    pub waiting_for_password: bool,
    pub pending_command: Option<String>, // The command waiting for password confirmation
}

impl SudoState {
    pub fn new() -> Self {
        SudoState {
            authenticated: false,
            waiting_for_password: false,
            pending_command: None,
        }
    }
}

/// Terminal session state for backend-managed commands
pub struct TerminalState {
    pub history: Vec<String>,
    pub theme: String,
}

impl TerminalState {
    pub fn new() -> Self {
        TerminalState {
            history: Vec::new(),
            theme: "matrix".to_string(),
        }
    }
}

lazy_static! {
    static ref FS: Mutex<FileSystem> = Mutex::new(FileSystem::new());
    static ref SUDO: Mutex<SudoState> = Mutex::new(SudoState::new());
    static ref TERM: Mutex<TerminalState> = Mutex::new(TerminalState::new());
}

/// Main WebAssembly entry point
/// Executes a command on the virtual filesystem and returns output
///
/// # Arguments
/// * `input` - Command string to execute
///
/// # Returns
/// Command output as a string
#[wasm_bindgen]
pub fn run_command(input: &str) -> String {
    let mut fs = FS.lock().unwrap();
    let mut sudo = SUDO.lock().unwrap();
    let mut term = TERM.lock().unwrap();
    commands::execute_command(&mut fs, &mut sudo, &mut term, input)
}
