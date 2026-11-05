use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const APP_NAME: &str = "Whyed Code";
pub const APP_MODEL: &str = "whyed-dev";

pub(crate) struct Accent {
    pub(crate) color: &'static str,
    pub(crate) dim: &'static str,
}

pub(crate) const ACCENTS: &[(&str, Accent)] = &[
    ("orange", Accent { color: "\x1b[38;5;173m", dim: "\x1b[38;5;130m" }),
    ("blue",   Accent { color: "\x1b[38;5;67m",  dim: "\x1b[38;5;60m" }),
    ("green",  Accent { color: "\x1b[38;5;71m",  dim: "\x1b[38;5;65m" }),
    ("purple", Accent { color: "\x1b[38;5;97m",  dim: "\x1b[38;5;61m" }),
];

pub(crate) struct Colors {
    pub(crate) reset: &'static str,
    pub(crate) bold: &'static str,
    pub(crate) dim: &'static str,
    pub(crate) accent: &'static str,
    pub(crate) accent_dim: &'static str,
    pub(crate) gray: &'static str,
    pub(crate) white: &'static str,
    pub(crate) red: &'static str,
}

impl Colors {
    fn new(accent_name: &str) -> Self {
        let acc = ACCENTS
            .iter()
            .find(|(n, _)| *n == accent_name)
            .map(|(_, a)| a)
            .unwrap_or(&ACCENTS[0].1);
        Colors {
            reset: "\x1b[0m",
            bold: "\x1b[1m",
            dim: "\x1b[2m",
            accent: acc.color,
            accent_dim: acc.dim,
            gray: "\x1b[38;5;245m",
            white: "\x1b[38;5;255m",
            red: "\x1b[38;5;203m",
        }
    }
}

fn parse_ini(content: &str) -> HashMap<String, HashMap<String, String>> {
    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut section = String::from("default");
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line[1..line.len() - 1].trim().to_string();
        } else if let Some(eq) = line.find('=') {
            let key = line[..eq].trim().to_string();
            let val = line[eq + 1..].trim().to_string();
            result.entry(section.clone()).or_default().insert(key, val);
        }
    }
    result
}

pub(crate) fn write_ini(path: &Path, data: &HashMap<String, HashMap<String, String>>) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut out = String::new();
    let mut sections: Vec<&String> = data.keys().collect();
    sections.sort();
    for section in sections {
        out.push_str(&format!("[{}]\n", section));
        let mut keys: Vec<&String> = data[section].keys().collect();
        keys.sort();
        for k in keys {
            out.push_str(&format!("{} = {}\n", k, data[section][k]));
        }
        out.push('\n');
    }
    fs::write(path, out)
}

pub(crate) fn dirs_home() -> Option<PathBuf> {
    env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
}

pub struct Config {
    pub(crate) path: PathBuf,
    pub(crate) data: HashMap<String, HashMap<String, String>>,
    pub(crate) colors: Colors,
}

impl Config {
    pub fn new() -> Self {
        let home = dirs_home().unwrap_or_else(|| PathBuf::from("."));
        let path = home.join(".whyed").join("config.ini");

        let mut data: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut app = HashMap::new();
        app.insert("accent".to_string(), "orange".to_string());
        app.insert("version".to_string(), "v0.0.1-dev".to_string());
        data.insert("app".to_string(), app);

        let mut ai = HashMap::new();
        ai.insert("api".to_string(), "token-abc123".to_string());
        ai.insert("api_url".to_string(), "http://localhost:8080/v1".to_string());
        data.insert("ai".to_string(), ai);

        let mut history = HashMap::new();
        history.insert("enabled".to_string(), "true".to_string());
        history.insert("max_items".to_string(), "100".to_string());
        history.insert("save_path".to_string(), "~/.whyed/history.json".to_string());
        data.insert("history".to_string(), history);

        match fs::read_to_string(&path) {
            Ok(content) => {
                let parsed = parse_ini(&content);
                for (sec, kv) in parsed {
                    let entry = data.entry(sec).or_default();
                    for (k, v) in kv {
                        entry.insert(k, v);
                    }
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                if let Err(write_err) = write_ini(&path, &data) {
                    eprintln!("warning: failed to create config: {}", write_err);
                }
            }
            Err(e) => {
                eprintln!("warning: failed to read config: {}", e);
            }
        }

        let accent = data
            .get("app")
            .and_then(|s| s.get("accent"))
            .cloned()
            .unwrap_or_else(|| "orange".to_string());
        let colors = Colors::new(&accent);

        Config { path, data, colors }
    }

    pub fn app_name(&self) -> String {
        APP_NAME.to_string()
    }

    pub fn model(&self) -> String {
        APP_MODEL.to_string()
    }

    pub fn version(&self) -> String {
        self.data
            .get("app")
            .and_then(|s| s.get("version"))
            .cloned()
            .unwrap_or_else(|| "v0.0.1-dev".to_string())
    }

    pub fn accent(&self) -> String {
        self.data
            .get("app")
            .and_then(|s| s.get("accent"))
            .cloned()
            .unwrap_or_else(|| "orange".to_string())
    }

    pub fn ai_key(&self) -> Option<String> {
        self.data
            .get("ai")
            .and_then(|s| s.get("api"))
            .cloned()
    }

    pub fn ai_url(&self) -> Option<String> {
        self.data
            .get("ai")
            .and_then(|s| s.get("api_url"))
            .cloned()
    }

    pub fn history_enabled(&self) -> bool {
        self.data
            .get("history")
            .and_then(|s| s.get("enabled"))
            .map(|v| v == "true")
            .unwrap_or(true)
    }

    pub fn history_max_items(&self) -> usize {
        self.data
            .get("history")
            .and_then(|s| s.get("max_items"))
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(500)
    }

    pub fn history_save_path(&self) -> PathBuf {
        if let Some(p) = self.data.get("history").and_then(|s| s.get("save_path")) {
            let home = dirs_home().unwrap_or_else(|| PathBuf::from("."));
            let expanded = p.replace("~", &home.to_string_lossy());
            PathBuf::from(expanded)
        } else {
            dirs_home().unwrap_or_else(|| PathBuf::from(".")).join(".whyed").join("history.txt")
        }
    }

    pub fn refresh_colors(&mut self) {
        self.colors = Colors::new(&self.accent());
    }
}
