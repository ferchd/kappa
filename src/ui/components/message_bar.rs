use crate::editor::Editor;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

pub fn render_message_bar(f: &mut Frame, editor: &Editor, area: Rect) {
    let message = editor.get_message().unwrap_or("");
    let paragraph = Paragraph::new(message).style(Style::default().fg(Color::Yellow));
    f.render_widget(paragraph, area);
}
