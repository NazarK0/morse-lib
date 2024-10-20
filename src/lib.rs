use std::{cell::RefCell, thread, time};

mod morse_char;
use morse_char::*;

mod morse_unit;
pub use morse_unit::MorseUnit;

mod morse_processors;
use morse_processors::*;

mod display_chars;
use display_chars::DisplayChars;

mod sound;
use sound::Sound;

#[derive(Debug, PartialEq, Clone)]
pub struct Morse {
    morse: Vec<MorseChar>,
    language: String,
    display_as: DisplayChars,
    sound: Sound,
    from_char_converter: fn(char) -> Vec<MorseUnit>,
    into_char_converter: fn(Vec<MorseUnit>) -> char,
}

impl Morse {
    pub fn new(
        language: String,
        from_char: fn(char) -> Vec<MorseUnit>,
        into_char: fn(Vec<MorseUnit>) -> char,
    ) -> Morse {
        Morse {
            morse: Vec::new(),
            language,
            display_as: DisplayChars::default(),
            sound: Sound::default(),
            from_char_converter: from_char,
            into_char_converter: into_char,
        }
    }
    pub fn from_str(text: &str) -> Morse {
        let mut morse: Vec<MorseChar> = Vec::new();

        for letter in text.chars() {
            morse.push(MorseChar::from_char(letter, "International", from_int_char));
        }

        Morse {
            morse,
            ..Morse::default()
        }
    }

    pub fn parse_text(&mut self, text: &str) {
        let mut morse: Vec<MorseChar> = Vec::new();

        for letter in text.chars() {
            morse.push(MorseChar::from_char(
                letter,
                &self.language,
                self.from_char_converter,
            ));
        }
    }

    pub fn from_bin(bin: &str) -> Morse {
        let words: Vec<&str> = bin.split("0000000").collect();
        let mut morse: Vec<MorseChar> = Vec::new();

        for word in words {
            let letters: Vec<&str> = word.split("000").collect();

            for letter in letters {
                morse.push(MorseChar::from_bin(letter, "International", into_int_char));
            }
        }

        Morse {
            morse,
            ..Morse::default()
        }
    }

    pub fn parse_bin(&mut self, bin: &str) {
        let words: Vec<&str> = bin.split("0000000").collect();

        for word in words {
            let letters: Vec<&str> = word.split("000").collect();

            for letter in letters {
                self.morse.push(MorseChar::from_bin(
                    letter,
                    &self.language,
                    self.into_char_converter,
                ));
            }
        }
    }

    pub fn to_beep(&self) {
        let morse = RefCell::new(self.morse.clone());
        for (idx, m_char) in morse.borrow_mut().iter_mut().enumerate() {
            m_char.frequency(self.sound.frequency);
            m_char.play_speed(self.sound.speed);

            m_char.to_beep();

            // The space between letters is three units
            if idx < self.morse.len() - 1 {
                thread::sleep(time::Duration::from_secs(3));
            }
        }
    }

    pub fn get_language(&self) -> String {
        self.language.clone()
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

    pub fn frequency(&mut self, frequency: f32) {
        self.sound.frequency = frequency;
    }
    pub fn play_speed(&mut self, speed: f32) {
        self.sound.speed = speed;
    }

    pub fn to_bin_str(&self) -> String {
        let mut string = String::new();

        for (idx, m_char) in self.morse.iter().enumerate() {
            string.push_str(&m_char.to_bin_str());

            // The space between letters is three units
            if idx < self.morse.len() - 1 {
                string.push_str("000");
            }
        }

        string
    }
}

impl Default for Morse {
    fn default() -> Self {
        Self {
            morse: Vec::new(),
            language: "International".to_string(),
            display_as: DisplayChars::default(),
            sound: Sound::default(),
            from_char_converter: from_int_char,
            into_char_converter: into_int_char,
        }
    }
}

impl ToString for Morse {
    fn to_string(&self) -> String {
        let mut string = String::new();
        let morse = RefCell::new(self.morse.clone());

        for (idx, m_char) in morse.borrow_mut().iter_mut().enumerate() {
            m_char.dot_as(&self.display_as.dot);
            m_char.line_as(&self.display_as.line);
            m_char.whitespace_as(&self.display_as.whitespace);
            string.push_str(&m_char.to_string());

            // The space between letters is three units
            if idx < self.morse.len() - 1 {
                string.push_str("   ");
            }
        }

        string
    }
}

#[cfg(test)]
mod morse_tests {
    use super::*;

    #[test]
    fn create_from_text_str() {
        assert_eq!(
            Morse::from_str("Hello").to_bin_str(),
            "1010101000100010111010100010111010100011101110111"
        );
    }

    #[test]
    fn create_from_binary_str() {
        const HELLO_BIN: &str = "1010101000100010111010100010111010100011101110111";
        assert_eq!(Morse::from_bin(HELLO_BIN).to_bin_str(), HELLO_BIN);
    }

    #[test]
    fn get_language() {
        assert_eq!(
            Morse::from_str("hello").get_language(),
            "International".to_string()
        );
        assert_eq!(
            Morse::from_bin("1").get_language(),
            "International".to_string()
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Morse::from_str("hi u").to_string(),
            ". . . .   . .       . . ⚊"
        );
    }

    #[test]
    fn to_bin_str() {
        assert_eq!(
            Morse::from_str("hi u").to_bin_str(),
            "101010100010100000001010111"
        );
    }
    #[test]
    fn set_aliases_for_whitespace_lines_and_dots() {
        let mut morse = Morse::from_str("hi u");

        morse.dot_as("🔥");
        morse.line_as("➖");
        morse.whitespace_as("🚧");

        assert_eq!(morse.to_string(), "🔥 🔥 🔥 🔥   🔥 🔥   🚧   🔥 🔥 ➖");
    }
}
