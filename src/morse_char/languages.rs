use std::fmt;

use crate::{MorseResult, MorseUnit};

type  FromChar = fn(char) -> MorseResult<Vec<MorseUnit>>;
type  ToChar = fn(Vec<MorseUnit>) -> MorseResult<char>;


#[derive(Debug, PartialEq, Clone)]
pub enum Languages {
    International,
    Custom(String, FromChar, ToChar),
}

impl fmt::Display for Languages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Languages::International => write!(f, "International"),
            Languages::Custom(name, _, _) => write!(f, "{}", name),
        }
    }
}