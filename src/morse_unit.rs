use std::fmt;

const DOT: &str = ".";
const LINE: &str = "⚊";
const WHITESPACE: &str = " ";

#[derive(Debug, PartialEq, Copy, Clone)]
/// ## Units of Morse Code.
pub enum MorseUnit {
    Dot,
    Line,
    Whitespace, // End Of Word
}

impl MorseUnit {
    
    pub fn from_string(unit: &str) -> Self {
        match unit {
            DOT => MorseUnit::Dot,
            LINE => MorseUnit::Line,
            WHITESPACE => MorseUnit::Whitespace,
            _ => panic!("Invalid Morse Unit"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            MorseUnit::Dot => DOT.to_string(),
            MorseUnit::Line => LINE.to_string(),
            MorseUnit::Whitespace => WHITESPACE.to_string(),
        }
    }
}

impl fmt::Display for MorseUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MorseUnit::Dot => write!(f, "{}", DOT),
            MorseUnit::Line => write!(f, "{}", LINE),
            MorseUnit::Whitespace => write!(f, "{}", WHITESPACE),
        }
    }
}
