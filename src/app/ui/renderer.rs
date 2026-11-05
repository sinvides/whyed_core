use crate::app::ui::{colors, layout};
use crate::app::ui::theme::Theme;
use crossterm::{cursor, execute, terminal::{self, Clear, ClearType}};
use std::io::{self, Write};

pub struct Renderer {
    pub theme: Theme,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            theme: Theme::dark(),
        }
    }

    pub fn render_box(&self, stdout: &mut io::Stdout, lines: &[String]) -> io::Result<()> {
        let c = &self.theme.colors;
        let mut max_w: usize = 0;
        for line in lines {
            let w = layout::visible_width(line);
            if w > max_w {
                max_w = w;
            }
        }
        let avail = layout::term_width().saturating_sub(6);
        let width = (max_w + 2).max(20).min(avail.max(20));
        let offset = "  ";

        let top = format!(
            "{}{}{}{}{}{}",
            c.accent, colors::box_chars::TOP_LEFT,
            colors::box_chars::HORIZONTAL.repeat(width),
            colors::box_chars::TOP_RIGHT, c.reset,
            "\r\n"
        );
        write!(stdout, "{}{}", offset, top)?;

        for line in lines {
            let padded = layout::pad_to(line, width);
            write!(
                stdout,
                "{}{}{}{}{}{}{}\r\n",
                offset,
                c.accent, colors::box_chars::VERTICAL, c.reset,
                c.dim, padded, c.reset
            )?;
        }

        let bottom = format!(
            "{}{}{}{}{}{}",
            c.accent, colors::box_chars::BOTTOM_LEFT,
            colors::box_chars::HORIZONTAL.repeat(width),
            colors::box_chars::BOTTOM_RIGHT, c.reset,
            "\r\n"
        );
        write!(stdout, "{}{}", offset, bottom)?;
        stdout.flush()
    }

    pub fn clear_screen(stdout: &mut io::Stdout) -> io::Result<()> {
        execute!(
            stdout,
            Clear(ClearType::All),
            Clear(ClearType::Purge),
            cursor::MoveTo(0, 0)
        )
    }

    pub fn set_title(title: &str) -> io::Result<()> {
        execute!(io::stdout(), terminal::SetTitle(title))
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
