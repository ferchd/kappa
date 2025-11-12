use crate::state::EditorState;
use crate::ui::theme::Theme;
use ratatui::{layout::Rect, style::Style, widgets::Paragraph, Frame};

pub fn render_message_bar(f: &mut Frame, state: &EditorState, area: Rect) {
    let theme = Theme::default();
    let message = state.message().get().unwrap_or("");
    let paragraph = Paragraph::new(message).style(Style::default().fg(theme.message_bar));
    f.render_widget(paragraph, area);
}