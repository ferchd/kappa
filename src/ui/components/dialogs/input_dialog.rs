use crate::state::EditorState;
use crate::ui::theme::Theme;
use ratatui::layout::Position;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph},
};

pub fn render_input_dialog(f: &mut Frame, state: &EditorState) {
    let theme = Theme::default();
    let area = f.area();
    let popup_width = area.width.min(60);
    let popup_height = 3;

    let popup_area = Rect {
        x: (area.width - popup_width) / 2,
        y: (area.height - popup_height) / 2,
        width: popup_width,
        height: popup_height,
    };

    f.render_widget(Clear, popup_area);

    let title = state.mode().dialog_title();

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().bg(theme.dialog_bg));

    let inner_area = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let input = Paragraph::new(state.command_input()).style(Style::default().fg(theme.dialog_fg));
    f.render_widget(input, inner_area);

    let cursor_x = popup_area.x + 1 + state.command_input().len() as u16;
    let cursor_y = popup_area.y + 1;
    f.set_cursor_position(Position::new(cursor_x, cursor_y));
}
