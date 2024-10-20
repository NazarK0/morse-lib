#[derive(Debug, PartialEq, Clone)]
pub struct DisplayChars {
    pub dot: String,
    pub line: String,
    pub whitespace: String,
}

impl Default for DisplayChars {
    fn default() -> Self {
        Self {
            dot: '.'.to_string(),
            line: '⚊'.to_string(),
            whitespace: ' '.to_string(),
        }
    }
}
