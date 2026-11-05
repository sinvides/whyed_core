use crate::app::ui::layout;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    queue,
    style::Print,
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

/// Guard to enable/disable raw terminal mode.
struct RawModeGuard;

impl RawModeGuard {
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(RawModeGuard)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}

/// Result of reading a line.
pub enum LineResult {
    /// User submitted a line of text.
    Submitted(String),
    /// User cancelled (Ctrl+C / Ctrl+D on empty).
    Cancelled,
}

/// Line editor with visible-space prompt and @file reference detection.
pub struct LineEditor {
    prompt: String,
    buffer: Vec<char>,
    cursor: usize,
    history: Vec<String>,
    history_idx: Option<usize>,
    saved_current: Vec<char>,
}

impl LineEditor {
    pub fn new(prompt_text: &str, history: &[String]) -> Self {
        Self {
            prompt: prompt_text.to_string(),
            buffer: Vec::new(),
            cursor: 0,
            history: history.to_vec(),
            history_idx: None,
            saved_current: Vec::new(),
        }
    }

    /// Read one line from the user in raw mode.
    pub fn read(&mut self, stdout: &mut io::Stdout) -> io::Result<LineResult> {
        let prompt_visible = layout::visible_width(&strip_ansi(&self.prompt));
        write!(stdout, "{}", self.prompt)?;
        stdout.flush()?;

        let _guard = RawModeGuard::new()?;
        let result = (|| -> io::Result<LineResult> {
            loop {
                if let Event::Key(KeyEvent { code, modifiers, kind, .. }) = event::read()? {
                    if kind != KeyEventKind::Press {
                        continue;
                    }

                    match (code, modifiers) {
                        (KeyCode::Enter, _) => {
                            write!(stdout, "\r\n")?;
                            stdout.flush()?;
                            let text: String = self.buffer.iter().collect();
                            return Ok(LineResult::Submitted(text));
                        }
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                            write!(stdout, "\r\n")?;
                            stdout.flush()?;
                            return Ok(LineResult::Cancelled);
                        }
                        (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                            if self.buffer.is_empty() {
                                write!(stdout, "\r\n")?;
                                stdout.flush()?;
                                return Ok(LineResult::Cancelled);
                            }
                        }
                        (KeyCode::Char(ch), _) => {
                            self.buffer.insert(self.cursor, ch);
                            self.cursor += 1;
                        }
                        (KeyCode::Backspace, _) => {
                            if self.cursor > 0 {
                                self.buffer.remove(self.cursor - 1);
                                self.cursor -= 1;
                            }
                        }
                        (KeyCode::Delete, _) => {
                            if self.cursor < self.buffer.len() {
                                self.buffer.remove(self.cursor);
                            }
                        }
                        (KeyCode::Left, _) => {
                            if self.cursor > 0 {
                                self.cursor -= 1;
                            }
                        }
                        (KeyCode::Right, _) => {
                            if self.cursor < self.buffer.len() {
                                self.cursor += 1;
                            }
                        }
                        (KeyCode::Home, _) => self.cursor = 0,
                        (KeyCode::End, _) => self.cursor = self.buffer.len(),
                        (KeyCode::Up, _) => {
                            if !self.history.is_empty() {
                                let next_idx = match self.history_idx {
                                    None => {
                                        self.saved_current = self.buffer.clone();
                                        Some(self.history.len().saturating_sub(1))
                                    }
                                    Some(0) => Some(0),
                                    Some(i) => Some(i - 1),
                                };
                                if let Some(i) = next_idx {
                                    self.history_idx = Some(i);
                                    self.buffer = self.history[i].chars().collect();
                                    self.cursor = self.buffer.len();
                                }
                            }
                        }
                        (KeyCode::Down, _) => {
                            if let Some(i) = self.history_idx {
                                if i + 1 < self.history.len() {
                                    self.history_idx = Some(i + 1);
                                    self.buffer = self.history[i + 1].chars().collect();
                                    self.cursor = self.buffer.len();
                                } else {
                                    self.history_idx = None;
                                    self.buffer = self.saved_current.clone();
                                    self.cursor = self.buffer.len();
                                }
                            }
                        }
                        _ => {}
                    }

                    self.redraw(stdout, prompt_visible)?;
                }
            }
        })();

        result
    }

    fn redraw(&self, stdout: &mut io::Stdout, prompt_visible: usize) -> io::Result<()> {
        let current: String = self.buffer.iter().collect();
        let cur_pos = current.char_indices().nth(self.cursor).map(|(i, _)| i).unwrap_or(current.len());
        let cur_width = layout::visible_width(&current[..cur_pos]);

        queue!(
            stdout,
            cursor::MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            Print(&self.prompt),
            Print(&current),
            cursor::MoveToColumn((prompt_visible + cur_width) as u16)
        )?;
        stdout.flush()
    }

    pub fn set_prompt(&mut self, text: &str) {
        self.prompt = text.to_string();
    }
}

/// Strip ANSI escape sequences for width calculation.
fn strip_ansi(text: &str) -> String {
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
                        if c == '\x07' { break; }
                        if c == '\x1b' {
                            if chars.peek() == Some(&'\\') { chars.next(); }
                            break;
                        }
                    }
                }
                Some('P') | Some('^') | Some('_') | Some('X') => {
                    chars.next();
                    while let Some(&c) = chars.peek() {
                        chars.next();
                        if c == '\x1b' {
                            if chars.peek() == Some(&'\\') { chars.next(); }
                            break;
                        }
                    }
                }
                Some(_) => { chars.next(); }
                None => {}
            }
        } else {
            result.push(ch);
        }
    }
    result
}
