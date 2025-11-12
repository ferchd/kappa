use crate::core::document::Document;
use crate::features::command_palette::CommandFilter;
use crate::features::file_operations::{SaveFile};
use crate::input::InputAction;
use crate::state::{EditorMode, EditorState};
use crossterm::event::{KeyCode, KeyEvent};
use std::io;

pub fn handle_command_palette(
    key: KeyEvent,
    state: &mut EditorState,
) -> io::Result<Option<InputAction>> {
    match key.code {
        KeyCode::Char(c) => {
            state.push_command_input(c);
            update_filtered_commands(state);
        }
        KeyCode::Backspace => {
            state.pop_command_input();
            update_filtered_commands(state);
        }
        KeyCode::Enter => {
            if !state.filtered_commands().is_empty() {
                let command_id = state.filtered_commands()[0].id.clone();
                execute_command(state, &command_id)?;
            }
            state.set_mode(EditorMode::Normal);
            state.clear_command_input();
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

fn update_filtered_commands(state: &mut EditorState) {
    let all_commands = state.command_registry().all_commands();
    let filtered = CommandFilter::filter_commands(all_commands, state.command_input());
    state.set_filtered_commands(filtered);
}

fn execute_command(state: &mut EditorState, command_id: &str) -> io::Result<()> {
    match command_id {
        "open_file" => {
            state.set_mode(EditorMode::OpenFile);
            state.clear_command_input();
            state
                .message_mut()
                .set("Enter file path to open:".to_string());
        }
        "save" => {
            let has_file_path = state.document().file_path().is_some();
            if !has_file_path {
                state.set_mode(EditorMode::SaveAs);
                state.clear_command_input();
                state
                    .message_mut()
                    .set("Enter file path to save as:".to_string());
            } else {
                let file_name = state.document().file_name();
                SaveFile::execute(state.document_mut())?;
                state
                    .message_mut()
                    .set(format!("Saved: {}", file_name));
            }
        }
        "save_as" => {
            state.set_mode(EditorMode::SaveAs);
            state.clear_command_input();
            state
                .message_mut()
                .set("Enter file path to save as:".to_string());
        }
        "new_file" => {
            let new_document = Document::new();
            state.replace_document(new_document);
            state.message_mut().set("New file created".to_string());
        }
        "close_file" => {
            let new_document = Document::new();
            state.replace_document(new_document);
        }
        _ => {}
    }
    Ok(())
}