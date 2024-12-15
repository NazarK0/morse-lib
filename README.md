Morse Library is a library parsing text and binary data
to Morse Code and vice versa.
Morse Library support International rules and codes for Morse
Code, but if needed it support custom language(s) to convert any language-specific
Morse Code implementations. The library provides **Lines**, **Dots** and **Whitespace**
aliasing. That means output Morse Code could be not only lines, dots and whitespaces,
but also any UTF-8 emoji or even text! Also the library support playing Morse Code by sound
if needed, and customization of speed, frequency of playing.
## Extend multimultilingualism
To provide custom language conversion the library has struct MorseCustom. The constructor accept two functions:
- first that match conversion from character to Morse Code
- second that match conversion from Morse Code to Character

## Features
* international (default)
* custom
* audio
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

## Use

This is some examples as a dependency in Cargo.toml of enabling/disabling features in library:
```toml
// for international Morse code, no audio:
morse-lib = "0.4.1"
// for all features:
morse-lib = { version = "0.4.1", features = ["custom", "audio"] } 
// for custom Morse code only:
morse-lib = { version = "0.4.1", default-features = false, features = ["custom"] } 
// for custom Morse code only and audio features:
morse-lib = { version = "0.4.1", default-features = false, features = ["custom", "audio"] } 
```





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

#### International Morse Code usage (default)
(With enabled "audio" feature)
 ```
 use morse_lib::Morse;

 let morse = Morse::from_text("sos").unwrap();

 assert_eq!(
        morse.to_string(),
        ". . .   ⚊ ⚊ ⚊   . . ."
    );

let morse = Morse::from_text("sos").unwrap();
morse.dot_as("🔥");
morse.line_as("➖");

// if enabled "audio" feature"
morse.frequency(500.0);
morse.play_speed(2.0);
morse.beep();

assert_eq!(
        morse.to_string(),
        "🔥 🔥 🔥   ➖ ➖ ➖   🔥 🔥 🔥"
    );

let morse = Morse::from_bin("101010001110111011100010101").unwrap();

 assert_eq!(
        morse.to_string(),
        ". . .   ⚊ ⚊ ⚊   . . ."
    );

let text = morse.to_text();
assert_eq!(text,"sos");

 ```


#### Custom language Morse Code usage

```
use morse_lib::{MorseCustom, MorseUnit, MorseResult, MorseError, TMorse};
use MorseUnit::{Dot, Line, Whitespace};

fn from_char(letter: char) -> MorseResult<Vec<MorseUnit>>{
    match letter {
        'а' | 'А' => Ok(vec![Dot, Line]),
        'б' | 'Б' => Ok(vec![Line, Dot, Dot, Dot]),
        'в' | 'В' => Ok(vec![Dot, Line, Line]),
        'г' | 'Г' => Ok(vec![Dot, Dot, Dot, Dot]),
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

let morse = MorseCustom::new(from_char, into_char);

morse.parse_text("Баба").unwrap();
morse.dot_as("🔥");
morse.line_as("➖");

// if enabled "audio" feature"
morse.beep();
```