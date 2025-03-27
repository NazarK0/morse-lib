#[cfg(feature = "international")]
use morse_lib::Morse;
use morse_lib::Languages;

#[test]
#[cfg(feature = "international")]
fn it_convert_sos() {
  let morse = Morse::from_text("sos", Languages::International).unwrap();

  assert_eq!(morse.to_string(), ". . .   ⚊ ⚊ ⚊   . . .")
}