# Secure Terminal - Architecture

## Overview
The codebase has been refactored into separate, modular components for better maintainability and clarity.

## Module Structure

### 📁 `src/lib.rs` - Main Entry Point
- **Purpose**: Core WebAssembly bindings and global state management
- **Key Components**:
  - `run_command()` - Main WASM function exposed to JavaScript
  - `FS` - Global filesystem instance (thread-safe with Mutex)
  - Module declarations and re-exports

### 📁 `src/encryption.rs` - Encryption & Encoding
- **Purpose**: Handle data encryption, decryption, and encoding operations
- **Key Functions**:
  - `decode(data)` - XOR decryption with SECRET_KEY (0x53)
  - `encode(text)` - XOR encryption
  - `base64_encode(input)` - Base64 encoding for file downloads

### 📁 `src/filesystem.rs` - Virtual Filesystem
- **Purpose**: Simulates a Unix-like filesystem with files and directories
- **Key Structures**:
  - `FileType` - Enum (File, Directory)
  - `FileNode` - Represents a single file/directory with content and timestamp
  - `FileSystem` - Main struct managing all nodes and current path
- **Key Methods**:
  - `new()` - Initialize filesystem with standard directory structure
  - `resolve_path()` - Convert relative paths to absolute paths
  - `create_dir()`, `create_file()` - Create filesystem nodes
  - `list_directory()` - Get directory contents
  - `delete()` - Remove files/empty directories
  - Path utilities: `exists()`, `is_dir()`, `is_file()`, `normalize_path()`

### 📁 `src/commands.rs` - Command Execution
- **Purpose**: Process and execute shell commands
- **Key Functions**:
  - `execute_command()` - Main dispatcher for all commands
  - Individual command handlers:
    - `handle_pwd()` - Print working directory
    - `handle_cd()` - Change directory
    - `handle_ls()` - List directory contents
    - `handle_cat()` - Display file contents
    - `handle_mkdir()` - Create directory
    - `handle_touch()` - Create empty file
    - `handle_rm()` - Remove files/directories
    - `handle_date()` - Display current date/time
    - `handle_echo()` - Echo arguments
    - `handle_whoami()` - Display user (encrypted)
    - `handle_help()` - Display help
    - `handle_download()` - Download file as base64

## Data Flow

```
JavaScript/Frontend
        ↓
   run_command() [lib.rs]
        ↓
   execute_command() [commands.rs]
        ↓
   Individual command handlers
        ↓
   FileSystem queries/modifications [filesystem.rs]
        ↓
   Encryption/Decryption as needed [encryption.rs]
        ↓
   Return output to JavaScript
```

## Design Principles

1. **Separation of Concerns**
   - Each module has a single responsibility
   - Clear boundaries between encryption, filesystem, and command logic

2. **Encapsulation**
   - Filesystem operations are isolated in `filesystem.rs`
   - Encryption logic is concentrated in `encryption.rs`
   - All command handling is in `commands.rs`

3. **Maintainability**
   - Easy to add new commands by adding handlers to `commands.rs`
   - Easy to modify filesystem behavior in one place
   - Encryption changes don't affect command logic

4. **Testability**
   - Each module can be tested independently
   - Minimal interdependencies
   - Pure functions where possible

## Adding New Features

### Adding a New Command
1. Create a handler function in `commands.rs`: `fn handle_newcmd()`
2. Add a match arm in `execute_command()`
3. Implement the logic using filesystem methods

### Extending Filesystem
1. Add new methods to `FileSystem` impl block in `filesystem.rs`
2. Use existing methods like `exists()`, `resolve_path()`, etc.

### Improving Encryption
1. Modify functions in `encryption.rs`
2. No need to change command or filesystem logic

## Current Commands
- `pwd` - Print working directory
- `cd` - Change directory
- `ls` - List directory
- `cat` - Display file content
- `mkdir` - Create directory
- `touch` - Create file
- `cp` - Copy file/directory
- `mv` - Move/rename file/directory
- `date` - Show date/time
- `echo` - Echo text
- `whoami` - Show user
- `downld` - Download file as base64
- `clear` - Clear screen
- `help` - Show available commands
