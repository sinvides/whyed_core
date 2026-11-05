use crate::app::config::dirs_home;
use crossterm::terminal;
use std::ffi::OsStr;
use std::path::Path;

pub(crate) fn visible_width(text: &str) -> usize {
    let mut width = 0;
    for ch in text.chars() {
        if is_combining(ch) {
            continue;
        }
        width += if is_wide(ch) { 2 } else { 1 };
    }
    width
}

fn is_combining(ch: char) -> bool {
    matches!(ch as u32,
        0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF |
        0x20D0..=0x20FF | 0xFE20..=0xFE2F)
}

fn is_wide(ch: char) -> bool {
    matches!(ch as u32,
        0x1100..=0x115F | 0x2E80..=0xA4CF | 0xAC00..=0xD7A3 |
        0xF900..=0xFAFF | 0xFE10..=0xFE19 | 0xFE30..=0xFE6F |
        0xFF00..=0xFF60 | 0xFFE0..=0xFFE6 | 0x1F300..=0x1F64F |
        0x1F900..=0x1F9FF | 0x20000..=0x3FFFD)
}

pub(crate) fn strip_ansi(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    while let Some(&c) = chars.peek() {
                        chars.next();
                        if c.is_ascii_alphabetic() || c == '@' || c == '`' || c == '~' {
                            break;
                        }
                    }
                }
                Some(']') => {
                    chars.next();
                    while let Some(&c) = chars.peek() {
                        chars.next();
                        if c == '\x07' {
                            break;
                        }
                        if c == '\x1b' {
                            if chars.peek() == Some(&'\\') {
                                chars.next();
                            }
                            break;
                        }
                    }
                }
                Some('P') | Some('^') | Some('_') | Some('X') => {
                    chars.next();
                    while let Some(&c) = chars.peek() {
                        chars.next();
                        if c == '\x1b' {
                            if chars.peek() == Some(&'\\') {
                                chars.next();
                            }
                            break;
                        }
                    }
                }
                Some(_) => {
                    chars.next();
                }
                None => {}
            }
        } else {
            result.push(ch);
        }
    }
    result
}

pub(crate) fn pad_line(content: &str, total_width: usize) -> String {
    let visible = visible_width(&strip_ansi(content));
    let mut result = content.to_string();
    if visible < total_width {
        result.push_str(&" ".repeat(total_width - visible));
    }
    result
}

pub(crate) fn shorten_path(path: &Path, max_parts: usize) -> String {
    if let Some(home) = dirs_home() {
        if let Ok(rel) = path.strip_prefix(&home) {
            return format!("~/{}", rel.to_string_lossy().replace('\\', "/"));
        }
    }
    let parts: Vec<&OsStr> = path.iter().collect();
    if parts.len() <= max_parts {
        return path.to_string_lossy().replace('\\', "/");
    }
    let tail: Vec<String> = parts[parts.len() - max_parts..]
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    format!(".../{}", tail.join("/"))
}

pub(crate) fn term_width() -> usize {
    terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(80)
}
