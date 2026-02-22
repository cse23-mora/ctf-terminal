/// Virtual filesystem module
/// Simulates a Unix-like file system with directories and files

use std::collections::HashMap;

/// Represents the type of a filesystem node
#[derive(Clone, PartialEq, Debug)]
pub enum FileType {
    File,
    Directory,
}

/// Represents a single node in the filesystem (file or directory)
#[derive(Clone)]
pub struct FileNode {
    pub ftype: FileType,
    pub content: Vec<u8>,
    #[allow(dead_code)]
    pub timestamp: f64,
}

/// Virtual filesystem implementation
pub struct FileSystem {
    nodes: HashMap<String, FileNode>,
    pub current_path: String,
}

impl FileSystem {
    /// Creates a new filesystem with default structure and files
    pub fn new() -> Self {
        let mut fs = FileSystem {
            nodes: HashMap::new(),
            current_path: "/home".to_string(),
        };

        // Initialize directory structure
        fs.create_dir("/", 0.0);
        fs.create_dir("/home", 0.0);
        fs.create_dir("/bin", 0.0);
        fs.create_dir("/etc", 0.0);

        // Initialize default encrypted files
        // Encrypted: "1. WASM Terminal (Rust)\n2. Encrypted Portfolio"
        let projects_content = vec![
            68, 11, 85, 34, 18, 32, 26, 85, 39, 18, 31, 24, 20, 25, 20, 23, 85, 107, 37, 30, 36,
            11, 85, 99, 10, 85, 23, 27, 20, 31, 10, 33, 11, 18, 27, 85, 33, 26, 31, 11, 25, 26,
            23, 20, 26,
        ];
        fs.create_file("/home/projects.txt", projects_content, 0.0);

        // Encrypted: "GitHub: @sangeeth\nEmail: hello@sangeeth.dev"
        let contact_content = vec![
            26, 24, 37, 21, 38, 17, 13, 11, 85, 11, 36, 42, 45, 18, 16, 16, 37, 21, 10, 22, 24,
            20, 24, 27, 107, 85, 21, 18, 27, 27, 26, 11, 11, 30, 25, 20, 18, 18, 37, 21, 107, 27,
            18, 35,
        ];
        fs.create_file("/home/contact.txt", contact_content, 0.0);

        fs
    }

    /// Creates a new directory at the specified path
    pub fn create_dir(&mut self, path: &str, time: f64) {
        self.nodes.insert(
            path.to_string(),
            FileNode {
                ftype: FileType::Directory,
                content: Vec::new(),
                timestamp: time,
            },
        );
    }

    /// Creates a new file with content at the specified path
    pub fn create_file(&mut self, path: &str, content: Vec<u8>, time: f64) {
        self.nodes.insert(
            path.to_string(),
            FileNode {
                ftype: FileType::File,
                content,
                timestamp: time,
            },
        );
    }

    /// Resolves a path (relative or absolute) to an absolute path
    pub fn resolve_path(&self, path: &str) -> String {
        if path.starts_with("/") {
            self.normalize_path(path)
        } else {
            let mut full = self.current_path.clone();
            if !full.ends_with('/') {
                full.push('/');
            }
            full.push_str(path);
            self.normalize_path(&full)
        }
    }

    /// Normalizes a path by resolving . and .. components
    fn normalize_path(&self, path: &str) -> String {
        let mut stack = Vec::new();
        for part in path.split('/') {
            if part == "" || part == "." {
                continue;
            }
            if part == ".." {
                stack.pop();
            } else {
                stack.push(part);
            }
        }
        let res = "/".to_string() + &stack.join("/");
        if res.is_empty() {
            "/".to_string()
        } else {
            res
        }
    }

    /// Checks if a node exists at the given path
    pub fn exists(&self, path: &str) -> bool {
        self.nodes.contains_key(path)
    }

    /// Checks if a path points to a directory
    pub fn is_dir(&self, path: &str) -> bool {
        self.nodes
            .get(path)
            .map(|node| node.ftype == FileType::Directory)
            .unwrap_or(false)
    }

    /// Checks if a path points to a file
    #[allow(dead_code)]
    pub fn is_file(&self, path: &str) -> bool {
        self.nodes
            .get(path)
            .map(|node| node.ftype == FileType::File)
            .unwrap_or(false)
    }

    /// Retrieves file content at the given path
    pub fn get_file_content(&self, path: &str) -> Option<&Vec<u8>> {
        self.nodes.get(path).map(|node| &node.content)
    }

    /// Lists direct children of a directory
    pub fn list_directory(&self, dir_path: &str) -> Vec<String> {
        let current = if dir_path == "/" {
            "/".to_string()
        } else {
            dir_path.to_string() + "/"
        };

        let mut entries = Vec::new();

        for (path, node) in &self.nodes {
            if path.starts_with(&current) {
                let relative = &path[current.len()..];
                if !relative.contains('/') && !relative.is_empty() {
                    let suffix = if node.ftype == FileType::Directory {
                        "/"
                    } else {
                        ""
                    };
                    entries.push(format!("{}{}", relative, suffix));
                }
            }
        }

        entries.sort();
        entries
    }

    /// Deletes a file or empty directory
    pub fn delete(&mut self, path: &str) -> bool {
        if !self.exists(path) {
            return false;
        }

        if self.is_dir(path) {
            let prefix = path.to_string() + "/";
            if self.nodes.keys().any(|k| k.starts_with(&prefix)) {
                return false; // Directory not empty
            }
        }

        self.nodes.remove(path).is_some()
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}
