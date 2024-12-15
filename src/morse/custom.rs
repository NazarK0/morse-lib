use std::{cell::RefCell, ops::Index, thread, time};

use crate::{display_chars::DisplayChars, sound::Sound, MorseChar, MorseResult, MorseUnit};

mod iterator;
use iterator::*;

use super::TMorse;

/// ## Main library struct.
///
/// All magic going here
#[derive(Debug, PartialEq, Clone)]
pub struct MorseCustom {
    morse_str: Vec<MorseChar>,
    display_as: DisplayChars,
    sound: Sound,
    from_char_converter: fn(char) -> MorseResult<Vec<MorseUnit>>,
    into_char_converter: fn(Vec<MorseUnit>) -> MorseResult<char>,
}

impl TMorse for MorseCustom {
    /// Parse text into Morse Code.
    fn parse_text(&mut self, text: &str) -> MorseResult<()> {
        let mut morse: Vec<MorseChar> = Vec::new();

        for letter in text.chars() {
            morse.push(MorseChar::from_char(
                letter,
                self.from_char_converter,
            )?);
        }

        self.morse_str  = morse;
        Ok(())
    }

    /// Parse binary into Morse Code.
    fn parse_bin(&mut self, bin: &str) -> MorseResult<()> {
        let words: Vec<&str> = bin.split("0000000").collect();

        for word in words {
            let letters: Vec<&str> = word.split("000").collect();

            for letter in letters {
                self.morse_str.push(MorseChar::from_bin(
                    letter,
                    self.into_char_converter,
                )?);
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

impl Index<usize> for MorseCustom {
    type Output = MorseChar;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.morse_str[idx]
    }
}

impl MorseCustom {
    /// Creates custom language Morse Code struct.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::{MorseCustom, MorseUnit, MorseError, MorseResult};
    /// use MorseUnit::{Dot, Line, Whitespace};
    ///
    /// fn from_char(letter: char) -> MorseResult<Vec<MorseUnit>>{
    ///     match letter {
    ///         'а' | 'А' => Ok(vec![Dot, Line]),
    ///         'б' | 'Б' => Ok(vec![Line, Dot, Dot, Dot]),
    ///         'в' | 'В' => Ok(vec![Dot, Line, Line]),
    ///         'г' | 'Г' => Ok(vec![Dot, Dot, Dot, Dot]),
    ///         ' ' => Ok(vec![Whitespace]),
    ///           _ => Err(MorseError::InvalidChar)
    ///     }
    /// }
    ///
    /// fn into_char(letter: Vec<MorseUnit>) -> MorseResult<char> {
    ///     if letter.len() == 1 && letter[0] == Whitespace {
    ///         return Ok(' ');
    ///     } else if letter.len() == 2 && letter[0] == Dot && letter[1] == Line {
    ///         return Ok('а')
    ///     } else if letter.len() == 3 && letter[0] == Dot && letter[1] == Line && letter[2] == Line {
    ///         return Ok('в');
    ///     } else if letter.len() == 4 {
    ///         if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
    ///             return Ok('б');
    ///         } else {
    ///             return Ok('г');
    ///         }
    ///     } else {
    ///         Err(MorseError::InvalidMorseSequence)
    ///     }
    /// }
    ///
    /// let morse_ua = MorseCustom::new(from_char, into_char);
    /// ```
    pub fn new(
        from_char: fn(char) -> MorseResult<Vec<MorseUnit>>,
        into_char: fn(Vec<MorseUnit>) -> MorseResult<char>,
    ) -> MorseCustom {
        MorseCustom {
            morse_str: Vec::new(),
            display_as: DisplayChars::default(),
            sound: Sound::default(),
            from_char_converter: from_char,
            into_char_converter: into_char,
        }
    }

    

    

    /// Play sound that represent Morse Code.
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

    /// Creates alias for dot in output string.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::{MorseCustom, MorseUnit, MorseError, MorseResult, TMorse};
    /// use MorseUnit::{Dot, Line, Whitespace};
    ///
    /// fn from_char(letter: char) -> MorseResult<Vec<MorseUnit>>{
    ///     match letter {
    ///         'а' | 'А' => Ok(vec![Dot, Line]),
    ///         'б' | 'Б' => Ok(vec![Line, Dot, Dot, Dot]),
    ///         'в' | 'В' => Ok(vec![Dot, Line, Line]),
    ///         'г' | 'Г' => Ok(vec![Dot, Dot, Dot, Dot]),
    ///         ' ' => Ok(vec![Whitespace]),
    ///           _ => Err(MorseError::InvalidChar)
    ///     }
    /// }
    ///
    /// fn into_char(letter: Vec<MorseUnit>) -> MorseResult<char> {
    ///     if letter.len() == 1 && letter[0] == Whitespace {
    ///         return Ok(' ');
    ///     } else if letter.len() == 2 && letter[0] == Dot && letter[1] == Line {
    ///         return Ok('а')
    ///     } else if letter.len() == 3 && letter[0] == Dot && letter[1] == Line && letter[2] == Line {
    ///         return Ok('в');
    ///     } else if letter.len() == 4 {
    ///         if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
    ///             return Ok('б');
    ///         } else {
    ///             return Ok('г');
    ///         }
    ///     } else {
    ///         Err(MorseError::InvalidMorseSequence)
    ///     }
    /// }
    ///
    /// let mut morse = MorseCustom::new(from_char, into_char);
    ///
    /// morse.parse_text("ба").unwrap();
    /// morse.dot_as("🔥");
    ///
    /// assert_eq!(
    ///        morse.to_string(),
    ///        "⚊ 🔥 🔥 🔥   🔥 ⚊"
    ///    );
    /// ```
    pub fn dot_as(&mut self, alias: &str) {
        self.display_as.dot = alias.to_string();
    }
    /// Creates alias for line in output string.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::{MorseCustom, MorseUnit, MorseError, MorseResult, TMorse};
    /// use MorseUnit::{Dot, Line, Whitespace};
    ///
    /// fn from_char(letter: char) -> MorseResult<Vec<MorseUnit>>{
    ///     match letter {
    ///         'а' | 'А' => Ok(vec![Dot, Line]),
    ///         'б' | 'Б' => Ok(vec![Line, Dot, Dot, Dot]),
    ///         'в' | 'В' => Ok(vec![Dot, Line, Line]),
    ///         'г' | 'Г' => Ok(vec![Dot, Dot, Dot, Dot]),
    ///         ' ' => Ok(vec![Whitespace]),
    ///           _ => Err(MorseError::InvalidChar)
    ///     }
    /// }
    ///
    /// fn into_char(letter: Vec<MorseUnit>) -> MorseResult<char> {
    ///     if letter.len() == 1 && letter[0] == Whitespace {
    ///         return Ok(' ');
    ///     } else if letter.len() == 2 && letter[0] == Dot && letter[1] == Line {
    ///         return Ok('а')
    ///     } else if letter.len() == 3 && letter[0] == Dot && letter[1] == Line && letter[2] == Line {
    ///         return Ok('в');
    ///     } else if letter.len() == 4 {
    ///         if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
    ///             return Ok('б');
    ///         } else {
    ///             return Ok('г');
    ///         }
    ///     } else {
    ///         Err(MorseError::InvalidMorseSequence)
    ///     }
    /// }
    ///
    /// let mut morse = MorseCustom::new( from_char, into_char);
    ///
    /// morse.parse_text("ба").unwrap();
    /// morse.line_as("➖");
    ///
    /// assert_eq!(
    ///        morse.to_string(),
    ///        "➖ . . .   . ➖"
    ///    );
    /// ```
    pub fn line_as(&mut self, alias: &str) {
        self.display_as.line = alias.to_string();
    }
    /// Creates alias for whitespace in output string.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::{MorseCustom, MorseUnit, MorseError, MorseResult, TMorse};
    /// use MorseUnit::{Dot, Line, Whitespace};
    ///
    /// fn from_char(letter: char) -> MorseResult<Vec<MorseUnit>>{
    ///     match letter {
    ///         'а' | 'А' => Ok(vec![Dot, Line]),
    ///         'б' | 'Б' => Ok(vec![Line, Dot, Dot, Dot]),
    ///         'в' | 'В' => Ok(vec![Dot, Line, Line]),
    ///         'г' | 'Г' => Ok(vec![Dot, Dot, Dot, Dot]),
    ///         ' ' => Ok(vec![Whitespace]),
    ///           _ => Err(MorseError::InvalidChar)
    ///     }
    /// }
    ///
    /// fn into_char(letter: Vec<MorseUnit>) -> MorseResult<char> {
    ///     if letter.len() == 1 && letter[0] == Whitespace {
    ///         return Ok(' ');
    ///     } else if letter.len() == 2 && letter[0] == Dot && letter[1] == Line {
    ///         return Ok('а')
    ///     } else if letter.len() == 3 && letter[0] == Dot && letter[1] == Line && letter[2] == Line {
    ///         return Ok('в');
    ///     } else if letter.len() == 4 {
    ///         if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
    ///             return Ok('б');
    ///         } else {
    ///             return Ok('г');
    ///         }
    ///     } else {
    ///         Err(MorseError::InvalidMorseSequence)
    ///     }
    /// }
    ///
    /// let mut morse = MorseCustom::new( from_char, into_char);
    ///
    /// morse.parse_text("б а").unwrap();
    /// morse.whitespace_as("🚧");
    ///
    /// assert_eq!(
    ///        morse.to_string(),
    ///        "⚊ . . .   🚧   . ⚊"
    ///    );
    /// ```
    pub fn whitespace_as(&mut self, alias: &str) {
        self.display_as.whitespace = alias.to_string();
    }
    /// Set sound frequency in MHz.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::{MorseCustom, MorseUnit, MorseError, MorseResult, TMorse};
    /// use MorseUnit::{Dot, Line, Whitespace};
    ///
    /// fn from_char(letter: char) -> MorseResult<Vec<MorseUnit>>{
    ///     match letter {
    ///         'а' | 'А' => Ok(vec![Dot, Line]),
    ///         'б' | 'Б' => Ok(vec![Line, Dot, Dot, Dot]),
    ///         'в' | 'В' => Ok(vec![Dot, Line, Line]),
    ///         'г' | 'Г' => Ok(vec![Dot, Dot, Dot, Dot]),
    ///         ' ' => Ok(vec![Whitespace]),
    ///           _ => Err(MorseError::InvalidChar)
    ///     }
    /// }
    ///
    /// fn into_char(letter: Vec<MorseUnit>) -> MorseResult<char> {
    ///     if letter.len() == 1 && letter[0] == Whitespace {
    ///         return Ok(' ');
    ///     } else if letter.len() == 2 && letter[0] == Dot && letter[1] == Line {
    ///         return Ok('а')
    ///     } else if letter.len() == 3 && letter[0] == Dot && letter[1] == Line && letter[2] == Line {
    ///         return Ok('в');
    ///     } else if letter.len() == 4 {
    ///         if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
    ///             return Ok('б');
    ///         } else {
    ///             return Ok('г');
    ///         }
    ///     } else {
    ///         Err(MorseError::InvalidMorseSequence)
    ///     }
    /// }
    ///
    /// let mut morse = MorseCustom::new(from_char, into_char);
    ///
    /// morse.parse_text("б а").unwrap();
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
    /// use morse_lib::{MorseCustom, MorseUnit, MorseError, MorseResult, TMorse};
    /// use MorseUnit::{Dot, Line, Whitespace};
    ///
    /// fn from_char(letter: char) -> MorseResult<Vec<MorseUnit>>{
    ///     match letter {
    ///         'а' | 'А' => Ok(vec![Dot, Line]),
    ///         'б' | 'Б' => Ok(vec![Line, Dot, Dot, Dot]),
    ///         'в' | 'В' => Ok(vec![Dot, Line, Line]),
    ///         'г' | 'Г' => Ok(vec![Dot, Dot, Dot, Dot]),
    ///         ' ' => Ok(vec![Whitespace]),
    ///           _ => Err(MorseError::InvalidChar)
    ///     }
    /// }
    ///
    /// fn into_char(letter: Vec<MorseUnit>) -> MorseResult<char> {
    ///     if letter.len() == 1 && letter[0] == Whitespace {
    ///         return Ok(' ');
    ///     } else if letter.len() == 2 && letter[0] == Dot && letter[1] == Line {
    ///         return Ok('а')
    ///     } else if letter.len() == 3 && letter[0] == Dot && letter[1] == Line && letter[2] == Line {
    ///         return Ok('в');
    ///     } else if letter.len() == 4 {
    ///         if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
    ///             return Ok('б');
    ///         } else {
    ///             return Ok('г');
    ///         }
    ///     } else {
    ///         Err(MorseError::InvalidMorseSequence)
    ///     }
    /// }
    ///
    /// let mut morse = MorseCustom::new( from_char, into_char);
    ///
    /// morse.parse_text("б а").unwrap();
    /// morse.play_speed(2.0);
    /// ```
    pub fn play_speed(&mut self, speed: f32) {
        self.sound.speed = speed;
    }
    /// Creates binary-formatted Morse Code.
    /// # Examples
    ///
    /// ```
    /// use morse_lib::{MorseCustom, MorseUnit, MorseError, MorseResult, TMorse};
    /// use MorseUnit::{Dot, Line, Whitespace};
    ///
    /// fn from_char(letter: char) -> MorseResult<Vec<MorseUnit>>{
    ///     match letter {
    ///         'а' | 'А' => Ok(vec![Dot, Line]),
    ///         'б' | 'Б' => Ok(vec![Line, Dot, Dot, Dot]),
    ///         'в' | 'В' => Ok(vec![Dot, Line, Line]),
    ///         'г' | 'Г' => Ok(vec![Dot, Dot, Dot, Dot]),
    ///         ' ' => Ok(vec![Whitespace]),
    ///           _ => Err(MorseError::InvalidChar)
    ///     }
    /// }
    ///
    /// fn into_char(letter: Vec<MorseUnit>) -> MorseResult<char> {
    ///     if letter.len() == 1 && letter[0] == Whitespace {
    ///         return Ok(' ');
    ///     } else if letter.len() == 2 && letter[0] == Dot && letter[1] == Line {
    ///         return Ok('а')
    ///     } else if letter.len() == 3 && letter[0] == Dot && letter[1] == Line && letter[2] == Line {
    ///         return Ok('в');
    ///     } else if letter.len() == 4 {
    ///         if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
    ///             return Ok('б');
    ///         } else {
    ///             return Ok('г');
    ///         }
    ///     } else {
    ///         Err(MorseError::InvalidMorseSequence)
    ///     }
    /// }
    ///
    /// let mut morse = MorseCustom::new(from_char, into_char);
    ///
    /// morse.parse_text("б а").unwrap();
    ///
    /// assert_eq!(morse.to_bin_str(),"111010101000000010111");
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
    pub fn to_text(&self) -> String {
        let mut text = String::new();

        for m_char in &self.morse_str {
            text.push(m_char.get_letter());
        }

        text
    }

    pub fn iter(&self) -> MorseIterator {
        MorseIterator::init(self)
    }
}

impl IntoIterator for MorseCustom {
    type Item = MorseChar;
    type IntoIter = MorseIntoIterator;

    fn into_iter(self) -> MorseIntoIterator {
        MorseIntoIterator { morse: self }
    }
}

impl ToString for MorseCustom {
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
    use crate::morse_unit::MorseUnit::{Dot, Line, Whitespace};
    use crate::{MorseError, MorseResult, MorseUnit};

    fn from_char(letter: char) -> MorseResult<Vec<MorseUnit>> {
        println!("{} confver", letter.to_ascii_lowercase());
        match letter {
            'а' | 'А' => Ok(vec![Dot, Line]),
            'б' | 'Б' => Ok(vec![Line, Dot, Dot, Dot]),
            'в' | 'В' => Ok(vec![Dot, Line, Line]),
            'г' | 'Г' => Ok(vec![Dot, Dot, Dot, Dot]),
            ' ' => Ok(vec![Whitespace]),
            _ => Err(MorseError::InvalidChar),
        }
    }
    fn into_char(letter: Vec<MorseUnit>) -> MorseResult<char> {
        if letter.len() == 1 && letter[0] == Whitespace {
            return Ok(' ');
        } else if letter.len() == 2 && letter[0] == Dot && letter[1] == Line {
            return Ok('а');
        } else if letter.len() == 3 && letter[0] == Dot && letter[1] == Line && letter[2] == Line {
            return Ok('в');
        } else if letter.len() == 4 {
            if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
                return Ok('б');
            } else {
                return Ok('г');
            }
        } else {
            Err(MorseError::InvalidMorseSequence)
        }
    }

    #[test]
    fn create_from_text_str() {
        let mut morse = MorseCustom::new(from_char, into_char);
        morse.parse_text("Ба").unwrap();

        assert_eq!(
            morse.to_bin_str(),
            "11101010100010111"
        );
    }

    #[test]
    fn create_from_binary_str() {
        const BIN: &str = "11101010100010111";
        let mut morse = MorseCustom::new(from_char, into_char);

        morse.parse_bin(BIN).unwrap();

        assert_eq!(morse.to_text(), "ба");
    }

    #[test]
    fn to_string() {
        let mut morse = MorseCustom::new(from_char, into_char);
        morse.parse_text("ба").unwrap();
        
        assert_eq!(morse.to_string(), "⚊ . . .   . ⚊");
    }

    #[test]
    fn to_bin_str() {
        let mut morse = MorseCustom::new(from_char, into_char);
        morse.parse_text("ба").unwrap();
        println!("{} BIIIN", morse.to_bin_str());

        assert_eq!(morse.to_bin_str(), "11101010100010111");
    }
    #[test]
    fn set_aliases_for_whitespace_lines_and_dots() {
        let mut morse = MorseCustom::new(from_char, into_char);
        morse.parse_text("ба ба").unwrap();

        println!("{} TEXXT", morse.to_text());

        morse.dot_as("🔥");
        morse.line_as("➖");
        morse.whitespace_as("🚧");

        assert_eq!(morse.to_string(), "➖ 🔥 🔥 🔥   🔥 ➖   🚧   ➖ 🔥 🔥 🔥   🔥 ➖");
    }
}
