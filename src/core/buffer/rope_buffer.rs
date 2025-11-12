use ropey::Rope;

pub struct RopeBuffer {
    rope: Rope,
}

impl RopeBuffer {
    pub fn new() -> Self {
        Self { rope: Rope::new() }
    }

    pub fn from_string(content: &str) -> Self {
        Self {
            rope: Rope::from_str(content),
        }
    }

    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn insert_char(&mut self, idx: usize, ch: char) {
        self.rope.insert_char(idx, ch);
    }

    pub fn insert(&mut self, idx: usize, text: &str) {
        self.rope.insert(idx, text);
    }

    pub fn remove(&mut self, range: std::ops::Range<usize>) {
        if range.start < range.end && range.end <= self.rope.len_chars() {
            self.rope.remove(range);
        }
    }

    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx < self.rope.len_lines() {
            Some(self.rope.line(line_idx).to_string())
        } else {
            None
        }
    }

    pub fn line_len(&self, line_idx: usize) -> usize {
        if line_idx < self.rope.len_lines() {
            let line = self.rope.line(line_idx);
            let line_str = line.to_string();
            line_str.trim_end_matches(&['\n', '\r'][..]).len()
        } else {
            0
        }
    }

    pub fn char_to_line(&self, char_idx: usize) -> usize {
        self.rope.char_to_line(char_idx)
    }

    pub fn line_to_char(&self, line_idx: usize) -> usize {
        self.rope.line_to_char(line_idx)
    }

    pub fn get_slice(&self, start: usize, end: usize) -> String {
        if start < end && end <= self.rope.len_chars() {
            self.rope.slice(start..end).to_string()
        } else {
            String::new()
        }
    }

    pub fn chunks(&self) -> impl Iterator<Item = &str> {
        self.rope.chunks()
    }
}

impl Default for RopeBuffer {
    fn default() -> Self {
        Self::new()
    }
}