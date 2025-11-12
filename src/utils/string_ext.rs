pub trait StringExt {
    fn trim_line_endings(&self) -> &str;
}

impl StringExt for str {
    fn trim_line_endings(&self) -> &str {
        self.trim_end_matches(&['\n', '\r'][..])
    }
}