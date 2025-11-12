mod command_palette;
mod dialog;
mod normal;

use crate::input::InputAction;
use crate::state::{EditorMode, EditorState};
use crossterm::event::{Event, KeyEventKind};
use std::io;

pub struct InputHandler;

impl InputHandler {
    pub fn handle(event: Event, state: &mut EditorState) -> io::Result<Option<InputAction>> {
        match event {
            Event::Key(key) => {
                if key.kind != KeyEventKind::Press {
                    return Ok(Some(InputAction::Continue));
                }

                match state.mode() {
                    EditorMode::Normal => normal::handle_normal_mode(key, state),
                    EditorMode::CommandPalette => {
                        command_palette::handle_command_palette(key, state)
                    }
                    EditorMode::OpenFile | EditorMode::SaveAs => {
                        dialog::handle_dialog_mode(key, state)
                    }
                }
            }
            _ => Ok(Some(InputAction::Continue)),
        }
    }
}
