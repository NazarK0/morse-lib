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

// Private modules
mod morse_char;
use morse_char::*;

mod morse_processors;
use morse_processors::*;

mod display_chars;
use display_chars::DisplayChars;

mod sound;
use sound::Sound;

// Public modules
mod morse_unit;
pub use morse_unit::MorseUnit;

mod error;
pub use error::*;

mod morse;
pub use morse::{Morse, MorseCustom, TMorse};
// pub use morse;
