use crate::state::EditorState;
use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use unicode_width::UnicodeWidthStr;

pub fn render_status_bar(f: &mut Frame, state: &EditorState, area: Rect) {
    let theme = Theme::default();
    let document = state.document();
    let cursor = state.cursor();

    let modified = if document.is_modified() { " [+]" } else { "" };
    let status = format!(
        " {} {}  Ln {}, Col {}",
        document.file_name(),
        modified,
        cursor.line + 1,
        cursor.column + 1
    );

    let line_count = format!(" {} lines ", document.buffer().len_lines());

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

    let paragraph = Paragraph::new(status_line).style(
        Style::default()
            .bg(theme.status_bar_bg)
            .fg(theme.status_bar_fg),
    );

    f.render_widget(paragraph, area);
}