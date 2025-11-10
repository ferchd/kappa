#[derive(Debug, Clone, PartialEq)]
pub enum EditorMode {
    Normal,
    CommandPalette,
    OpenFile,
    SaveAs,
}

impl EditorMode {
    pub fn is_dialog(&self) -> bool {
        matches!(
            self,
            EditorMode::CommandPalette | EditorMode::OpenFile | EditorMode::SaveAs
        )
    }

    pub fn dialog_title(&self) -> &'static str {
        match self {
            EditorMode::CommandPalette => "Command Palette",
            EditorMode::OpenFile => "Open File",
            EditorMode::SaveAs => "Save As",
            _ => "",
        }
    }
}
