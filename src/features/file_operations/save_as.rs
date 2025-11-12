use crate::core::document::{Document, FileIO};
use std::io;

pub struct SaveFileAs;

impl SaveFileAs {
    pub fn execute(document: &mut Document, path: &str) -> io::Result<()> {
        FileIO::save_document_as(document, path)
    }
}