use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub struct History {
    pub(crate) file_path: PathBuf,
    pub(crate) items: Vec<String>,
    max_items: usize,
    enabled: bool,
}

impl History {
    pub fn new(file_path: &Path, max_items: usize, enabled: bool) -> Self {
        let mut h = History {
            file_path: file_path.to_path_buf(),
            items: Vec::new(),
            max_items,
            enabled,
        };
        h.load();
        h
    }

    fn load(&mut self) {
        if !self.enabled {
            return;
        }
        match fs::read_to_string(&self.file_path) {
            Ok(content) => {
                self.items = content
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .map(|l| l.to_string())
                    .collect();
                if self.items.len() > self.max_items {
                    let start = self.items.len() - self.max_items;
                    self.items = self.items[start..].to_vec();
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => {
                eprintln!("warning: failed to read history: {}", e);
            }
        }
    }

    pub fn add(&mut self, entry: &str) -> io::Result<()> {
        if !self.enabled {
            return Ok(());
        }
        if self.items.last().map(|s| s == entry).unwrap_or(false) {
            return Ok(());
        }
        self.items.push(entry.to_string());
        if self.items.len() > self.max_items {
            self.items.remove(0);
            self.rewrite()?;
            return Ok(());
        }
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;
        writeln!(f, "{}", entry)?;
        Ok(())
    }

    fn rewrite(&self) -> io::Result<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut out = String::new();
        for item in &self.items {
            out.push_str(item);
            out.push('\n');
        }
        fs::write(&self.file_path, out)
    }
}
