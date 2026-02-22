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

lazy_static! {
    static ref FS: Mutex<FileSystem> = Mutex::new(FileSystem::new());
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
    commands::execute_command(&mut fs, input)
}