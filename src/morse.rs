mod t_morse;
pub use t_morse::TMorse;

#[cfg(feature = "international")]
mod international;
#[cfg(feature = "international")]
pub use international::Morse;
#[cfg(feature = "custom")]
mod custom;
#[cfg(feature = "custom")]
pub use custom::MorseCustom;
