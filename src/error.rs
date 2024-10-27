use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MorseError {
  InvalidChar,
  InvalidMorseSequence,
  InvalidBinary,
}


impl fmt::Display for MorseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MorseError::InvalidChar => write!(f, "Invalid char"),
            MorseError::InvalidMorseSequence => write!(f, "Invalid Morse sequence "),
            MorseError::InvalidBinary => write!(f, "Invalid binary "),

        }
    }
}

impl Error for MorseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}


pub type MorseResult<T> = Result<T, MorseError>;
