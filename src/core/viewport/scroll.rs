pub struct Viewport {
    scroll_offset: usize,
    height: usize,
}

impl Viewport {
    pub fn new(height: usize) -> Self {
        Self {
            scroll_offset: 0,
            height,
        }
    }

    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    pub fn adjust_for_cursor(&mut self, cursor_line: usize) {
        if cursor_line < self.scroll_offset {
            self.scroll_offset = cursor_line;
        } else if cursor_line >= self.scroll_offset + self.height {
            self.scroll_offset = cursor_line - self.height + 1;
        }
    }

    pub fn visible_range(&self) -> std::ops::Range<usize> {
        self.scroll_offset..self.scroll_offset + self.height
    }
}