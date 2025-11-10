mod cursor;
mod mode;

pub use cursor::Cursor;
pub use mode::EditorMode;

use crate::buffer::Buffer;
use std::io;

pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,
    scroll_offset: usize,
    viewport_height: usize,
    message: Option<String>,
    mode: EditorMode,
    command_input: String,
    filtered_commands: Vec<String>,
}

impl Editor {
    pub fn new(filename: Option<String>) -> io::Result<Self> {
        let buffer = if let Some(path) = filename {
            Buffer::from_file(&path)?
        } else {
            Buffer::new()
        };

        Ok(Self {
            buffer,
            cursor: Cursor::new(),
            scroll_offset: 0,
            viewport_height: 20,
            message: Some("Ctrl+P: Command Palette | Ctrl+S: Save | Ctrl+Q: Quit".to_string()),
            mode: EditorMode::Normal,
            command_input: String::new(),
            filtered_commands: Vec::new(),
        })
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }
    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }
    pub fn mode(&self) -> &EditorMode {
        &self.mode
    }
    pub fn command_input(&self) -> &str {
        &self.command_input
    }
    pub fn filtered_commands(&self) -> &[String] {
        &self.filtered_commands
    }
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_deref()
    }
    pub fn has_unsaved_changes(&self) -> bool {
        self.buffer.is_modified()
    }

    pub fn set_viewport_height(&mut self, height: usize) {
        self.viewport_height = height;
    }
    pub fn set_message(&mut self, msg: &str) {
        self.message = Some(msg.to_string());
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.mode != EditorMode::Normal {
            return;
        }

        let idx = self.cursor_to_char_idx();
        self.buffer.insert_char(idx, ch);
        self.cursor.column += 1;
        self.clear_message_if_needed();
    }

    pub fn insert_newline(&mut self) {
        if self.mode != EditorMode::Normal {
            return;
        }

        let idx = self.cursor_to_char_idx();
        self.buffer.insert_char(idx, '\n');
        self.cursor.line += 1;
        self.cursor.column = 0;
        self.adjust_scroll();
        self.clear_message_if_needed();
    }

    pub fn insert_tab(&mut self) {
        if self.mode != EditorMode::Normal {
            return;
        }

        let idx = self.cursor_to_char_idx();
        self.buffer.insert(idx, "    ");
        self.cursor.column += 4;
        self.clear_message_if_needed();
    }

    pub fn backspace(&mut self) {
        if self.mode != EditorMode::Normal {
            return;
        }

        if self.cursor.column > 0 {
            let idx = self.cursor_to_char_idx();
            self.buffer.remove(idx - 1..idx);
            self.cursor.column -= 1;
        } else if self.cursor.line > 0 {
            let prev_line_len = self.buffer.line_len(self.cursor.line - 1);
            let idx = self.cursor_to_char_idx();
            self.buffer.remove(idx - 1..idx);
            self.cursor.line -= 1;
            self.cursor.column = prev_line_len;
            self.adjust_scroll();
        }
        self.clear_message_if_needed();
    }

    pub fn delete(&mut self) {
        if self.mode != EditorMode::Normal {
            return;
        }

        let idx = self.cursor_to_char_idx();
        if idx < self.buffer.len_chars() {
            self.buffer.remove(idx..idx + 1);
        }
        self.clear_message_if_needed();
    }

    pub fn move_cursor_left(&mut self) {
        if self.mode != EditorMode::Normal {
            return;
        }

        if self.cursor.column > 0 {
            self.cursor.column -= 1;
        } else if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.column = self.buffer.line_len(self.cursor.line);
            self.adjust_scroll();
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.mode != EditorMode::Normal {
            return;
        }

        let line_len = self.buffer.line_len(self.cursor.line);
        if self.cursor.column < line_len {
            self.cursor.column += 1;
        } else if self.cursor.line < self.buffer.len_lines() - 1 {
            self.cursor.line += 1;
            self.cursor.column = 0;
            self.adjust_scroll();
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.mode != EditorMode::Normal {
            return;
        }

        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            let line_len = self.buffer.line_len(self.cursor.line);
            if self.cursor.column > line_len {
                self.cursor.column = line_len;
            }
            self.adjust_scroll();
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.mode != EditorMode::Normal {
            return;
        }

        if self.cursor.line < self.buffer.len_lines() - 1 {
            self.cursor.line += 1;
            let line_len = self.buffer.line_len(self.cursor.line);
            if self.cursor.column > line_len {
                self.cursor.column = line_len;
            }
            self.adjust_scroll();
        }
    }

    pub fn move_cursor_line_start(&mut self) {
        if self.mode != EditorMode::Normal {
            return;
        }
        self.cursor.column = 0;
    }

    pub fn move_cursor_line_end(&mut self) {
        if self.mode != EditorMode::Normal {
            return;
        }
        self.cursor.column = self.buffer.line_len(self.cursor.line);
    }

    pub fn page_up(&mut self) {
        if self.mode != EditorMode::Normal {
            return;
        }

        let move_amount = self.viewport_height.saturating_sub(1);
        self.cursor.line = self.cursor.line.saturating_sub(move_amount);
        self.adjust_scroll();
        let line_len = self.buffer.line_len(self.cursor.line);
        if self.cursor.column > line_len {
            self.cursor.column = line_len;
        }
    }

    pub fn page_down(&mut self) {
        if self.mode != EditorMode::Normal {
            return;
        }

        let move_amount = self.viewport_height.saturating_sub(1);
        self.cursor.line = (self.cursor.line + move_amount).min(self.buffer.len_lines() - 1);
        self.adjust_scroll();
        let line_len = self.buffer.line_len(self.cursor.line);
        if self.cursor.column > line_len {
            self.cursor.column = line_len;
        }
    }

    pub fn save(&mut self) -> io::Result<()> {
        if self.buffer.file_path().is_none() {
            self.save_as_dialog();
            return Ok(());
        }

        self.buffer.save()?;
        self.message = Some(format!("Saved: {}", self.buffer.file_name()));
        Ok(())
    }

    pub fn open_command_palette(&mut self) {
        self.mode = EditorMode::CommandPalette;
        self.command_input.clear();
        self.update_filtered_commands();
    }

    pub fn open_file_dialog(&mut self) {
        self.mode = EditorMode::OpenFile;
        self.command_input.clear();
        self.message = Some("Enter file path to open:".to_string());
    }

    pub fn save_as_dialog(&mut self) {
        self.mode = EditorMode::SaveAs;
        self.command_input.clear();
        self.message = Some("Enter file path to save as:".to_string());
    }

    pub fn cancel_dialog(&mut self) {
        self.mode = EditorMode::Normal;
        self.command_input.clear();
        self.clear_message_if_needed();
    }

    pub fn handle_command_input(&mut self, ch: char) {
        self.command_input.push(ch);
        if self.mode == EditorMode::CommandPalette {
            self.update_filtered_commands();
        }
    }

    pub fn backspace_command_input(&mut self) {
        self.command_input.pop();
        if self.mode == EditorMode::CommandPalette {
            self.update_filtered_commands();
        }
    }

    pub fn execute_command_input(&mut self) -> io::Result<()> {
        match self.mode {
            EditorMode::OpenFile => {
                let path = self.command_input.clone();
                if !path.is_empty() {
                    match Buffer::from_file(&path) {
                        Ok(new_buffer) => {
                            self.buffer = new_buffer;
                            self.cursor = Cursor::new();
                            self.scroll_offset = 0;
                            self.message = Some(format!("Opened: {}", path));
                        }
                        Err(e) => {
                            self.message = Some(format!("Error opening file: {}", e));
                        }
                    }
                }
                self.mode = EditorMode::Normal;
                self.command_input.clear();
            }
            EditorMode::SaveAs => {
                let path = self.command_input.clone();
                if !path.is_empty() {
                    self.buffer.set_file_path(&path);
                    match self.buffer.save() {
                        Ok(_) => {
                            self.message = Some(format!("Saved as: {}", path));
                        }
                        Err(e) => {
                            self.message = Some(format!("Error saving file: {}", e));
                        }
                    }
                }
                self.mode = EditorMode::Normal;
                self.command_input.clear();
            }
            EditorMode::CommandPalette => {
                if !self.filtered_commands.is_empty() {
                    self.execute_palette_command(&self.filtered_commands[0].clone())?;
                }
                self.mode = EditorMode::Normal;
                self.command_input.clear();
            }
            _ => {}
        }
        Ok(())
    }

    pub fn clear_message_if_needed(&mut self) {
        if self.message.is_some() {
            self.message = None;
        }
    }

    fn execute_palette_command(&mut self, command: &str) -> io::Result<()> {
        match command {
            "Open File" => {
                self.open_file_dialog();
            }
            "Save" => {
                self.save()?;
            }
            "Save As" => {
                self.save_as_dialog();
            }
            "New File" => {
                self.buffer = Buffer::new();
                self.cursor = Cursor::new();
                self.scroll_offset = 0;
                self.message = Some("New file created".to_string());
            }
            "Close File" => {
                self.buffer = Buffer::new();
                self.cursor = Cursor::new();
                self.scroll_offset = 0;
            }
            _ => {}
        }
        Ok(())
    }

    fn update_filtered_commands(&mut self) {
        let all_commands = vec![
            "Open File".to_string(),
            "Save".to_string(),
            "Save As".to_string(),
            "New File".to_string(),
            "Close File".to_string(),
        ];

        if self.command_input.is_empty() {
            self.filtered_commands = all_commands;
        } else {
            let input_lower = self.command_input.to_lowercase();
            self.filtered_commands = all_commands
                .into_iter()
                .filter(|cmd| cmd.to_lowercase().contains(&input_lower))
                .collect();
        }
    }

    fn cursor_to_char_idx(&self) -> usize {
        let line_start = self.buffer.line_to_char(self.cursor.line);
        line_start + self.cursor.column
    }

    fn adjust_scroll(&mut self) {
        if self.cursor.line < self.scroll_offset {
            self.scroll_offset = self.cursor.line;
        } else if self.cursor.line >= self.scroll_offset + self.viewport_height {
            self.scroll_offset = self.cursor.line - self.viewport_height + 1;
        }
    }
}
