#[cfg(all(feature = "international", feature="audio"))]
use morse_lib::{Morse, Languages};

#[test]
#[cfg(all(feature = "international", feature="audio"))]
fn it_convert_sos() {
  let morse = Morse::from_text("sos", Languages::International).unwrap();
  morse.to_beep();

  assert_eq!(morse.to_string(), ". . .   ⚊ ⚊ ⚊   . . .")
}