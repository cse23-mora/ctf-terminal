/// Virtual filesystem module
/// Simulates a Unix-like file system with directories and files

use std::collections::HashMap;

const LOGO_PNG: &[u8] = include_bytes!("../assets/secret.png");

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
    fn parent_path(path: &str) -> Option<String> {
        if path == "/" {
            return None;
        }

        path.rfind('/').map(|idx| {
            if idx == 0 {
                "/".to_string()
            } else {
                path[..idx].to_string()
            }
        })
    }

    fn has_path_prefix(path: &str, base: &str) -> bool {
        path == base || path.starts_with(&(base.to_string() + "/"))
    }

    fn remap_path(path: &str, src: &str, dst: &str) -> String {
        if path == src {
            dst.to_string()
        } else {
            format!("{}/{}", dst, &path[src.len() + 1..])
        }
    }

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
        fs.create_dir("/boot", 0.0);
          fs.create_dir("/sys", 0.0);
            fs.create_dir("/env", 0.0);
             fs.create_dir("/home/document", 0.0);
        fs.create_dir("/home/media", 0.0);


        // Initialize default encrypted files
        // Encrypted: "1. WASM Terminal (Rust)\n2. Encrypted admin"
        let projects_content = vec![170, 141, 235, 158, 109, 65, 66, 58, 215, 253, 151, 223, 88, 32, 98, 101, 218, 184, 187, 205, 88, 32, 116, 104, 155, 179, 250, 210, 64, 115, 116, 32, 131, 184, 187, 204, 43, 80, 80, 50, 218, 145, 155, 252, 27, 45, 32, 68, 149, 179, 253, 202, 1, 102, 111, 103, 159, 169, 250, 202, 78, 32, 97, 100, 158, 253, 185, 209, 76, 109, 101, 110, 142, 174, 250, 215, 79, 115, 105, 100, 159, 253, 174, 214, 68, 32, 99, 111, 158, 184, 246, 158, 104, 102, 32, 99, 149, 185, 191, 158, 79, 111, 116, 32, 141, 178, 168, 213, 82, 45, 62, 32, 163, 178, 175, 158, 70, 101, 116, 32, 142, 181, 191, 158, 76, 97, 114, 107, 137, 253, 188, 209, 83, 32, 116, 104, 149, 184, 169, 158, 66, 111, 109, 109, 159, 179, 174, 205, 15];
        fs.create_file("/home/document/programming_fundamentals.txt", projects_content, 0.0);

        // Encrypted: "GitHub: @cse23\nEmail: hello@cse23.org"
        let contact_content = vec![185, 178, 180, 208, 68, 99, 116, 32, 141, 180, 174, 214, 1, 85, 115, 58, 240, 63, 90, 28, 1, 71, 105, 116, 178, 168, 184, 132, 1, 64, 99, 115, 159, 239, 233, 211, 78, 114, 97, 10, 240, 63, 90, 28, 1, 76, 105, 110, 145, 184, 190, 247, 79, 58, 32, 105, 148, 182, 191, 218, 72, 110, 46, 99, 149, 176, 245, 221, 78, 109, 112, 97, 148, 164, 245, 221, 82, 101, 50, 51, 151, 178, 168, 223, 43, 10, 226, 128, 88, 253, 156, 223, 66, 101, 98, 111, 149, 182, 224, 158, 97, 99, 115, 101, 200, 238, 183, 209, 83, 97, 10, 10, 24, 93, 120, 158, 100, 109, 97, 105, 150, 231, 250, 221, 78, 110, 116, 97, 153, 169, 154, 221, 82, 101, 50, 51, 212, 178, 168, 217];
        fs.create_file("/home/document/contact.txt", contact_content, 0.0);



         let about_content =vec![178, 180, 180, 202, 27, 32, 32, 73, 156, 253, 163, 209, 84, 32, 97, 114, 159, 253, 187, 158, 69, 114, 105, 118, 159, 175, 246, 158, 88, 111, 117, 32, 153, 188, 180, 158, 76, 111, 118, 101, 218, 169, 178, 219, 1, 118, 101, 104, 147, 190, 182, 219, 1, 102, 111, 114, 141, 188, 168, 218, 1, 97, 110, 100, 218, 175, 191, 200, 68, 114, 115, 101, 212];
        fs.create_file("/home/document/ctf.txt", about_content, 0.0);

        let cv_content = vec![217, 144, 151, 251, 99, 10, 45, 32, 183, 188, 189, 219, 1, 77, 97, 116, 146, 184, 250, 251, 74, 97, 32, 66, 155, 253, 208, 158, 1, 32, 32, 32, 218, 253, 129, 243, 84, 116, 116, 105, 155, 181, 250, 243, 84, 114, 97, 108, 147, 169, 178, 223, 83, 97, 110, 93];
        fs.create_file("/home/document/golden_Key.txt", cv_content, 0.0);

        let sudo_pssed_content = vec![185, 142, 159, 197, 98, 48, 109, 101, 165, 142, 191, 219, 126, 69, 110, 106, 149, 164, 167];
        fs.create_file("/env/mypass.txt", sudo_pssed_content, 0.0);
        fs.create_file("/home/media/secret.png", LOGO_PNG.to_vec(), 0.0);

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

    /// Copies a file or directory recursively to a new path
    pub fn copy_path(&mut self, src: &str, dst: &str) -> Result<(), String> {
        if src == "/" {
            return Err("cannot copy root directory".to_string());
        }

        if !self.exists(src) {
            return Err(format!("cannot stat '{}': No such file or directory", src));
        }

        if self.exists(dst) {
            return Err(format!("cannot create '{}': File exists", dst));
        }

        let parent = Self::parent_path(dst)
            .ok_or_else(|| format!("cannot create '{}': Invalid destination", dst))?;
        if !self.exists(&parent) || !self.is_dir(&parent) {
            return Err(format!("cannot create '{}': No such directory", dst));
        }

        if self.is_dir(src) && Self::has_path_prefix(dst, src) {
            return Err("cannot copy a directory into itself".to_string());
        }

        let entries: Vec<(String, FileNode)> = self
            .nodes
            .iter()
            .filter(|(path, _)| Self::has_path_prefix(path, src))
            .map(|(path, node)| (Self::remap_path(path, src, dst), node.clone()))
            .collect();

        for (new_path, node) in entries {
            self.nodes.insert(new_path, node);
        }

        Ok(())
    }

    /// Moves (renames) a file or directory recursively to a new path
    pub fn move_path(&mut self, src: &str, dst: &str) -> Result<(), String> {
        if src == "/" {
            return Err("cannot move root directory".to_string());
        }

        if !self.exists(src) {
            return Err(format!("cannot stat '{}': No such file or directory", src));
        }

        if self.exists(dst) {
            return Err(format!("cannot move to '{}': File exists", dst));
        }

        let parent =
            Self::parent_path(dst).ok_or_else(|| format!("cannot move to '{}': Invalid destination", dst))?;
        if !self.exists(&parent) || !self.is_dir(&parent) {
            return Err(format!("cannot move to '{}': No such directory", dst));
        }

        if self.is_dir(src) && Self::has_path_prefix(dst, src) {
            return Err("cannot move a directory into itself".to_string());
        }

        let entries: Vec<(String, FileNode)> = self
            .nodes
            .iter()
            .filter(|(path, _)| Self::has_path_prefix(path, src))
            .map(|(path, node)| (path.clone(), node.clone()))
            .collect();

        for (old_path, _) in &entries {
            self.nodes.remove(old_path);
        }

        for (old_path, node) in entries {
            let new_path = Self::remap_path(&old_path, src, dst);
            self.nodes.insert(new_path, node);
        }

        if Self::has_path_prefix(&self.current_path, src) {
            self.current_path = Self::remap_path(&self.current_path, src, dst);
        }

        Ok(())
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}
