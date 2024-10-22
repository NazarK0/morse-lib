# Morse Library

Morse Library is a library parsing text and binary data
to Morse Code and vice versa.
By default Morse Library support only International rules and codes for Morse
Code, but if needed it support extend metods to convert any language-specific
Morse Code implementations. The library provides **Lines**, **Dots** and **Whitespace**
aliasing. That means output Morse Code could be not only lines, dots and whitespaces,
but also any UTF-8 emoji or even text! Also the library support playing Morse Code by sound
if needed, and customization of speed, frequency of playing.

## Extend multimultilingualism

To provide custom language conversion the library accept two functions:
- first that match conversion from character to Morse Code
- second that match conversion from Morse Code to Character

## Data formats
The following is a list of data formats that have been implemented
for Morse Library.
### Input
- [String], the casual String or &str that contains text
- [Binary String], the casual String or &str that contains Morse Code represented by byte code.
### Output
- [String], the casual String that contains Morse Code. By default **lines** and **dots**, but could be
  any UTF-8 character or even string
- [Binary String], the casual String that contains Morse Code represented by byte code.
- [Sound], sound representation of Morse Code

### Examples

#### Basic usage (International Morse Code)
 ```
 use morse_lib::Morse;

 let morse = Morse::from_int_text("sos");

 assert_eq!(
        morse.to_string(),
        ". . .   ⚊ ⚊ ⚊   . . ."
    );

let morse = Morse::from_int_text("sos");
morse.dot_as("🔥");
morse.line_as("➖");

morse.frequency(500.0);
morse.play_speed(2.0);
morse.beep();

assert_eq!(
        morse.to_string(),
        "🔥 🔥 🔥   ➖ ➖ ➖   🔥 🔥 🔥"
    );

let morse = Morse::from_int_bin("101010001110111011100010101");

 assert_eq!(
        morse.to_string(),
        ". . .   ⚊ ⚊ ⚊   . . ."
    );

let text = morse.to_text();
assert_eq!(text,"sos");

 ```


#### Extended usage (Any language Morse Code)

```
use morse_lib::{Morse, MorseUnit};
use MorseUnit::{Dot, Line, Whitespace};

fn from_char(letter: char) -> Vec<MorseUnit> {
    match letter {
        'a' => vec![Dot, Line],
        'б' => vec![Line, Dot, Dot, Dot],
        'в' => vec![Dot, Line, Line],
        'г' => vec![Dot, Dot, Dot, Dot],
        ... and other letters from alphabet
        ' ' => vec![Whitespace],
        _ => panic!("Wrong character")
    }
}

fn into_char(letter: Vec<MorseUnit>) -> char {
    if letter.len() == 1 && letter[0] == Whitespace {
        return ' ';
    } else if letter.len() == 2 && letter[0] == Dot && letter[1] == Line {
        return 'а'
    } else if letter.len() == 3 && letter[0] == Dot && letter[1] == Line && letter[2] == Line {
        return 'в';
    } else if letter.len() == 4 {
        if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
            return 'б';
        } else {
            return 'г';
        }
    } else {
        panic!("Wrong Morse Char sequence")
    }
}

let morse = Morse::new("Ukrainian".to_string(), from_char, into_char);

morse.parse_text("Баба");
morse.dot_as("🔥");
morse.line_as("➖");
morse.beep();
```