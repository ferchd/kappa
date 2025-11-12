use ratatui::style::Color;

pub struct Theme {
    pub line_number: Color,
    pub status_bar_bg: Color,
    pub status_bar_fg: Color,
    pub message_bar: Color,
    pub dialog_bg: Color,
    pub dialog_fg: Color,
    pub dialog_highlight: Color,
}

impl Theme {
    pub fn default_theme() -> Self {
        Self {
            line_number: Color::DarkGray,
            status_bar_bg: Color::DarkGray,
            status_bar_fg: Color::White,
            message_bar: Color::Yellow,
            dialog_bg: Color::Black,
            dialog_fg: Color::White,
            dialog_highlight: Color::Yellow,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_theme()
    }
}