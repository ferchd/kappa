#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

impl CursorPosition {
    pub fn new() -> Self {
        Self { line: 0, column: 0 }
    }

    pub fn at(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn move_to(&mut self, line: usize, column: usize) {
        self.line = line;
        self.column = column;
    }

    pub fn reset(&mut self) {
        self.line = 0;
        self.column = 0;
    }
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self::new()
    }
}
