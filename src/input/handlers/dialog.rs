use crate::features::file_operations::{OpenFile, SaveFileAs};
use crate::input::InputAction;
use crate::state::{EditorMode, EditorState};
use crossterm::event::{KeyCode, KeyEvent};
use std::io;

pub fn handle_dialog_mode(
    key: KeyEvent,
    state: &mut EditorState,
) -> io::Result<Option<InputAction>> {
    match key.code {
        KeyCode::Char(c) => {
            state.push_command_input(c);
        }
        KeyCode::Backspace => {
            state.pop_command_input();
        }
        KeyCode::Enter => {
            execute_dialog_action(state)?;
        }
        KeyCode::Esc => {
            state.set_mode(EditorMode::Normal);
            state.clear_command_input();
            state.message_mut().clear();
        }
        _ => {}
    }

    Ok(Some(InputAction::Continue))
}

fn execute_dialog_action(state: &mut EditorState) -> io::Result<()> {
    let input = state.command_input().to_string();

    match state.mode() {
        EditorMode::OpenFile => {
            if !input.is_empty() {
                match OpenFile::execute(&input) {
                    Ok(document) => {
                        state.replace_document(document);
                        state.message_mut().set(format!("Opened: {}", input));
                    }
                    Err(e) => {
                        state
                            .message_mut()
                            .set(format!("Error opening file: {}", e));
                    }
                }
            }
        }
        EditorMode::SaveAs => {
            if !input.is_empty() {
                match SaveFileAs::execute(state.document_mut(), &input) {
                    Ok(_) => {
                        state.message_mut().set(format!("Saved as: {}", input));
                    }
                    Err(e) => {
                        state
                            .message_mut()
                            .set(format!("Error saving file: {}", e));
                    }
                }
            }
        }
        _ => {}
    }

    state.set_mode(EditorMode::Normal);
    state.clear_command_input();
    Ok(())
}