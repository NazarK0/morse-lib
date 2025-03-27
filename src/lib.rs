//! # Morse Library
//!
//! Morse Library is a library parsing text and binary data
//! to Morse Code and vice versa.
//!
//! Morse Library support International rules and codes for Morse
//! Code, but if needed it support custom language(s) to convert any language-specific
//! Morse Code implementations. The library provides **Lines**, **Dots** and **Whitespace**
//! aliasing. That means output Morse Code could be not only lines, dots and whitespaces,
//! but also any UTF-8 emoji or even text! Also the library support playing Morse Code by sound
//! if needed, and customization of speed, frequency of playing.
//!
//! ## Extend multimultilingualism
//!
//! To provide custom language conversion the library has struct MorseCustom. The constructor accept two functions:
//! - first that match conversion from character to Morse Code
//! - second that match conversion from Morse Code to Character
//! 
//! ## Features
//! * international (default)
//! * custom
//! * audio
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
//! 
//! ## Use
//! 
//! This is some examples as a dependency in Cargo.toml of enabling/disabling features in library:
//! ```toml
//! // for international Morse code, no audio:
//! morse-lib = "0.4.3"
//! // for all features:
//! morse-lib = { version = "0.4.3", features = ["custom", "audio"] } 
//! // for custom Morse code only:
//! morse-lib = { version = "0.4.3", default-features = false, features = ["custom"] } 
//! // for custom Morse code only and audio features:
//! morse-lib = { version = "0.4.3", default-features = false, features = ["custom", "audio"] } 
//! ```

// Private modules
mod morse_char;
use morse_char::*;
pub use morse_char::Languages;

mod morse_processors;
use morse_processors::*;

mod display_chars;
use display_chars::DisplayChars;

#[cfg(feature = "audio")]
mod sound;
#[cfg(feature = "audio")]
pub use sound::Sound;

// Public modules
mod morse_unit;
pub use morse_unit::MorseUnit;

mod error;
pub use error::*;

mod morse;
#[cfg(feature = "international")]
pub use morse::Morse;
