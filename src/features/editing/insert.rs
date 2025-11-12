use crate::core::buffer::RopeBuffer;
use crate::core::cursor::CursorPosition;

pub struct InsertFeature;

impl InsertFeature {
    pub fn insert_char(
        buffer: &mut RopeBuffer,
        cursor: &mut CursorPosition,
        ch: char,
    ) {
        let idx = Self::cursor_to_char_idx(buffer, cursor);
        buffer.insert_char(idx, ch);
        cursor.column += 1;
    }

    pub fn insert_newline(
        buffer: &mut RopeBuffer,
        cursor: &mut CursorPosition,
    ) {
        let idx = Self::cursor_to_char_idx(buffer, cursor);
        buffer.insert_char(idx, '\n');
        cursor.line += 1;
        cursor.column = 0;
    }

    pub fn insert_tab(
        buffer: &mut RopeBuffer,
        cursor: &mut CursorPosition,
    ) {
        let idx = Self::cursor_to_char_idx(buffer, cursor);
        buffer.insert(idx, "    ");
        cursor.column += 4;
    }

    fn cursor_to_char_idx(buffer: &RopeBuffer, cursor: &CursorPosition) -> usize {
        let line_start = buffer.line_to_char(cursor.line);
        line_start + cursor.column
    }
}