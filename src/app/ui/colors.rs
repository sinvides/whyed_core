/// Dark, eye-friendly color palette (ANSI 256).
/// Avoids neon/overexposed colors; uses muted, desaturated tones.
#[derive(Clone, Copy)]
pub struct Colors {
    pub reset: &'static str,
    pub bold: &'static str,
    pub dim: &'static str,
    /// Background surface (dark gray)
    pub surface: &'static str,
    /// Primary text (muted off-white)
    pub text: &'static str,
    /// Muted gray for secondary info
    pub muted: &'static str,
    /// Accent color (muted blue)
    pub accent: &'static str,
    /// Secondary accent (muted purple)
    pub secondary: &'static str,
    /// Success (soft green)
    pub success: &'static str,
    /// Warning (soft amber)
    pub warning: &'static str,
    /// Error (soft red)
    pub error: &'static str,
    /// Highlight (soft teal)
    pub highlight: &'static str,
}

impl Colors {
    pub fn dark() -> Self {
        Colors {
            reset: "\x1b[0m",
            bold: "\x1b[1m",
            dim: "\x1b[2m",
            surface: "\x1b[38;5;236m",   // dark gray background feel
            text: "\x1b[38;5;252m",      // muted off-white
            muted: "\x1b[38;5;145m",     // gray
            accent: "\x1b[38;5;109m",    // muted blue
            secondary: "\x1b[38;5;147m", // muted purple
            success: "\x1b[38;5;150m",   // soft green
            warning: "\x1b[38;5;221m",   // soft amber
            error: "\x1b[38;5;210m",     // soft red
            highlight: "\x1b[38;5;87m",  // soft teal
        }
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self::dark()
    }
}

/// Visible space character (U+2003 EM SPACE) — wider than ASCII, clearly visible.
pub const VISIBLE_SPACE: &str = "\u{2003}";

/// Box-drawing characters for borders.
pub mod box_chars {
    pub const HORIZONTAL: &str = "─";
    pub const VERTICAL: &str = "│";
    pub const TOP_LEFT: &str = "╭";
    pub const TOP_RIGHT: &str = "╮";
    pub const BOTTOM_LEFT: &str = "╰";
    pub const BOTTOM_RIGHT: &str = "╯";
}
