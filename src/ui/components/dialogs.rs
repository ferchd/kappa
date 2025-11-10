use crate::editor::Editor;
use ratatui::layout::Position;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn render_command_palette(f: &mut Frame, editor: &Editor) {
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
        .style(Style::default().bg(Color::Black));

    let inner_area = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(inner_area);

    let input = Paragraph::new(format!("> {}", editor.command_input()))
        .style(Style::default().fg(Color::White));
    f.render_widget(input, chunks[0]);

    let commands: Vec<Line> = editor
        .filtered_commands()
        .iter()
        .take(8)
        .enumerate()
        .map(|(i, cmd)| {
            let style = if i == 0 {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            Line::from(Span::styled(format!("  {}", cmd), style))
        })
        .collect();

    let list = Paragraph::new(commands);
    f.render_widget(list, chunks[1]);

    let cursor_x = popup_area.x + 3 + editor.command_input().len() as u16;
    let cursor_y = popup_area.y + 1;
    f.set_cursor_position(Position::new(cursor_x, cursor_y));
}

pub fn render_input_dialog(f: &mut Frame, editor: &Editor) {
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

    let title = match editor.mode() {
        crate::editor::EditorMode::OpenFile => "Open File",
        crate::editor::EditorMode::SaveAs => "Save As",
        _ => "Input",
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let inner_area = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let input = Paragraph::new(editor.command_input()).style(Style::default().fg(Color::White));
    f.render_widget(input, inner_area);

    let cursor_x = popup_area.x + 1 + editor.command_input().len() as u16;
    let cursor_y = popup_area.y + 1;
    f.set_cursor_position(Position::new(cursor_x, cursor_y));
}
