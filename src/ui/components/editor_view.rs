use crate::state::EditorState;
use crate::ui::components::line_numbers::LineNumbers;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Position, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_editor(f: &mut Frame, state: &EditorState, area: Rect) {
    let theme = Theme::default();
    let buffer = state.document().buffer();
    let scroll = state.viewport().scroll_offset();
    let cursor = state.cursor();

    let viewport_height = area.height as usize;
    let mut lines = Vec::new();
    let end_line = (scroll + viewport_height).min(buffer.len_lines());

    for line_idx in scroll..end_line {
        if let Some(line_content) = buffer.line(line_idx) {
            let line_num = LineNumbers::format(line_idx);
            let line_text = line_content.trim_end_matches(&['\n', '\r'][..]).to_string();

            let line_num_span = Span::styled(line_num, Style::default().fg(theme.line_number));
            let text_span = Span::raw(line_text);
            lines.push(Line::from(vec![line_num_span, text_span]));
        }
    }

    let paragraph = Paragraph::new(lines).block(Block::default().borders(Borders::NONE));

    f.render_widget(paragraph, area);

    if cursor.line >= scroll && cursor.line < scroll + viewport_height {
        let cursor_y = area.y + (cursor.line - scroll) as u16;
        let line_num_width = 5;
        let cursor_x = area.x + line_num_width + cursor.column as u16;

        if cursor_x < area.x + area.width && cursor_y < area.y + area.height {
            f.set_cursor_position(Position::new(cursor_x, cursor_y));
        }
    }
}