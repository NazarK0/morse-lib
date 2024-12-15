use crate::error::MorseError;
use crate::MorseUnit;
use crate::MorseUnit::{Dot, Line};


pub fn convert_from_bin(letter: &str) -> Result<Vec<MorseUnit>, MorseError> {
    let parts: Vec<&str> = letter.split('0').collect();
    let mut morse_letter = Vec::new();
    for unit in parts {
        match unit {
            "111" => morse_letter.push(Line),
            "1" => morse_letter.push(Dot),
            _ => return Err(MorseError::InvalidBinary),
        }
    }

    Ok(morse_letter)
}
