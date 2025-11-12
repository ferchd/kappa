use crate::core::document::{Document, FileIO};
use std::io;

pub struct OpenFile;

impl OpenFile {
    pub fn execute(path: &str) -> io::Result<Document> {
        FileIO::load_from_file(path)
    }
}