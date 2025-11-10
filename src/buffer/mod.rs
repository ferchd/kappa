use ropey::Rope;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub struct Buffer {
    rope: Rope,
    file_path: Option<PathBuf>,
    modified: bool,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            file_path: None,
            modified: false,
        }
    }

    pub fn from_file(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(Self {
            rope: Rope::from_str(&content),
            file_path: Some(PathBuf::from(path)),
            modified: false,
        })
    }

    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }
    pub fn is_modified(&self) -> bool {
        self.modified
    }
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn file_name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("[No Name]")
            .to_string()
    }

    pub fn insert_char(&mut self, idx: usize, ch: char) {
        self.rope.insert_char(idx, ch);
        self.modified = true;
    }

    pub fn insert(&mut self, idx: usize, text: &str) {
        self.rope.insert(idx, text);
        self.modified = true;
    }

    pub fn remove(&mut self, range: std::ops::Range<usize>) {
        if range.start < range.end && range.end <= self.rope.len_chars() {
            self.rope.remove(range);
            self.modified = true;
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

    pub fn save(&mut self) -> io::Result<()> {
        if let Some(path) = &self.file_path {
            let mut file = fs::File::create(path)?;
            for chunk in self.rope.chunks() {
                file.write_all(chunk.as_bytes())?;
            }
            self.modified = false;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No file path set"))
        }
    }

    pub fn set_file_path(&mut self, path: &str) {
        self.file_path = Some(PathBuf::from(path));
    }

    pub fn get_slice(&self, start: usize, end: usize) -> String {
        if start < end && end <= self.rope.len_chars() {
            self.rope.slice(start..end).to_string()
        } else {
            String::new()
        }
    }
}
