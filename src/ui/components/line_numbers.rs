pub struct LineNumbers;

impl LineNumbers {
    pub fn format(line_idx: usize) -> String {
        format!("{:4} ", line_idx + 1)
    }

    pub fn width() -> usize {
        5
    }
}