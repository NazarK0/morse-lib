use std::fmt;

use crate::MorseResult;

// use super::MorseUnit::Whitespace;
use super::{convert_from_bin, DisplayChars, MorseUnit};

#[cfg(feature = "audio")]
use std::{thread, time};
#[cfg(feature = "audio")]
use super::{sound::TSound, Sound};

#[derive(Debug, PartialEq, Clone)]
pub struct MorseChar {
    m_char: Vec<MorseUnit>,
    letter: char,
    display_as: DisplayChars,
    #[cfg(feature = "audio")]
    sound: Sound,
}

impl MorseChar {
    pub fn from_char(
        letter: char,
        converter: fn(char) -> MorseResult<Vec<MorseUnit>>,
    ) -> MorseResult<MorseChar> {
        let m_char: Vec<MorseUnit> = converter(letter)?;

        Ok(MorseChar {
            m_char,
            letter,
            display_as: DisplayChars::default(),
            #[cfg(feature = "audio")]
            sound: Sound::default(),
        })
    }

    pub fn from_bin(
        letter: &str,
        into_char: fn(Vec<MorseUnit>) -> MorseResult<char>,
    ) -> MorseResult<MorseChar> {
        let m_char: Vec<MorseUnit> = convert_from_bin(letter)?;

        Ok(MorseChar {
            m_char: m_char.clone(),
            letter: into_char(m_char)?,
            display_as: DisplayChars::default(),
            #[cfg(feature = "audio")]
            sound: Sound::default(),
        })
    }

    #[cfg(feature = "audio")]
    pub fn to_beep(&self) {
        for (idx, m_unit) in self.m_char.iter().enumerate() {
            let _ = match m_unit {
                MorseUnit::Dot => {
                    self.sound.play(self.sound.frequency, 1, self.sound.speed);
                }
                MorseUnit::Line => {
                    self.sound.play(self.sound.frequency, 3, self.sound.speed);
                }
                MorseUnit::Whitespace => {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            };

            // The space between parts of the same letter is one unit
            if idx < self.m_char.len() - 1 {
                thread::sleep(time::Duration::from_secs(1));
            }
        }
    }

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

    pub fn dot_as(&mut self, alias: &str) {
        self.display_as.dot = alias.to_string();
    }

    pub fn line_as(&mut self, alias: &str) {
        self.display_as.line = alias.to_string();
    }

    pub fn whitespace_as(&mut self, alias: &str) {
        self.display_as.whitespace = alias.to_string();
    }

    #[cfg(feature = "audio")]
    pub fn frequency(&mut self, frequency: f32) {
        self.sound.frequency = frequency;
    }
    #[cfg(feature = "audio")]
    pub fn play_speed(&mut self, speed: f32) {
        self.sound.speed = speed;
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
                MorseUnit::Dot => string.push_str(&self.display_as.dot),
                MorseUnit::Line => string.push_str(&self.display_as.line),
                MorseUnit::Whitespace => string.push_str(&self.display_as.whitespace),
            }

            // println!("dot len:{}", self.display_as.dot.len());
            // println!("line len:{}", self.display_as.line.len());
            // println!("whitespace len:{}", self.display_as.whitespace.len());

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
    use crate::morse::{from_int_char, into_int_char};

    use super::*;

    #[test]
    #[cfg(feature = "international")]
    fn create_from_text_str() {
        assert_eq!(
            MorseChar::from_char('H', from_int_char)
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
            MorseChar::from_bin(H_BIN, into_int_char)
                .unwrap()
                .to_bin_str(),
            H_BIN
        );
    }

    #[test]
    #[cfg(feature = "international")]
    fn to_string() {
        assert_eq!(
            MorseChar::from_char('u', from_int_char)
                .unwrap()
                .to_string(),
            ". . ⚊"
        );
    }

    #[test]
    #[cfg(feature = "international")]
    fn to_bin_str() {
        assert_eq!(
            MorseChar::from_char('u', from_int_char)
                .unwrap()
                .to_bin_str(),
            "1010111"
        );
    }
    #[test]
    #[cfg(feature = "international")]
    fn set_aliases_for_whitespace_lines_and_dots() {
        let mut morse = MorseChar::from_char('u', from_int_char).unwrap();

        morse.dot_as("🔥");
        morse.line_as("➖");

        assert_eq!(morse.to_string(), "🔥 🔥 ➖");

        let mut morse = MorseChar::from_char(' ', from_int_char).unwrap();

        morse.whitespace_as("🚧");

        assert_eq!(morse.to_string(), "🚧");
    }
}
