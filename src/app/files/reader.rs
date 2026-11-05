use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// FileRead tool — reads file or directory contents.
pub struct FileReader;

impl FileReader {
    /// Read a file's contents as a string.
    pub fn read_file(path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }

    /// Read a directory's entries (names only).
    pub fn read_dir(path: &Path) -> io::Result<Vec<DirEntry>> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let is_dir = path.is_dir();
            entries.push(DirEntry { name, path, is_dir });
        }
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(entries)
    }

    /// Read a file and return its lines with line numbers.
    pub fn read_file_with_lines(path: &Path) -> io::Result<Vec<(usize, String)>> {
        let content = fs::read_to_string(path)?;
        Ok(content
            .lines()
            .enumerate()
            .map(|(i, line)| (i + 1, line.to_string()))
            .collect())
    }

    /// Check if path exists.
    pub fn exists(path: &Path) -> bool {
        path.exists()
    }
}

/// Directory entry information.
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_exists() {
        assert!(FileReader::exists(Path::new("Cargo.toml")));
        assert!(!FileReader::exists(Path::new("nonexistent_file_xyz.rs")));
    }
}
