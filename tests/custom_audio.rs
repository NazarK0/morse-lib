#[cfg(feature = "custom")]
use morse_lib::{TMorse, MorseError, MorseResult, MorseUnit};
#[cfg(feature = "custom")]
use MorseUnit::{Dot, Line, Whitespace};

#[cfg(feature = "custom")]
use morse_lib::MorseCustom;

#[test]
#[cfg(feature = "custom")]
fn it_convert_ua() {
    fn from_char(letter: char) -> MorseResult<Vec<MorseUnit>> {
        match letter {
            'а' | 'А' => Ok(vec![Dot, Line]),
            'б' | 'Б' => Ok(vec![Line, Dot, Dot, Dot]),
            'в' | 'В' => Ok(vec![Dot, Line, Line]),
            'г' | 'Г' => Ok(vec![Dot, Dot, Dot, Dot]),
            ' ' => Ok(vec![Whitespace]),
            _ => Err(MorseError::InvalidChar),
        }
    }

    fn into_char(letter: Vec<MorseUnit>) -> MorseResult<char> {
        if letter.len() == 1 && letter[0] == Whitespace {
            return Ok(' ');
        } else if letter.len() == 2 && letter[0] == Dot && letter[1] == Line {
            return Ok('а');
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

    let mut morse = MorseCustom::new(from_char, into_char);

    morse.parse_text("ба").unwrap();
    
    assert_eq!(morse.to_string(), "⚊ . . .   . ⚊")
}
