use crate::editor::Editor;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub fn render_status_bar(f: &mut Frame, editor: &Editor, area: Rect) {
    let buffer = editor.buffer();
    let cursor = editor.cursor();

    let modified = if buffer.is_modified() { " [+]" } else { "" };
    let status = format!(
        " {} {}  Ln {}, Col {}",
        buffer.file_name(),
        modified,
        cursor.line + 1,
        cursor.column + 1
    );

    let line_count = format!(" {} lines ", buffer.len_lines());

    let status_width = status.width();
    let line_count_width = line_count.width();
    let padding_width = area
        .width
        .saturating_sub(status_width as u16)
        .saturating_sub(line_count_width as u16);

    let padding = " ".repeat(padding_width as usize);

    let status_line = Line::from(vec![
        Span::raw(status),
        Span::raw(padding),
        Span::raw(line_count),
    ]);

    let paragraph =
        Paragraph::new(status_line).style(Style::default().bg(Color::DarkGray).fg(Color::White));

    f.render_widget(paragraph, area);
}
