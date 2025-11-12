use crate::core::cursor::CursorMovement;
use crate::features::editing::{DeleteFeature, InsertFeature};
use crate::features::file_operations::SaveFile;
use crate::input::InputAction;
use crate::state::{EditorMode, EditorState};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io;

pub fn handle_normal_mode(
    key: KeyEvent,
    state: &mut EditorState,
) -> io::Result<Option<InputAction>> {
    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if state.document().is_modified() {
                state.message_mut().set(
                    "Unsaved changes! Press Ctrl+Q again to quit without saving, or Ctrl+S to save"
                        .to_string(),
                );
                if let Event::Key(confirm_key) = crossterm::event::read()? {
                    if confirm_key.kind == KeyEventKind::Press {
                        match confirm_key.code {
                            KeyCode::Char('q')
                            if confirm_key.modifiers.contains(KeyModifiers::CONTROL) =>
                                {
                                    return Ok(Some(InputAction::Quit));
                                }
                            KeyCode::Char('s')
                            if confirm_key.modifiers.contains(KeyModifiers::CONTROL) =>
                                {
                                    if let Err(e) = SaveFile::execute(state.document_mut()) {
                                        state
                                            .message_mut()
                                            .set(format!("Save failed: {}", e));
                                    } else {
                                        return Ok(Some(InputAction::Quit));
                                    }
                                }
                            _ => {
                                state.message_mut().set("Quit canceled".to_string());
                            }
                        }
                    }
                }
            } else {
                return Ok(Some(InputAction::Quit));
            }
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            let has_file_path = state.document().file_path().is_some();
            if !has_file_path {
                state.set_mode(EditorMode::SaveAs);
                state.clear_command_input();
                state
                    .message_mut()
                    .set("Enter file path to save as:".to_string());
            } else {
                let file_name = state.document().file_name();
                if let Err(e) = SaveFile::execute(state.document_mut()) {
                    state.message_mut().set(format!("Save failed: {}", e));
                } else {
                    state
                        .message_mut()
                        .set(format!("Saved: {}", file_name));
                    return Ok(Some(InputAction::Save));
                }
            }
        }
        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            state.set_mode(EditorMode::CommandPalette);
            state.clear_command_input();
            let all_commands = state.command_registry().all_commands().to_vec();
            state.set_filtered_commands(all_commands);
        }
        KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            state.set_mode(EditorMode::OpenFile);
            state.clear_command_input();
            state
                .message_mut()
                .set("Enter file path to open:".to_string());
        }
        KeyCode::Char(c) => {
            let (buffer, cursor, viewport) = state.buffer_cursor_and_viewport_mut();
            InsertFeature::insert_char(buffer, cursor, c);
            viewport.adjust_for_cursor(cursor.line);
            state.message_mut().clear();
        }
        KeyCode::Enter => {
            let (buffer, cursor, viewport) = state.buffer_cursor_and_viewport_mut();
            InsertFeature::insert_newline(buffer, cursor);
            viewport.adjust_for_cursor(cursor.line);
            state.message_mut().clear();
        }
        KeyCode::Backspace => {
            let (buffer, cursor, viewport) = state.buffer_cursor_and_viewport_mut();
            DeleteFeature::backspace(buffer, cursor);
            viewport.adjust_for_cursor(cursor.line);
            state.message_mut().clear();
        }
        KeyCode::Delete => {
            let (buffer, cursor) = state.buffer_and_cursor_mut();
            DeleteFeature::delete(buffer, cursor);
            state.message_mut().clear();
        }
        KeyCode::Left => {
            let (buffer, cursor, viewport) = state.buffer_cursor_viewport();
            CursorMovement::move_left(cursor, buffer);
            viewport.adjust_for_cursor(cursor.line);
        }
        KeyCode::Right => {
            let (buffer, cursor, viewport) = state.buffer_cursor_viewport();
            CursorMovement::move_right(cursor, buffer);
            viewport.adjust_for_cursor(cursor.line);
        }
        KeyCode::Up => {
            let (buffer, cursor, viewport) = state.buffer_cursor_viewport();
            CursorMovement::move_up(cursor, buffer);
            viewport.adjust_for_cursor(cursor.line);
        }
        KeyCode::Down => {
            let (buffer, cursor, viewport) = state.buffer_cursor_viewport();
            CursorMovement::move_down(cursor, buffer);
            viewport.adjust_for_cursor(cursor.line);
        }
        KeyCode::Home => {
            CursorMovement::move_line_start(state.cursor_mut());
        }
        KeyCode::End => {
            let (buffer, cursor) = state.buffer_cursor();
            CursorMovement::move_line_end(cursor, buffer);
        }
        KeyCode::PageUp => {
            let (buffer, cursor, viewport) = state.buffer_cursor_viewport();
            let viewport_height = viewport.height();
            CursorMovement::page_up(cursor, buffer, viewport_height);
            viewport.adjust_for_cursor(cursor.line);
        }
        KeyCode::PageDown => {
            let (buffer, cursor, viewport) = state.buffer_cursor_viewport();
            let viewport_height = viewport.height();
            CursorMovement::page_down(cursor, buffer, viewport_height);
            viewport.adjust_for_cursor(cursor.line);
        }
        KeyCode::Tab => {
            let (buffer, cursor) = state.buffer_and_cursor_mut();
            InsertFeature::insert_tab(buffer, cursor);
            state.message_mut().clear();
        }
        _ => {}
    }

    Ok(Some(InputAction::Continue))
}