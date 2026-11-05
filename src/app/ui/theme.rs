use crate::app::ui::colors::Colors;

/// Rendered theme context passed to UI components.
pub struct Theme {
    pub colors: Colors,
}

impl Theme {
    pub fn dark() -> Self {
        Theme {
            colors: Colors::dark(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
