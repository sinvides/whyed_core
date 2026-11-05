use std::fs;
use std::io;
use std::path::Path;

/// FileEdit tool — performs a string replacement in an existing file.
pub struct FileEditor;

impl FileEditor {
    /// Replace the first occurrence of `old` with `new` in `path`.
    pub fn edit_file(path: &Path, old: &str, new: &str) -> io::Result<EditResult> {
        let content = fs::read_to_string(path)?;
        if !content.contains(old) {
            return Ok(EditResult::NotFound);
        }
        let new_content = content.replacen(old, new, 1);
        fs::write(path, &new_content)?;
        Ok(EditResult::Applied)
    }

    /// Replace all occurrences of `old` with `new` in `path`.
    pub fn edit_file_all(path: &Path, old: &str, new: &str) -> io::Result<EditResult> {
        let content = fs::read_to_string(path)?;
        if !content.contains(old) {
            return Ok(EditResult::NotFound);
        }
        let new_content = content.replace(old, new);
        fs::write(path, &new_content)?;
        Ok(EditResult::Applied)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EditResult {
    Applied,
    NotFound,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_edit() {
        let path = PathBuf::from(".whyed_test_edit.txt");
        let _ = std::fs::write(&path, "foo bar foo");
        let res = FileEditor::edit_file(&path, "foo", "baz").unwrap();
        assert_eq!(res, EditResult::Applied);
        let back = std::fs::read_to_string(&path).unwrap();
        assert_eq!(back, "baz bar foo");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_edit_not_found() {
        let path = PathBuf::from(".whyed_test_edit2.txt");
        let _ = std::fs::write(&path, "hello");
        let res = FileEditor::edit_file(&path, "xyz", "abc").unwrap();
        assert_eq!(res, EditResult::NotFound);
        let _ = std::fs::remove_file(&path);
    }
}
