use std::fs;
use std::io;
use std::path::Path;

/// FileWrite tool — creates or overwrites a file.
pub struct FileWriter;

impl FileWriter {
    /// Write content to a file, creating parent directories if needed.
    pub fn write_file(path: &Path, content: &str) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)
    }

    /// Append content to a file.
    pub fn append_file(path: &Path, content: &str) -> io::Result<()> {
        use std::io::Write;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut f = fs::OpenOptions::new().create(true).append(true).open(path)?;
        f.write_all(content.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_write_and_read() {
        let path = PathBuf::from(".whyed_test_write.txt");
        let content = "hello from file writer test";
        assert!(FileWriter::write_file(&path, content).is_ok());
        let back = std::fs::read_to_string(&path).unwrap();
        assert_eq!(back, content);
        let _ = std::fs::remove_file(&path);
    }
}
