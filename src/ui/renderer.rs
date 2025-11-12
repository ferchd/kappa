use crate::state::{EditorMode, EditorState};
use crate::ui::components::{render_editor, render_message_bar, render_status_bar};
use crate::ui::components::dialogs::{render_command_palette, render_input_dialog};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;

pub fn render(f: &mut Frame, state: &EditorState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(f.area());

    render_editor(f, state, chunks[0]);
    render_status_bar(f, state, chunks[1]);
    render_message_bar(f, state, chunks[2]);

    match state.mode() {
        EditorMode::CommandPalette => render_command_palette(f, state),
        EditorMode::OpenFile | EditorMode::SaveAs => render_input_dialog(f, state),
        _ => {}
    }
}