use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Glob-style file search (simple recursive prefix/suffix match).
pub struct FileSearch;

impl FileSearch {
    /// Find files under `root` matching `pattern` (suffix match on file name).
    pub fn glob(root: &Path, pattern: &str) -> io::Result<Vec<PathBuf>> {
        let mut results = Vec::new();
        Self::glob_recursive(root, pattern, &mut results)?;
        results.sort();
        Ok(results)
    }

    fn glob_recursive(dir: &Path, pattern: &str, results: &mut Vec<PathBuf>) -> io::Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                Self::glob_recursive(&path, pattern, results)?;
            } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with(pattern) {
                    results.push(path);
                }
            }
        }
        Ok(())
    }

    /// Grep: search for `needle` in files under `root` matching `pattern`.
    pub fn grep(root: &Path, needle: &str, pattern: &str) -> io::Result<Vec<GrepMatch>> {
        let files = Self::glob(root, pattern)?;
        let mut matches = Vec::new();
        for path in files {
            if let Ok(content) = fs::read_to_string(&path) {
                for (i, line) in content.lines().enumerate() {
                    if line.contains(needle) {
                        matches.push(GrepMatch {
                            path: path.clone(),
                            line_number: i + 1,
                            line: line.to_string(),
                        });
                    }
                }
            }
        }
        Ok(matches)
    }
}

#[derive(Debug, Clone)]
pub struct GrepMatch {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob_rs() {
        let files = FileSearch::glob(Path::new("src"), ".rs").unwrap();
        assert!(!files.is_empty());
    }

    #[test]
    fn test_grep() {
        let results = FileSearch::grep(Path::new("src"), "fn ", ".rs").unwrap();
        assert!(!results.is_empty());
    }
}
