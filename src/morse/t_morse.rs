use crate::{MorseChar, MorseResult};

/// ## Morse Code common methods.
pub trait TMorse {
    fn parse_text(&mut self, text: &str) -> MorseResult<()>;
    fn parse_bin(&mut self, bin: &str) -> MorseResult<()>;
    fn len(&self) -> usize;
    fn remove(&mut self, idx: usize) -> MorseChar;
}
