use crate::state::EditorState;
use crate::ui::theme::Theme;
use ratatui::layout::Position;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn render_command_palette(f: &mut Frame, state: &EditorState) {
    let theme = Theme::default();
    let area = f.area();
    let popup_width = area.width.min(60);
    let popup_height = 12;

    let popup_area = Rect {
        x: (area.width - popup_width) / 2,
        y: (area.height - popup_height) / 2,
        width: popup_width,
        height: popup_height,
    };

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title("Command Palette")
        .borders(Borders::ALL)
        .style(Style::default().bg(theme.dialog_bg));

    let inner_area = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner_area);

    let input = Paragraph::new(format!("> {}", state.command_input()))
        .style(Style::default().fg(theme.dialog_fg));
    f.render_widget(input, chunks[0]);

    let commands: Vec<Line> = state
        .filtered_commands()
        .iter()
        .take(8)
        .enumerate()
        .map(|(i, cmd)| {
            let style = if i == 0 {
                Style::default()
                    .fg(theme.dialog_highlight)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.dialog_fg)
            };
            Line::from(Span::styled(format!("  {}", cmd.name), style))
        })
        .collect();

    let list = Paragraph::new(commands);
    f.render_widget(list, chunks[1]);

    let cursor_x = popup_area.x + 3 + state.command_input().len() as u16;
    let cursor_y = popup_area.y + 1;
    f.set_cursor_position(Position::new(cursor_x, cursor_y));
}