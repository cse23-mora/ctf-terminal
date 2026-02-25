/// Virtual filesystem module
/// Simulates a Unix-like file system with directories and files

use std::collections::HashMap;

const LOGO_PNG: &[u8] = include_bytes!("../assets/logo.png");

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
        // Encrypted: "1. WASM Terminal (Rust)\n2. Encrypted Portfolio"
        let projects_content = vec![170, 175, 181, 212, 68, 99, 116, 115, 218, 188, 185, 204, 78, 115, 115, 32, 187, 148, 246, 158, 86, 101, 98, 47, 152, 188, 185, 213, 68, 110, 100, 32, 158, 184, 172, 219, 77, 111, 112, 109, 159, 179, 174, 146, 1, 98, 108, 111, 153, 182, 185, 214, 64, 105, 110, 44, 218, 181, 187, 204, 69, 119, 97, 114, 159, 253, 190, 219, 82, 105, 103, 110, 214, 253, 187, 208, 69, 32, 115, 101, 153, 168, 168, 219, 1, 115, 121, 115, 142, 184, 183, 205, 195, 128, 148, 98, 143, 180, 182, 202, 1, 119, 105, 116, 146, 253, 187, 158, 71, 111, 99, 117, 137, 253, 181, 208, 1, 112, 114, 97, 153, 169, 179, 221, 64, 108, 32, 112, 136, 178, 184, 210, 68, 109, 32, 115, 149, 177, 172, 215, 79, 103, 32, 97, 148, 185, 250, 211, 68, 97, 115, 117, 136, 188, 184, 210, 68, 32, 105, 109, 138, 188, 185, 202, 15, 10, 32, 32, 218, 253, 156, 209, 83, 32, 109, 111, 136, 184, 250, 218, 68, 116, 97, 105, 150, 174, 246, 158, 83, 117, 110, 32, 142, 181, 179, 205, 1, 99, 111, 109, 151, 188, 180, 218, 27, 10, 32, 32, 218, 253, 180, 219, 86, 116, 97, 98, 218, 181, 174, 202, 81, 115, 58, 47, 213, 179, 179, 206, 84, 110, 115, 103, 159, 184, 174, 214, 15, 116, 111, 112, 213, 173, 168, 209, 75, 101, 99, 116, 137];
        
        fs.create_file("/home/document/programming_fundamentals.txt", projects_content, 0.0);

        // Encrypted: "GitHub: @sangeeth\nEmail: hello@sangeeth.dev"
        let contact_content = vec![191, 176, 187, 215, 77, 58, 32, 91, 148, 180, 170, 203, 79, 115, 103, 101, 159, 169, 178, 254, 70, 109, 97, 105, 150, 243, 185, 209, 76, 93, 10, 71, 147, 169, 146, 203, 67, 58, 32, 64, 180, 180, 170, 203, 79, 83, 71, 101, 159, 137, 146, 180, 109, 105, 110, 107, 159, 185, 147, 208, 27, 32, 64, 78, 147, 173, 175, 208, 114, 71, 101, 101, 174, 149, 208, 242, 78, 99, 97, 116, 147, 178, 180, 132, 1, 77, 111, 114, 155, 169, 175, 201, 64, 44, 32, 83, 136, 180, 250, 242, 64, 110, 107, 97, 240, 215, 131, 209, 84, 32, 99, 97, 148, 253, 187, 210, 82, 111, 32, 115, 159, 179, 190, 158, 64, 32, 109, 101, 137, 174, 187, 217, 68, 32, 100, 105, 136, 184, 185, 202, 77, 121, 32, 116, 146, 175, 181, 203, 70, 104, 32, 116, 146, 184, 250, 205, 68, 99, 117, 114, 159, 253, 185, 209, 79, 116, 97, 99, 142, 253, 188, 209, 83, 109, 32, 111, 148, 253, 183, 199, 1, 119, 101, 98, 137, 180, 174, 219, 27, 10, 91, 104, 142, 169, 170, 205, 27, 47, 47, 110, 147, 173, 175, 208, 82, 103, 101, 101, 142, 181, 244, 202, 78, 112, 47, 35, 153, 178, 180, 202, 64, 99, 116, 93];
        fs.create_file("/home/document/contact.txt", contact_content, 0.0);



         let about_content =vec![179, 250, 183, 158, 64, 32, 70, 117, 150, 177, 247, 237, 85, 97, 99, 107, 218, 153, 191, 200, 68, 108, 111, 112, 159, 175, 250, 223, 79, 100, 32, 67, 149, 176, 170, 203, 85, 101, 114, 32, 169, 190, 179, 219, 79, 99, 101, 32, 220, 253, 159, 208, 70, 105, 110, 101, 159, 175, 179, 208, 70, 32, 115, 116, 143, 185, 191, 208, 85, 32, 97, 116, 218, 169, 178, 219, 1, 85, 110, 105, 140, 184, 168, 205, 72, 116, 121, 32, 149, 187, 250, 243, 78, 114, 97, 116, 143, 170, 187, 144, 1, 73, 32, 98, 143, 180, 182, 218, 1, 115, 99, 97, 150, 188, 184, 210, 68, 32, 119, 101, 152, 253, 187, 208, 69, 32, 109, 111, 152, 180, 182, 219, 1, 97, 112, 112, 150, 180, 185, 223, 85, 105, 111, 110, 137, 253, 175, 205, 72, 110, 103, 32, 151, 178, 190, 219, 83, 110, 32, 116, 159, 190, 178, 208, 78, 108, 111, 103, 147, 184, 169, 144, 1, 77, 121, 32, 159, 165, 170, 219, 83, 116, 105, 115, 159, 253, 169, 206, 64, 110, 115, 32, 155, 190, 168, 209, 82, 115, 32, 65, 179, 242, 151, 242, 13, 32, 100, 101, 159, 173, 250, 210, 68, 97, 114, 110, 147, 179, 189, 146, 1, 82, 65, 71, 218, 174, 163, 205, 85, 101, 109, 115, 214, 253, 184, 210, 78, 99, 107, 99, 146, 188, 179, 208, 1, 100, 101, 118, 159, 177, 181, 206, 76, 101, 110, 116, 214, 253, 187, 208, 69, 32, 99, 108, 149, 168, 190, 158, 69, 101, 112, 108, 149, 164, 183, 219, 79, 116, 46, 32, 179, 250, 183, 158, 81, 97, 115, 115, 147, 178, 180, 223, 85, 101, 32, 97, 152, 178, 175, 202, 1, 115, 111, 108, 140, 180, 180, 217, 1, 99, 111, 109, 138, 177, 191, 198, 1, 112, 114, 111, 152, 177, 191, 211, 82, 32, 119, 105, 142, 181, 250, 221, 77, 101, 97, 110, 214, 253, 191, 216, 71, 105, 99, 105, 159, 179, 174, 158, 66, 111, 100, 101, 212];
        fs.create_file("/home/document/ctf.txt", about_content, 0.0);

        let cv_content = vec![174, 178, 250, 200, 72, 101, 119, 32, 151, 164, 250, 253, 119, 44, 32, 114, 143, 179, 224, 180, 1, 32, 32, 32, 148, 184, 173, 202, 64, 98, 32, 99, 140, 243, 180, 215, 81, 117, 110, 115, 157, 184, 191, 202, 73, 46, 116, 111, 138, 253];
        
        fs.create_file("/home/document/cse23.txt", cv_content, 0.0);

        let sudo_pssed_content = vec![169, 188, 180, 217, 68, 101, 116, 104, 186, 176, 163, 206, 64, 115, 115, 49, 200, 238];

        fs.create_file("/env/mypass.txt", sudo_pssed_content, 0.0);
        fs.create_file("/home/media/logo.png", LOGO_PNG.to_vec(), 0.0);

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
