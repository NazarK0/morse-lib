#[derive(Debug, PartialEq, Clone)]
pub enum MorseUnit {
    Dot,
    Line,
    Whitespace, // End Of Word
}

impl MorseUnit {
    pub fn to_beep(&self) {}
}
