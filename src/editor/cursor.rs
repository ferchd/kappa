#[derive(Debug, Clone)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { line: 0, column: 0 }
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

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}
