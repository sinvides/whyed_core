use crate::app::ui::{pad_line, strip_ansi, term_width, visible_width};
use crate::app::window::TerminalWindow;
use crossterm::{cursor, execute, terminal::{self, Clear, ClearType}};
use std::io::{self, Write};

impl TerminalWindow {
    pub fn set_title(&self) -> io::Result<()> {
        execute!(io::stdout(), terminal::SetTitle(&self.title))
    }

    pub fn render_box(&self, lines: &[String]) -> io::Result<()> {
        let c = &self.config.colors;
        let mut max_w: usize = 0;
        for line in lines {
            let w = visible_width(&strip_ansi(line));
            if w > max_w {
                max_w = w;
            }
        }
        let avail = term_width().saturating_sub(6);
        let width = (max_w + 2).max(20).min(avail.max(20));
        let mut stdout = io::stdout();
        let offset = "  ";
        let top = format!("{}╭{}╮{}", c.accent, "─".repeat(width), c.reset);
        let bottom = format!("{}╰{}╯{}", c.accent, "─".repeat(width), c.reset);
        write!(stdout, "{}{}\r\n", offset, top)?;
        for line in lines {
            let padded = pad_line(line, width);
            write!(
                stdout,
                "{}{}│{}{}{}│{}\r\n",
                offset,
                c.accent,
                c.reset,
                padded,
                c.accent,
                c.reset
            )?;
        }
        write!(stdout, "{}{}\r\n", offset, bottom)?;
        stdout.flush()
    }

    pub fn build_content(&self) -> Vec<String> {
        let c = &self.config.colors;
        let name = self.config.app_name();
        let model = self.config.model();
        let version = self.config.version();
        vec![
            format!(
                "  {}✻{}  {}{}Добро пожаловать в {}{}{}",
                c.accent,
                c.reset,
                c.bold,
                c.white,
                name,
                c.reset,
                ""
            ),
            format!(
                "  {}Ваш ИИ напарник по программированию.{}",
                c.gray,
                c.reset
            ),
            String::new(),
            format!(
                "  {}Модель: {} | Версия: {}{}",
                c.gray,
                model,
                version,
                c.reset
            ),
            format!(
                "  {}/status для проверки текущей конфигурации{}",
                c.gray,
                c.reset
            ),
            String::new(),
        ]
    }

    pub fn clear(&self) -> io::Result<()> {
        execute!(
            io::stdout(),
            Clear(ClearType::All),
            Clear(ClearType::Purge),
            cursor::MoveTo(0, 0)
        )
    }
}
