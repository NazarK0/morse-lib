use std::fmt;
mod languages;
pub use languages::Languages;

use crate::{
    morse::{from_bin_char, from_int_char, into_int_char},
    MorseResult, MorseUnit,
};

use super::MorseUnit::{Dot, Line, Whitespace};

// #[cfg(feature = "audio")]
// use super::{sound::TSound, Sound};
// #[cfg(feature = "audio")]
// use std::{thread, time};

#[derive(Debug, PartialEq, Clone)]
pub struct MorseChar {
    m_char: Vec<MorseUnit>,
    letter: char,
    language: String,
}

impl MorseChar {
    pub fn from_char(letter: char, language: Languages) -> MorseResult<MorseChar> {
        let m_char: Vec<MorseUnit> = match &language {
            #[cfg(feature = "international")]
            Languages::International => from_int_char(letter)?,
            Languages::Custom(_, convert_from_char, _) => convert_from_char(letter)?,
        };

        Ok(MorseChar {
            m_char,
            letter,
            language: language.to_string(),
        })
    }

    pub fn from_bin(letter: &str, language: Languages) -> MorseResult<MorseChar> {
        let m_char: Vec<MorseUnit> = from_bin_char(letter)?;

        let letter: char = match &language {
            #[cfg(feature = "international")]
            Languages::International => into_int_char(m_char.clone())?,
            Languages::Custom(_, _, convert_to_char) => convert_to_char(m_char.clone())?,
        };

        Ok(MorseChar {
            m_char: m_char.clone(),
            letter,
            language: language.to_string(),
        })
    }

    // #[cfg(feature = "audio")]
    // pub fn to_beep(&self) {
    //     for (idx, m_unit) in self.m_char.iter().enumerate() {
    //         let _ = match m_unit {
    //             MorseUnit::Dot => {
    //                 self.sound.play(self.sound.frequency, 1, self.sound.speed);
    //             }
    //             MorseUnit::Line => {
    //                 self.sound.play(self.sound.frequency, 3, self.sound.speed);
    //             }
    //             MorseUnit::Whitespace => {
    //                 std::thread::sleep(std::time::Duration::from_secs(1));
    //             }
    //         };

    //         // The space between parts of the same letter is one unit
    //         if idx < self.m_char.len() - 1 {
    //             thread::sleep(time::Duration::from_secs(1));
    //         }
    //     }
    // }

    pub fn to_bin_str(&self) -> String {
        let mut string = String::new();
        for (idx, m_unit) in self.m_char.iter().enumerate() {
            match m_unit {
                MorseUnit::Dot => string.push_str("1"),
                MorseUnit::Line => string.push_str("111"),
                MorseUnit::Whitespace => string.push_str("0"),
            }

            // The space between parts of the same letter is one unit
            if idx < self.m_char.len() - 1 {
                string.push('0');
            }
        }

        string
    }

    pub fn get_letter(&self) -> char {
        self.letter
    }
}

impl fmt::Display for MorseChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        for (idx, m_unit) in self.m_char.iter().enumerate() {
            match m_unit {
                Dot => string.push_str(&(Dot.to_string())),
                Line => string.push_str(&(Line.to_string())),
                Whitespace => string.push_str(&(Whitespace.to_string())),
            }

            // The space between parts of the same letter is one unit
            if idx < self.m_char.len() - 1 {
                string.push(' ');
            }
        }
        write!(f, "{}", string)
    }
}
#[cfg(test)]
mod morse_char_tests {
    #[cfg(feature = "international")]
    use super::*;

    #[test]
    #[cfg(feature = "international")]
    fn create_from_text_str() {
        assert_eq!(
            MorseChar::from_char('H', Languages::International)
                .unwrap()
                .to_bin_str(),
            "1010101"
        );
    }

    #[test]
    #[cfg(feature = "international")]
    fn create_from_binary_str() {
        const H_BIN: &str = "1010101";
        assert_eq!(
            MorseChar::from_bin(H_BIN, Languages::International)
                .unwrap()
                .to_bin_str(),
            H_BIN
        );
    }

    #[test]
    #[cfg(feature = "international")]
    fn to_string() {
        assert_eq!(
            MorseChar::from_char('u', Languages::International)
                .unwrap()
                .to_string(),
            ". . ⚊"
        );
    }

    #[test]
    #[cfg(feature = "international")]
    fn to_bin_str() {
        assert_eq!(
            MorseChar::from_char('u', Languages::International)
                .unwrap()
                .to_bin_str(),
            "1010111"
        );
    }
}
