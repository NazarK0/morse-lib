use crate::{MorseChar, MorseResult};

/// ## Morse Code common methods.
pub trait TMorse {
    /// Parse Morse Code from text.
    fn parse_text(&mut self, text: &str) -> MorseResult<()>;
    /// Parse Morse Code from binary string.
    fn parse_bin(&mut self, bin: &str) -> MorseResult<()>;
    /// Morse Code string length.
    fn len(&self) -> usize;
    /// Remove Morse Code character by index.
    fn remove(&mut self, idx: usize) -> MorseChar;
}
