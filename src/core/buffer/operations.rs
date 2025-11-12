use super::RopeBuffer;

pub struct TextOperation;

impl TextOperation {
    pub fn insert_char(buffer: &mut RopeBuffer, idx: usize, ch: char) {
        buffer.insert_char(idx, ch);
    }

    pub fn insert_text(buffer: &mut RopeBuffer, idx: usize, text: &str) {
        buffer.insert(idx, text);
    }

    pub fn delete_range(buffer: &mut RopeBuffer, start: usize, end: usize) {
        buffer.remove(start..end);
    }

    pub fn delete_char(buffer: &mut RopeBuffer, idx: usize) {
        if idx < buffer.len_chars() {
            buffer.remove(idx..idx + 1);
        }
    }
}