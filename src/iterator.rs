use crate::{Morse, MorseChar};

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
        if self.index < self.morse.morse_str.len() {
            let result = Some(&self.morse.morse_str[self.index]);
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
        if self.morse.morse_str.len() == 0 {
            return None;
        }
        let result = self.morse.morse_str.remove(0);
        Some(result)
    }
}
