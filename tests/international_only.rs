#[cfg(feature = "international")]
use morse_lib::Morse;

#[test]
#[cfg(feature = "international")]
fn it_convert_sos() {
  let morse = Morse::from_text("sos").unwrap();

  assert_eq!(morse.to_string(), ". . .   ⚊ ⚊ ⚊   . . .")
}