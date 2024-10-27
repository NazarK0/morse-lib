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

 let morse = Morse::from_int_text("sos").unwrap();

 assert_eq!(
        morse.to_string(),
        ". . .   ⚊ ⚊ ⚊   . . ."
    );

let morse = Morse::from_int_text("sos").unwrap();
morse.dot_as("🔥");
morse.line_as("➖");

morse.frequency(500.0);
morse.play_speed(2.0);
morse.beep();

assert_eq!(
        morse.to_string(),
        "🔥 🔥 🔥   ➖ ➖ ➖   🔥 🔥 🔥"
    );

let morse = Morse::from_int_bin("101010001110111011100010101").unwrap();

 assert_eq!(
        morse.to_string(),
        ". . .   ⚊ ⚊ ⚊   . . ."
    );

let text = morse.to_text();
assert_eq!(text,"sos");

 ```


#### Extended usage (Any language Morse Code)

```
use morse_lib::{Morse, MorseUnit, MorseResult, MorseError};
use MorseUnit::{Dot, Line, Whitespace};

fn from_char(letter: char) -> MorseResult<Vec<MorseUnit>> {
    match letter {
        'a' => Ok(vec![Dot, Line]),
        'б' => Ok(vec![Line, Dot, Dot, Dot]),
        'в' => Ok(vec![Dot, Line, Line]),
        'г' => Ok(vec![Dot, Dot, Dot, Dot]),
        ... and other letters from alphabet
        ' ' => Ok(vec![Whitespace]),
        _ => Err(MorseError::InvalidChar)
    }
}

fn into_char(letter: Vec<MorseUnit>) -> MorseResult<char> {
    if letter.len() == 1 && letter[0] == Whitespace {
        return Ok(' ');
    } else if letter.len() == 2 && letter[0] == Dot && letter[1] == Line {
        return Ok('а')
    } else if letter.len() == 3 && letter[0] == Dot && letter[1] == Line && letter[2] == Line {
        return Ok('в');
    } else if letter.len() == 4 {
        if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
            return Ok('б');
        } else {
            return Ok('г');
        }
    } else {
        Err(MorseError::InvalidMorseSequence)
    }
}

let morse = Morse::new("Ukrainian".to_string(), from_char, into_char);

morse.parse_text("Баба").unwrap();
morse.dot_as("🔥");
morse.line_as("➖");
morse.beep();
```