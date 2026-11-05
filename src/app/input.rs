use crate::app::ui::{shorten_path, strip_ansi, visible_width};
use crate::app::window::TerminalWindow;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    queue,
    style::Print,
    terminal::{self, Clear, ClearType},
};
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

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

impl TerminalWindow {
    pub fn read_line(&self, history: &[String]) -> io::Result<Option<String>> {
        let c = &self.config.colors;
        let mut stdout = io::stdout();
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let short = shorten_path(&cwd, 2);
        let prompt_text = format!(
            "  {}{}{} {}{}❯{} ",
            c.dim,
            short,
            c.reset,
            c.bold,
            c.accent,
            c.reset
        );
        let prompt_visible = visible_width(&strip_ansi(&prompt_text));
        write!(stdout, "{}", prompt_text)?;
        stdout.flush()?;

        let mut buffer: Vec<char> = Vec::new();
        let mut cursor: usize = 0;
        let mut history_idx: Option<usize> = None;
        let mut saved_current: Vec<char> = Vec::new();

        let _guard = RawModeGuard::new()?;
        let result = (|| -> io::Result<Option<String>> {
            loop {
                if let Event::Key(KeyEvent { code, modifiers, kind, .. }) = event::read()? {
                    if kind != KeyEventKind::Press {
                        continue;
                    }

                    match (code, modifiers) {
                        (KeyCode::Enter, _) => {
                            write!(stdout, "\r\n")?;
                            stdout.flush()?;
                            break;
                        }
                        (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                            write!(stdout, "\r\n")?;
                            stdout.flush()?;
                            return Ok(None);
                        }
                        (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                            if buffer.is_empty() {
                                write!(stdout, "\r\n")?;
                                stdout.flush()?;
                                return Ok(None);
                            }
                        }
                        (KeyCode::Char(ch), _) => {
                            buffer.insert(cursor, ch);
                            cursor += 1;
                        }
                        (KeyCode::Backspace, _) => {
                            if cursor > 0 {
                                buffer.remove(cursor - 1);
                                cursor -= 1;
                            }
                        }
                        (KeyCode::Delete, _) => {
                            if cursor < buffer.len() {
                                buffer.remove(cursor);
                            }
                        }
                        (KeyCode::Left, _) => {
                            if cursor > 0 {
                                cursor -= 1;
                            }
                        }
                        (KeyCode::Right, _) => {
                            if cursor < buffer.len() {
                                cursor += 1;
                            }
                        }
                        (KeyCode::Home, _) => cursor = 0,
                        (KeyCode::End, _) => cursor = buffer.len(),
                        (KeyCode::Up, _) => {
                            if !history.is_empty() {
                                let next_idx = match history_idx {
                                    None => {
                                        saved_current = buffer.clone();
                                        Some(history.len().saturating_sub(1))
                                    }
                                    Some(0) => Some(0),
                                    Some(i) => Some(i - 1),
                                };
                                if let Some(i) = next_idx {
                                    history_idx = Some(i);
                                    buffer = history[i].chars().collect();
                                    cursor = buffer.len();
                                }
                            }
                        }
                        (KeyCode::Down, _) => {
                            if let Some(i) = history_idx {
                                if i + 1 < history.len() {
                                    history_idx = Some(i + 1);
                                    buffer = history[i + 1].chars().collect();
                                    cursor = buffer.len();
                                } else {
                                    history_idx = None;
                                    buffer = saved_current.clone();
                                    cursor = buffer.len();
                                }
                            }
                        }
                        _ => {}
                    }

                    let current = buffer.iter().collect::<String>();
                    let cur_width = visible_width(&current[..current
                        .char_indices()
                        .nth(cursor)
                        .map(|(i, _)| i)
                        .unwrap_or(current.len())]);
                    queue!(
                        stdout,
                        cursor::MoveToColumn(0),
                        Clear(ClearType::CurrentLine),
                        Print(&prompt_text),
                        Print(&current),
                        cursor::MoveToColumn((prompt_visible + cur_width) as u16)
                    )?;
                    stdout.flush()?;
                }
            }
            Ok(Some(buffer.iter().collect()))
        })();

        result
    }
}
