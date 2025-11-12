use crate::core::buffer::RopeBuffer;
use crate::core::document::Document;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub struct FileIO;

impl FileIO {
    pub fn load_from_file(path: &str) -> io::Result<Document> {
        let content = fs::read_to_string(path)?;
        let buffer = RopeBuffer::from_string(&content);
        Ok(Document::from_buffer(buffer, Some(PathBuf::from(path))))
    }

    pub fn save_document(document: &mut Document) -> io::Result<()> {
        if let Some(path) = document.file_path() {
            Self::write_to_file(document.buffer(), path)?;
            document.mark_saved();
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No file path set"))
        }
    }

    pub fn save_document_as(document: &mut Document, path: &str) -> io::Result<()> {
        let path_buf = PathBuf::from(path);
        Self::write_to_file(document.buffer(), &path_buf)?;
        document.set_file_path(path_buf);
        document.mark_saved();
        Ok(())
    }

    fn write_to_file(buffer: &RopeBuffer, path: &Path) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        for chunk in buffer.chunks() {
            file.write_all(chunk.as_bytes())?;
        }
        Ok(())
    }
}