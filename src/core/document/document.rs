use crate::core::buffer::RopeBuffer;
use std::path::PathBuf;

pub struct Document {
    buffer: RopeBuffer,
    file_path: Option<PathBuf>,
    modified: bool,
}

impl Document {
    pub fn new() -> Self {
        Self {
            buffer: RopeBuffer::new(),
            file_path: None,
            modified: false,
        }
    }

    pub fn from_buffer(buffer: RopeBuffer, path: Option<PathBuf>) -> Self {
        Self {
            buffer,
            file_path: path,
            modified: false,
        }
    }

    pub fn buffer(&self) -> &RopeBuffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut RopeBuffer {
        self.modified = true;
        &mut self.buffer
    }

    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn set_file_path(&mut self, path: PathBuf) {
        self.file_path = Some(path);
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    pub fn mark_modified(&mut self) {
        self.modified = true;
    }

    pub fn file_name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("[No Name]")
            .to_string()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}