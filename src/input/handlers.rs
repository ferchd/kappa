use crate::editor::{Editor, EditorMode};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

#[derive(Debug)]
pub enum Action {
    Quit,
    Save,
    Continue,
}

pub fn handle_input(event: Event, editor: &mut Editor) -> Result<Option<Action>, std::io::Error> {
    match event {
        Event::Key(key) => {
            if key.kind != KeyEventKind::Press {
                return Ok(Some(Action::Continue));
            }

            match editor.mode() {
                EditorMode::Normal => handle_normal_mode(key, editor),
                EditorMode::CommandPalette | EditorMode::OpenFile | EditorMode::SaveAs => {
                    handle_dialog_mode(key, editor)
                }
            }
        }
        _ => Ok(Some(Action::Continue)),
    }
}

fn handle_normal_mode(
    key: crossterm::event::KeyEvent,
    editor: &mut Editor,
) -> Result<Option<Action>, std::io::Error> {
    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if editor.has_unsaved_changes() {
                editor.set_message(
                    "Unsaved changes! Press Ctrl+Q again to quit without saving, or Ctrl+S to save",
                );
                if let Event::Key(confirm_key) = crossterm::event::read()? {
                    if confirm_key.kind == KeyEventKind::Press {
                        match confirm_key.code {
                            KeyCode::Char('q')
                                if confirm_key.modifiers.contains(KeyModifiers::CONTROL) =>
                            {
                                return Ok(Some(Action::Quit));
                            }
                            KeyCode::Char('s')
                                if confirm_key.modifiers.contains(KeyModifiers::CONTROL) =>
                            {
                                editor.save()?;
                                return Ok(Some(Action::Quit));
                            }
                            _ => {
                                editor.set_message("Quit canceled");
                            }
                        }
                    }
                }
            } else {
                return Ok(Some(Action::Quit));
            }
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            editor.save()?;
            return Ok(Some(Action::Save));
        }
        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            editor.open_command_palette();
        }
        KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            editor.open_file_dialog();
        }
        KeyCode::Char(c) => {
            editor.insert_char(c);
        }
        KeyCode::Enter => {
            editor.insert_newline();
        }
        KeyCode::Backspace => {
            editor.backspace();
        }
        KeyCode::Delete => {
            editor.delete();
        }
        KeyCode::Left => {
            editor.move_cursor_left();
        }
        KeyCode::Right => {
            editor.move_cursor_right();
        }
        KeyCode::Up => {
            editor.move_cursor_up();
        }
        KeyCode::Down => {
            editor.move_cursor_down();
        }
        KeyCode::Home => {
            editor.move_cursor_line_start();
        }
        KeyCode::End => {
            editor.move_cursor_line_end();
        }
        KeyCode::PageUp => {
            editor.page_up();
        }
        KeyCode::PageDown => {
            editor.page_down();
        }
        KeyCode::Tab => {
            editor.insert_tab();
        }
        _ => {}
    }

    Ok(Some(Action::Continue))
}

fn handle_dialog_mode(
    key: crossterm::event::KeyEvent,
    editor: &mut Editor,
) -> Result<Option<Action>, std::io::Error> {
    match key.code {
        KeyCode::Char(c) => {
            editor.handle_command_input(c);
        }
        KeyCode::Backspace => {
            editor.backspace_command_input();
        }
        KeyCode::Enter => {
            editor.execute_command_input()?;
        }
        KeyCode::Esc => {
            editor.cancel_dialog();
        }
        _ => {}
    }

    Ok(Some(Action::Continue))
}
