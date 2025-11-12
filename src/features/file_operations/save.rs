use crate::core::document::{Document, FileIO};
use std::io;

pub struct SaveFile;

impl SaveFile {
    pub fn execute(document: &mut Document) -> io::Result<()> {
        FileIO::save_document(document)
    }
}