use super::CursorPosition;
use crate::core::buffer::RopeBuffer;

pub struct CursorMovement;

impl CursorMovement {
    pub fn move_left(cursor: &mut CursorPosition, buffer: &RopeBuffer) {
        if cursor.column > 0 {
            cursor.column -= 1;
        } else if cursor.line > 0 {
            cursor.line -= 1;
            cursor.column = buffer.line_len(cursor.line);
        }
    }

    pub fn move_right(cursor: &mut CursorPosition, buffer: &RopeBuffer) {
        let line_len = buffer.line_len(cursor.line);
        if cursor.column < line_len {
            cursor.column += 1;
        } else if cursor.line < buffer.len_lines() - 1 {
            cursor.line += 1;
            cursor.column = 0;
        }
    }

    pub fn move_up(cursor: &mut CursorPosition, buffer: &RopeBuffer) {
        if cursor.line > 0 {
            cursor.line -= 1;
            let line_len = buffer.line_len(cursor.line);
            if cursor.column > line_len {
                cursor.column = line_len;
            }
        }
    }

    pub fn move_down(cursor: &mut CursorPosition, buffer: &RopeBuffer) {
        if cursor.line < buffer.len_lines() - 1 {
            cursor.line += 1;
            let line_len = buffer.line_len(cursor.line);
            if cursor.column > line_len {
                cursor.column = line_len;
            }
        }
    }

    pub fn move_line_start(cursor: &mut CursorPosition) {
        cursor.column = 0;
    }

    pub fn move_line_end(cursor: &mut CursorPosition, buffer: &RopeBuffer) {
        cursor.column = buffer.line_len(cursor.line);
    }

    pub fn page_up(cursor: &mut CursorPosition, buffer: &RopeBuffer, page_size: usize) {
        let move_amount = page_size.saturating_sub(1);
        cursor.line = cursor.line.saturating_sub(move_amount);
        let line_len = buffer.line_len(cursor.line);
        if cursor.column > line_len {
            cursor.column = line_len;
        }
    }

    pub fn page_down(cursor: &mut CursorPosition, buffer: &RopeBuffer, page_size: usize) {
        let move_amount = page_size.saturating_sub(1);
        cursor.line = (cursor.line + move_amount).min(buffer.len_lines() - 1);
        let line_len = buffer.line_len(cursor.line);
        if cursor.column > line_len {
            cursor.column = line_len;
        }
    }
}