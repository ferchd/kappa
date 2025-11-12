use crate::core::buffer::RopeBuffer;
use crate::core::cursor::CursorPosition;

pub struct DeleteFeature;

impl DeleteFeature {
    pub fn backspace(
        buffer: &mut RopeBuffer,
        cursor: &mut CursorPosition,
    ) {
        if cursor.column > 0 {
            let idx = Self::cursor_to_char_idx(buffer, cursor);
            buffer.remove(idx - 1..idx);
            cursor.column -= 1;
        } else if cursor.line > 0 {
            let prev_line_len = buffer.line_len(cursor.line - 1);
            let idx = Self::cursor_to_char_idx(buffer, cursor);
            buffer.remove(idx - 1..idx);
            cursor.line -= 1;
            cursor.column = prev_line_len;
        }
    }

    pub fn delete(
        buffer: &mut RopeBuffer,
        cursor: &CursorPosition,
    ) {
        let idx = Self::cursor_to_char_idx(buffer, cursor);
        if idx < buffer.len_chars() {
            buffer.remove(idx..idx + 1);
        }
    }

    fn cursor_to_char_idx(buffer: &RopeBuffer, cursor: &CursorPosition) -> usize {
        let line_start = buffer.line_to_char(cursor.line);
        line_start + cursor.column
    }
}