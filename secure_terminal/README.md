# Secure Terminal - WebAssembly Encrypted Terminal Emulator

A fast, secure, web-based terminal emulator built with Rust and WebAssembly. Features an encrypted virtual filesystem with a command-line interface that runs directly in your browser.

## 🚀 Features

- **WebAssembly-powered** - Fast, compiled Rust code running in the browser
- **Virtual Filesystem** - Complete Unix-like filesystem simulation
- **Encrypted Storage** - XOR encryption for sensitive data
- **Modern CLI** - Familiar Unix commands (ls, cd, cat, etc.)
- **Responsive UI** - Beautiful terminal interface with theme support
- **Base64 Download** - Download encrypted files securely

## 📋 Prerequisites

Before you begin, ensure you have installed:

- **Rust** (1.56+) - [Install Rust](https://rustup.rs/)
- **wasm-pack** - WebAssembly packaging tool
  ```bash
  cargo install wasm-pack
  ```
- **Node.js** (14+) - [Install Node.js](https://nodejs.org/)
- **npm** - Usually comes with Node.js

## 🛠️ Installation & Setup

### 1. Clone or Navigate to Project
```bash
cd /media/cse23/storage/mytests/terminal-web/secure_terminal
```

### 2. Install Dependencies
```bash
# No additional npm dependencies needed for the core project
# The project uses wasm-bindgen which is managed via Cargo
```

### 3. Build WebAssembly
```bash
wasm-pack build --target web
```

This command:
- Compiles Rust code to WebAssembly
- Generates JavaScript bindings
- Creates the `pkg/` directory with `.wasm` and `.js` files

### 4. Start Development Server
```bash
# From the root directory (terminal-web)
cd ..

# Start a simple HTTP server
python3 -m http.server 8000

# Or use Node's http-server if installed
# npx http-server
```

### 5. Open in Browser
Navigate to `http://localhost:8000/web/` and start using the terminal!

## 📖 Usage Guide

### Getting Started

Once the terminal loads, you'll see:
```
cse23@admin:~$
```

Start by exploring the filesystem:

### Basic Commands

#### Navigation
```bash
pwd              # Print working directory
cd /home         # Change to /home directory
cd ..            # Go to parent directory
```

#### File Operations
```bash
ls               # List directory contents
cat projects.txt # View file contents
cat contact.txt  # View contact information
mkdir mydir      # Create new directory
touch newfile    # Create empty file
cp projects.txt backup.txt  # Copy file
mv backup.txt archive.txt   # Move/rename file
```

#### System Info
```bash
whoami           # Display current user
date             # Show current date/time
help             # List all available commands
```

#### Utilities
```bash
echo "Hello"           # Print text
clear                  # Clear terminal
downld projects.txt    # Download file as base64
```

### Built-in Files

The system includes pre-loaded files:

- **`/home/projects.txt`** - Encrypted list of projects
- **`/home/contact.txt`** - Encrypted contact information

Try `cat` to view them and watch the decryption happen!

## 🏗️ Project Structure

```
secure_terminal/
├── Cargo.toml                 # Rust package configuration
├── ARCHITECTURE.md            # Detailed architecture documentation
├── README.md                  # This file
├── src/
│   ├── lib.rs               # Main entry point, WASM bindings
│   ├── encryption.rs        # XOR encryption/encoding functions
│   ├── filesystem.rs        # Virtual filesystem implementation
│   └── commands.rs          # Command execution handlers
├── pkg/                      # Generated WebAssembly files (after build)
│   ├── secure_terminal.js
│   ├── secure_terminal.d.ts
│   ├── secure_terminal_bg.wasm
│   └── package.json
└── target/                   # Build artifacts
```

For detailed architecture, see [ARCHITECTURE.md](ARCHITECTURE.md)

## 🔧 Development

### Modular Design

The project is organized into 4 main modules:

1. **encryption.rs** - Data encryption/decryption
2. **filesystem.rs** - Virtual filesystem management
3. **commands.rs** - Command parsing and execution
4. **lib.rs** - WASM entry point

### 🛠️ Quick Text Encryption Tool

Located at `src/encrypt.rs` - A simple utility script for encrypting text strings into byte arrays.

#### How to Use

**Step 1: Open the encrypt.rs file**
```bash
# Edit the file
nano src/encrypt.rs
# or
code src/encrypt.rs
```

**Step 2: Modify the text you want to encrypt**

Find this line and change the text:
```rust
let text = "cse23_guest"; // CHANGE THIS TO WHATEVER YOU WANT
```

Examples:
```rust
let text = "my_password";
let text = "user@domain.com";
let text = "Secret Message";
```

**Step 3: Save and run**
```bash
# Run the encryption script
rustc src/encrypt.rs -o encrypt && ./encrypt
```

Or using cargo (after compiling):
```bash
cargo run --bin encrypt 2>/dev/null || rustc src/encrypt.rs -o encrypt && ./encrypt
```

**Step 4: Copy the output**

The script will output something like:
```
vec![34, 42, 18, 31, 26, 45, 56, 18, 31, 26, 45, 56]
```

Copy this exact output.

#### Example Walkthrough

**Want to encrypt: "ArchLinux"**

1. Edit `src/encrypt.rs`:
```rust
let text = "ArchLinux"; // Changed from "cse23_guest"
```

2. Run it:
```bash
rustc src/encrypt.rs -o encrypt && ./encrypt
```

3. Output:
```
vec![18, 21, 30, 11, 24, 24, 18, 43, 43]
```

4. Add to [src/filesystem.rs](src/filesystem.rs):
```rust
let distro = vec![18, 21, 30, 11, 24, 24, 18, 43, 43];
fs.create_file("/home/myfile.txt", distro, 0.0);
```

5. Rebuild the WebAssembly:
```bash
wasm-pack build --target web
```

6. Test in terminal:
```bash
cat /home/myfile.txt
# Output: ArchLinux
```

#### Important Details

**Secret Key**
- The encryption uses: `0x53` (can be seen in the code)
- This must match the key in [src/encryption.rs](src/encryption.rs)
- If you change the key in encrypt.rs, also change it in encryption.rs

**Key Line in encryption.rs:**
```rust
const SECRET_KEY: u8 = 0x53; // Line 6 in encryption.rs
```

**Key Line in encrypt.rs:**
```rust
let key = 0x53; // Line 2 in encrypt.rs
```

If you change one, change the other!

#### Quick Command Reference

For fastest workflow:
```bash
# 1. Edit the text
nano src/encrypt.rs

# 2. Run encryption
rustc src/encrypt.rs -o encrypt && ./encrypt

# 3. Copy output, add to filesystem.rs

# 4. Rebuild
wasm-pack build --target web
```



### Adding New Commands

To add a new command:

1. Create a handler function in `src/commands.rs`:
   ```rust
   fn handle_newcommand(fs: &mut FileSystem, args: &[&str]) -> String {
       // Implementation
       "output".to_string()
   }
   ```

2. Add a match arm in `execute_command()`:
   ```rust
   "newcommand" => handle_newcommand(fs, args),
   ```

3. Update `handle_help()` to include the new command

4. Rebuild:
   ```bash
   wasm-pack build --target web
   ```

### Building for Production

For optimized builds:
```bash
wasm-pack build --target web --release
```

This produces smaller, faster WebAssembly files.

## 🔐 Security Notes

- **Current encryption** is XOR with a fixed key - suitable for obfuscation, not cryptographic security
- **For production**: Replace with proper encryption (AES-GCM is already in dependencies)
- All operations run client-side - no data sent to servers
- Files exist only in browser memory during session

## 📦 Dependencies

### Rust Crates
- `wasm-bindgen` - Rust-JavaScript interop
- `js-sys` - JavaScript bindings
- `lazy_static` - Global state management
- `aes-gcm`, `generic-array` - Available for real encryption

### JavaScript
- None required! Pure Rust + WebAssembly

## 🐛 Troubleshooting

### Build fails: "wasm-pack not found"
```bash
cargo install wasm-pack
```

### WebAssembly module not loading
- Check browser console for errors
- Ensure `pkg/` directory exists after build
- Clear browser cache (Ctrl+F5)

### Commands not recognized
- Use `help` to see available commands
- Commands are case-sensitive
- Some commands require arguments (e.g., `cat filename`)

### File permissions error when building
```bash
chmod +x ~/wasm-pack-bin  # If applicable
```

## 📚 Learning Resources

- [Rust WASM Book](https://rustwasm.org/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.org/docs/wasm-bindgen/)
- [WebAssembly MDN Docs](https://developer.mozilla.org/en-US/docs/WebAssembly/)

## 🎨 UI Features

The web interface includes:

- **Theme Support** - Multiple color schemes
- **Command History** - Use arrow keys to navigate
- **Auto-focus** - Input field ready immediately
- **Real-time Output** - Instant command feedback

### Themes Available
- Matrix (default)
- Sunset
- Dracula
- Light

## 📝 License & Credit

Created by cse23 (@cse23)

GitHub: [@cse23](https://github.com/cse23)  
Email: [hello@cse23.org](mailto:hello@cse23.org)

## 🤝 Contributing

Contributions welcome! Areas to explore:

- Real AES-GCM encryption
- Additional Unix commands
- File upload functionality
- User authentication
- Shell script support
- Performance optimizations

## 📞 Support

Having issues? Check:
1. [ARCHITECTURE.md](ARCHITECTURE.md) for code structure
2. Browser console for error messages
3. Ensure you've run `wasm-pack build --target web`
4. Try clearing browser cache

---

**Happy exploring!** 🚀

Start your journey: `cd /home && ls`
