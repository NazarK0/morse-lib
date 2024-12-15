use std::{cell::RefCell, ops::Index};

use crate::{
    display_chars::DisplayChars, from_int_char, into_int_char, MorseChar, MorseResult, MorseUnit,
};

#[cfg(feature = "audio")]
use crate::sound::Sound;
#[cfg(feature = "audio")]
use std::{thread, time};

use super::TMorse;

mod iterator;
use iterator::*;

/// ## International standart Morse Code (default feature).
#[derive(Debug, PartialEq, Clone)]
pub struct Morse {
    morse_str: Vec<MorseChar>,
    display_as: DisplayChars,
    #[cfg(feature = "audio")]
    sound: Sound,
    from_char_converter: fn(char) -> MorseResult<Vec<MorseUnit>>,
    into_char_converter: fn(Vec<MorseUnit>) -> MorseResult<char>,
}

impl TMorse for Morse {
    /// Parse text into Morse Code.
    fn parse_text(&mut self, text: &str) -> MorseResult<()> {
        let mut morse: Vec<MorseChar> = Vec::new();

        for letter in text.chars() {
            morse.push(MorseChar::from_char(letter, self.from_char_converter)?);
        }

        Ok(())
    }

    /// Parse binary into Morse Code.
    fn parse_bin(&mut self, bin: &str) -> MorseResult<()> {
        let words: Vec<&str> = bin.split("0000000").collect();

        for word in words {
            let letters: Vec<&str> = word.split("000").collect();

            for letter in letters {
                self.morse_str
                    .push(MorseChar::from_bin(letter, self.into_char_converter)?);
            }
        }

        Ok(())
    }

    fn len(&self) -> usize {
        self.morse_str.len()
    }

    fn remove(&mut self, idx: usize) -> MorseChar {
        self.morse_str.remove(idx)
    }
}

impl Index<usize> for Morse {
    type Output = MorseChar;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.morse_str[idx]
    }
}

impl Morse {
    /// Creates International Morse Code struct from text.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let morse = Morse::from_text("sos").unwrap();
    ///
    /// assert_eq!(morse.to_string(), ". . .   ⚊ ⚊ ⚊   . . .");
    /// ```
    pub fn from_text(text: &str) -> MorseResult<Morse> {
        let mut morse_str: Vec<MorseChar> = Vec::new();

        for letter in text.chars() {
            morse_str.push(MorseChar::from_char(letter, from_int_char)?);
        }

        Ok(Morse {
            morse_str,
            ..Morse::default()
        })
    }

    /// Creates International Morse Code struct from binary.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let morse = Morse::from_bin("101010001110111011100010101").unwrap();
    ///
    /// assert_eq!(morse.to_string(), ". . .   ⚊ ⚊ ⚊   . . .");
    /// ```
    pub fn from_bin(bin: &str) -> MorseResult<Morse> {
        let words: Vec<&str> = bin.split("0000000").collect();
        let mut morse_str: Vec<MorseChar> = Vec::new();

        for word in words {
            let letters: Vec<&str> = word.split("000").collect();

            for letter in letters {
                morse_str.push(MorseChar::from_bin(letter, into_int_char)?);
            }
        }

        Ok(Morse {
            morse_str,
            ..Morse::default()
        })
    }

    

    /// Creates alias for dot in output string.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let mut morse = Morse::from_text("sos").unwrap();
    /// morse.dot_as("🔥");
    ///
    /// assert_eq!(morse.to_string(), "🔥 🔥 🔥   ⚊ ⚊ ⚊   🔥 🔥 🔥");
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
    /// let mut morse = Morse::from_text("sos").unwrap();
    /// morse.line_as("➖");
    ///
    /// assert_eq!(morse.to_string(), ". . .   ➖ ➖ ➖   . . .");
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
    /// let mut morse = Morse::from_text("s o").unwrap();
    /// morse.whitespace_as("🚧");
    ///
    /// assert_eq!(morse.to_string(), ". . .   🚧   ⚊ ⚊ ⚊");
    /// ```
    pub fn whitespace_as(&mut self, alias: &str) {
        self.display_as.whitespace = alias.to_string();
    }

    /// Play sound that represent Morse Code.
    /// <div class="warning">
    /// 
    /// **Only** available **if "audio"** feature is **enabled.**
    /// 
    /// </div>
    #[cfg(feature = "audio")]
    pub fn to_beep(&self) {
        let morse_str = RefCell::new(self.morse_str.clone());
        for (idx, m_char) in morse_str.borrow_mut().iter_mut().enumerate() {
            m_char.frequency(self.sound.frequency);
            m_char.play_speed(self.sound.speed);

            m_char.to_beep();

            // The space between letters is three units
            if idx < self.morse_str.len() - 1 {
                thread::sleep(time::Duration::from_secs(3));
            }
        }
    }

    /// Set sound frequency in MHz.
    /// <div class="warning">
    /// 
    /// **Only** available **if "audio"** feature is **enabled.**
    /// 
    /// </div>
    /// 
    /// # Examples
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let mut morse = Morse::from_text("s o").unwrap();
    /// morse.frequency(643.0);
    /// ```
    #[cfg(feature = "audio")]
    pub fn frequency(&mut self, frequency: f32) {
        self.sound.frequency = frequency;
    }

    /// Set sound speed.
    /// <div class="warning">
    /// 
    /// **Only** available **if "audio"** feature is **enabled.**
    /// 
    /// </div>
    /// 
    /// * '1' - normal speed
    /// * '> 1' - faster
    /// * '< 1' - slower
    /// 
    /// # Examples
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let mut morse = Morse::from_text("s o").unwrap();
    /// morse.play_speed(2.0);
    /// ```
    #[cfg(feature = "audio")]
    pub fn play_speed(&mut self, speed: f32) {
        self.sound.speed = speed;
    }
    /// Creates binary-formatted Morse Code.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let morse = Morse::from_text("sos").unwrap();
    ///
    /// assert_eq!(morse.to_bin_str(), "101010001110111011100010101");
    /// ```
    pub fn to_bin_str(&self) -> String {
        let mut string = String::new();

        for (idx, m_char) in self.morse_str.iter().enumerate() {
            string.push_str(&m_char.to_bin_str());

            // The space between letters is three units
            if idx < self.morse_str.len() - 1 {
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
    /// let morse = Morse::from_bin("101010001110111011100010101").unwrap();
    /// let text = morse.to_text();
    ///
    /// assert_eq!(text, "sos");
    /// ```
    pub fn to_text(&self) -> String {
        let mut text = String::new();

        for m_char in &self.morse_str {
            text.push(m_char.get_letter());
        }

        text
    }

    /// Now we can iterate:
    /// ```
    /// use morse_lib::Morse;
    ///
    /// let morse = Morse::from_text("sos").unwrap();
    /// for char in morse.iter() {
    ///     println!("{}", char);
    /// }
    /// ```
    pub fn iter(&self) -> MorseIterator {
        MorseIterator::init(self)
    }
}

impl IntoIterator for Morse {
    type Item = MorseChar;
    type IntoIter = MorseIntoIterator;

    /// ```
    /// use morse_lib::Morse;
    ///
    /// let morse = Morse::from_text("sos").unwrap();
    /// for char in morse {
    /// println!("{}", char);
    /// }
    /// ```
    fn into_iter(self) -> MorseIntoIterator {
        MorseIntoIterator { morse: self }
    }
}

impl Default for Morse {
    fn default() -> Self {
        Self {
            morse_str: Vec::new(),
            display_as: DisplayChars::default(),
            #[cfg(feature = "audio")]
            sound: Sound::default(),
            from_char_converter: from_int_char,
            into_char_converter: into_int_char,
        }
    }
}

impl ToString for Morse {
    /// Return String value of Morse Code.
    fn to_string(&self) -> String {
        let mut string = String::new();
        let morse = RefCell::new(self.morse_str.clone());

        for (idx, m_char) in morse.borrow_mut().iter_mut().enumerate() {
            m_char.dot_as(&self.display_as.dot);
            m_char.line_as(&self.display_as.line);
            m_char.whitespace_as(&self.display_as.whitespace);
            string.push_str(&m_char.to_string());

            // The space between letters is three units
            if idx < self.morse_str.len() - 1 {
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
            Morse::from_text("Hello").unwrap().to_bin_str(),
            "1010101000100010111010100010111010100011101110111"
        );
    }

    #[test]
    fn create_from_binary_str() {
        const HELLO_BIN: &str = "1010101000100010111010100010111010100011101110111";
        assert_eq!(Morse::from_bin(HELLO_BIN).unwrap().to_bin_str(), HELLO_BIN);
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Morse::from_text("hi u").unwrap().to_string(),
            ". . . .   . .       . . ⚊"
        );
    }

    #[test]
    fn to_bin_str() {
        assert_eq!(
            Morse::from_text("hi u").unwrap().to_bin_str(),
            "101010100010100000001010111"
        );
    }
    #[test]
    fn set_aliases_for_whitespace_lines_and_dots() {
        let mut morse = Morse::from_text("hi u").unwrap();

        morse.dot_as("🔥");
        morse.line_as("➖");
        morse.whitespace_as("🚧");

        assert_eq!(morse.to_string(), "🔥 🔥 🔥 🔥   🔥 🔥   🚧   🔥 🔥 ➖");
    }
}
