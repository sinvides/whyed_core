use crate::app::ui::colors::VISIBLE_SPACE;
use crossterm::terminal;

/// Pad content to a target width using visible (EM) spaces.
pub fn pad_to(content: &str, width: usize) -> String {
    let visible = visible_width(content);
    if visible >= width {
        return content.to_string();
    }
    format!("{}{}", content, VISIBLE_SPACE.repeat(width - visible))
}

/// Estimate visible width of a string (ANSI-aware approximation).
pub fn visible_width(text: &str) -> usize {
    let mut width = 0;
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip ANSI escape sequence
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c.is_ascii_alphabetic() || c == '@' || c == '`' || c == '~' {
                        break;
                    }
                }
            }
            continue;
        }
        if is_combining(ch as u32) {
            continue;
        }
        width += if is_wide(ch as u32) { 2 } else { 1 };
    }
    width
}

fn is_combining(cp: u32) -> bool {
    matches!(cp,
        0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF |
        0x20D0..=0x20FF | 0xFE20..=0xFE2F)
}

fn is_wide(cp: u32) -> bool {
    matches!(cp,
        0x1100..=0x115F | 0x2E80..=0xA4CF | 0xAC00..=0xD7A3 |
        0xF900..=0xFAFF | 0xFE10..=0xFE19 | 0xFE30..=0xFE6F |
        0xFF00..=0xFF60 | 0xFFE0..=0xFFE6 | 0x1F300..=0x1F64F |
        0x1F900..=0x1F9FF | 0x20000..=0x3FFFD)
}

/// Terminal width with fallback.
pub fn term_width() -> usize {
    terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(80)
}
