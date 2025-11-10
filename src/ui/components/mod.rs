mod dialogs;
pub mod editor_view;
pub mod message_bar;
pub mod status_bar;

use crate::{Editor, EditorMode};
pub use editor_view::render_editor;
pub use message_bar::render_message_bar;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;
pub use status_bar::render_status_bar;
use crate::ui::components::dialogs::{render_command_palette, render_input_dialog};

pub fn render(f: &mut Frame, editor: &Editor) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(f.area());

    render_editor(f, editor, chunks[0]);
    render_status_bar(f, editor, chunks[1]);
    render_message_bar(f, editor, chunks[2]);

    match editor.mode() {
        EditorMode::CommandPalette => render_command_palette(f, editor),
        EditorMode::OpenFile | EditorMode::SaveAs => render_input_dialog(f, editor),
        _ => {}
    }
}
