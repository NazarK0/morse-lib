use crate::MorseChar;

use super::Morse;
use crate::morse::TMorse;

pub struct MorseIterator<'a> {
    morse: &'a Morse,
    index: usize,
}

impl<'a> MorseIterator<'a> {
    pub fn init(morse: &'a Morse) -> MorseIterator<'a> {
        MorseIterator { morse, index: 0 }
    }
}

impl<'a> Iterator for MorseIterator<'a> {
    type Item = &'a MorseChar;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.morse.len() {
            let result = Some(&self.morse[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

pub struct MorseIntoIterator {
    pub morse: Morse,
}

impl Iterator for MorseIntoIterator {
    type Item = MorseChar;

    fn next(&mut self) -> Option<Self::Item> {
        if self.morse.len() == 0 {
            return None;
        }
        let result = self.morse.remove(0);
        Some(result)
    }
}
