use std::io::{self, Write};
use std::time::SystemTime;

/// Log level for structured output.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Success,
}

impl LogLevel {
    pub fn prefix(&self) -> &'static str {
        match self {
            LogLevel::Debug => "🔍",
            LogLevel::Info => "ℹ",
            LogLevel::Warn => "⚠",
            LogLevel::Error => "✖",
            LogLevel::Success => "✓",
        }
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            LogLevel::Debug => "\x1b[38;5;145m", // gray
            LogLevel::Info => "\x1b[38;5;109m",   // muted blue
            LogLevel::Warn => "\x1b[38;5;221m",   // soft amber
            LogLevel::Error => "\x1b[38;5;210m",  // soft red
            LogLevel::Success => "\x1b[38;5;150m", // soft green
        }
    }

    pub fn reset() -> &'static str {
        "\x1b[0m"
    }
}

/// Print a structured, eye-friendly log line.
pub fn log(level: LogLevel, target: &str, message: &str) {
    let c = level.color_code();
    let r = LogLevel::reset();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let timestamp = format_time(now);
    println!(
        "\r\n  {}{} {}{} {}{}{} {}{}{}\r\n",
        c, level.prefix(), r,
        c, timestamp, r,
        c, target, r,
        message
    );
    let _ = io::stdout().flush();
}

/// Convenience: info log.
pub fn info(target: &str, message: &str) {
    log(LogLevel::Info, target, message);
}

/// Convenience: warn log.
pub fn warn(target: &str, message: &str) {
    log(LogLevel::Warn, target, message);
}

/// Convenience: error log.
pub fn error(target: &str, message: &str) {
    log(LogLevel::Error, target, message);
}

/// Convenience: success log.
pub fn success(target: &str, message: &str) {
    log(LogLevel::Success, target, message);
}

/// Convenience: debug log.
pub fn debug(target: &str, message: &str) {
    log(LogLevel::Debug, target, message);
}

fn format_time(secs: u64) -> String {
    let h = (secs / 3600) % 24;
    let m = (secs / 60) % 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}
