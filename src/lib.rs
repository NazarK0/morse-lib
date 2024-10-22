//! # Morse Library
//!
//! Morse Library is a library parsing text and binary data
//! to Morse Code and vice versa.
//!
//! By default Morse Library support only International rules and codes for Morse
//! Code, but if needed it support extend metods to convert any language-specific
//! Morse Code implementations. The library provides **Lines**, **Dots** and **Whitespace**
//! aliasing. That means output Morse Code could be not only lines, dots and whitespaces,
//! but also any UTF-8 emoji or even text! Also the library support playing Morse Code by sound
//! if needed, and customization of speed, frequency of playing.
//!
//! ## Extend multimultilingualism
//!
//! To provide custom language conversion the library accept two functions:
//! - first that match conversion from character to Morse Code
//! - second that match conversion from Morse Code to Character
//!
//! ## Data formats
//!
//! The following is a list of data formats that have been implemented
//! for Morse Library.
//!
//! ### Input
//!
//! - [String], the casual String or &str that contains text
//! - [Binary String], the casual String or &str that contains Morse Code represented by byte code.
//!
//! ### Output
//!
//! - [String], the casual String that contains Morse Code. By default **lines** and **dots**, but could be
//!   any UTF-8 character or even string
//! - [Binary String], the casual String that contains Morse Code represented by byte code.
//! - [Sound], sound representation of Morse Code

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

/// ## Main library struct.
///
/// All magic going here
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
    /// Creates extended Morse Code struct.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::{Morse, MorseUnit};
    /// use MorseUnit::{Dot, Line, Whitespace};
    ///
    /// fn from_char(letter: char) -> Vec<MorseUnit> {
    ///     match letter {
    ///         'a' => vec![Dot, Line],
    ///         'б' => vec![Line, Dot, Dot, Dot],
    ///         'в' => vec![Dot, Line, Line],
    ///         'г' => vec![Dot, Dot, Dot, Dot],
    ///         ' ' => vec![Whitespace],
    ///           _ => panic!("Wrong character")
    ///     }
    /// }
    ///
    /// fn into_char(letter: Vec<MorseUnit>) -> char {
    ///     if letter.len() == 1 && letter[0] == Whitespace {
    ///         return ' ';
    ///     } else if letter.len() == 2 && letter[0] == Dot && letter[1] == Line {
    ///         return 'а'
    ///     } else if letter.len() == 3 && letter[0] == Dot && letter[1] == Line && letter[2] == Line {
    ///         return 'в';
    ///     } else if letter.len() == 4 {
    ///         if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
    ///             return 'б';
    ///         } else {
    ///             return 'г';
    ///         }
    ///     } else {
    ///         panic!("Wrong Morse Char sequence")
    ///     }
    /// }
    ///
    /// let morse = Morse::new("Ukrainian".to_string(), from_char, into_char);
    /// ```
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
    /// Creates International Morse Code struct from text.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let morse = Morse::from_int_text("sos");
    ///
    /// assert_eq!(
    ///        morse.to_string(),
    ///        ". . .   ⚊ ⚊ ⚊   . . ."
    ///    );
    /// ```
    pub fn from_int_text(text: &str) -> Morse {
        let mut morse: Vec<MorseChar> = Vec::new();

        for letter in text.chars() {
            morse.push(MorseChar::from_char(letter, "International", from_int_char));
        }

        Morse {
            morse,
            ..Morse::default()
        }
    }
    /// Parse text into Morse Code.
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

    /// Creates International Morse Code struct from binary.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let morse = Morse::from_int_bin("101010001110111011100010101");
    ///
    /// assert_eq!(
    ///        morse.to_string(),
    ///        ". . .   ⚊ ⚊ ⚊   . . ."
    ///    );
    /// ```
    pub fn from_int_bin(bin: &str) -> Morse {
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
    /// Parse binary into Morse Code.
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

    /// Play sound that represent Morse Code.
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
    /// Return String value that contains stored language label.
    pub fn get_language(&self) -> String {
        self.language.clone()
    }
    /// Creates alias for dot in output string.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let mut morse = Morse::from_int_text("sos");
    /// morse.dot_as("🔥");
    ///
    /// assert_eq!(
    ///        morse.to_string(),
    ///        "🔥 🔥 🔥   ⚊ ⚊ ⚊   🔥 🔥 🔥"
    ///    );
    /// ```
    pub fn dot_as(&mut self, alias: &str) {
        self.display_as.dot = alias.to_string();
    }
    /// Creates alias for line in output string.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let mut morse = Morse::from_int_text("sos");
    /// morse.line_as("➖");
    ///
    /// assert_eq!(
    ///        morse.to_string(),
    ///        ". . .   ➖ ➖ ➖   . . ."
    ///    );
    /// ```
    pub fn line_as(&mut self, alias: &str) {
        self.display_as.line = alias.to_string();
    }
    /// Creates alias for whitespace in output string.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let mut morse = Morse::from_int_text("s o");
    /// morse.whitespace_as("🚧");
    ///
    /// assert_eq!(
    ///        morse.to_string(),
    ///        ". . .   🚧   ⚊ ⚊ ⚊"
    ///    );
    /// ```
    pub fn whitespace_as(&mut self, alias: &str) {
        self.display_as.whitespace = alias.to_string();
    }
    /// Set sound frequency in MHz.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let mut morse = Morse::from_int_text("s o");
    /// morse.frequency(643.0);
    /// ```
    pub fn frequency(&mut self, frequency: f32) {
        self.sound.frequency = frequency;
    }
    /// Set sound speed.
    /// 1 - normal speed
    /// > 1 - faster
    /// < 1 - slower
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let mut morse = Morse::from_int_text("s o");
    /// morse.play_speed(2.0);
    /// ```
    pub fn play_speed(&mut self, speed: f32) {
        self.sound.speed = speed;
    }
    /// Creates binary-formatted Morse Code.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let morse = Morse::from_int_text("sos");
    ///
    /// assert_eq!(
    ///        morse.to_bin_str(),
    ///        "101010001110111011100010101"
    ///    );
    /// ```
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
    /// Convert Morse Code back to text.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let morse = Morse::from_int_bin("101010001110111011100010101");
    /// let text = morse.to_text();
    ///
    /// assert_eq!(
    ///        text,
    ///        "sos"
    ///    );
    /// ```
    pub fn to_text(&self) -> String {
        let mut text = String::new();

        for m_char in &self.morse {
            text.push(m_char.get_letter());
        }

        text
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
            Morse::from_int_text("Hello").to_bin_str(),
            "1010101000100010111010100010111010100011101110111"
        );
    }

    #[test]
    fn create_from_binary_str() {
        const HELLO_BIN: &str = "1010101000100010111010100010111010100011101110111";
        assert_eq!(Morse::from_int_bin(HELLO_BIN).to_bin_str(), HELLO_BIN);
    }

    #[test]
    fn get_language() {
        assert_eq!(
            Morse::from_int_text("hello").get_language(),
            "International".to_string()
        );
        assert_eq!(
            Morse::from_int_bin("1").get_language(),
            "International".to_string()
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Morse::from_int_text("hi u").to_string(),
            ". . . .   . .       . . ⚊"
        );
    }

    #[test]
    fn to_bin_str() {
        assert_eq!(
            Morse::from_int_text("hi u").to_bin_str(),
            "101010100010100000001010111"
        );
    }
    #[test]
    fn set_aliases_for_whitespace_lines_and_dots() {
        let mut morse = Morse::from_int_text("hi u");

        morse.dot_as("🔥");
        morse.line_as("➖");
        morse.whitespace_as("🚧");

        assert_eq!(morse.to_string(), "🔥 🔥 🔥 🔥   🔥 🔥   🚧   🔥 🔥 ➖");
    }
}
