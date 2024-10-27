use crate::error::MorseError;
use crate::MorseUnit::{Dot, Line, Whitespace};
use crate::{MorseResult, MorseUnit};

pub fn from_int_char(letter: char) -> MorseResult<Vec<MorseUnit>> {
    match letter.to_ascii_lowercase() {
        'a' => Ok(vec![Dot, Line]),
        'b' => Ok(vec![Line, Dot, Dot, Dot]),
        'c' => Ok(vec![Line, Dot, Line, Dot]),
        'd' => Ok(vec![Line, Dot, Dot]),
        'e' => Ok(vec![Dot]),
        'f' => Ok(vec![Dot, Dot, Line, Dot]),
        'g' => Ok(vec![Line, Line, Dot]),
        'h' => Ok(vec![Dot, Dot, Dot, Dot]),
        'i' => Ok(vec![Dot, Dot]),
        'j' => Ok(vec![Dot, Line, Line, Line]),
        'k' => Ok(vec![Line, Dot, Line]),
        'l' => Ok(vec![Dot, Line, Dot, Dot]),
        'm' => Ok(vec![Line, Line]),
        'n' => Ok(vec![Line, Dot]),
        'o' => Ok(vec![Line, Line, Line]),
        'p' => Ok(vec![Dot, Line, Line, Dot]),
        'q' => Ok(vec![Line, Line, Dot, Line]),
        'r' => Ok(vec![Dot, Line, Dot]),
        's' => Ok(vec![Dot, Dot, Dot]),
        't' => Ok(vec![Line]),
        'u' => Ok(vec![Dot, Dot, Line]),
        'v' => Ok(vec![Dot, Dot, Dot, Line]),
        'w' => Ok(vec![Dot, Line, Line]),
        'x' => Ok(vec![Line, Dot, Dot, Line]),
        'y' => Ok(vec![Line, Dot, Line, Line]),
        'z' => Ok(vec![Line, Line, Dot, Dot]),
        '1' => Ok(vec![Dot, Line, Line, Line, Line]),
        '2' => Ok(vec![Dot, Dot, Line, Line, Line]),
        '3' => Ok(vec![Dot, Dot, Dot, Line, Line]),
        '4' => Ok(vec![Dot, Dot, Dot, Dot, Line]),
        '5' => Ok(vec![Dot, Dot, Dot, Dot, Dot]),
        '6' => Ok(vec![Line, Dot, Dot, Dot, Dot]),
        '7' => Ok(vec![Line, Line, Dot, Dot, Dot]),
        '8' => Ok(vec![Line, Line, Line, Dot, Dot]),
        '9' => Ok(vec![Line, Line, Line, Line, Dot]),
        '0' => Ok(vec![Line, Line, Line, Line, Line]),
        ' ' => Ok(vec![Whitespace]),
        _ => Err(MorseError::InvalidChar),
    }
}

pub fn into_int_char(letter: Vec<MorseUnit>) -> MorseResult<char> {
    if letter.len() == 1 {
        match letter[0] {
            Dot => return Ok('e'),
            Line => return Ok('t'),
            Whitespace => return Ok(' '),
        }
    } else if letter.len() == 2 {
        if letter[0] == Dot && letter[1] == Line {
            return Ok('a');
        } else if letter[0] == Line && letter[1] == Dot {
            return Ok('n');
        } else if letter[0] == Dot && letter[1] == Dot {
            return Ok('i');
        } else {
            // letter[0] == Line && letter[1] == Line
            return Ok('m');
        }
    } else if letter.len() == 3 {
        if letter[0] == Line && letter[1] == Line && letter[2] == Dot {
            return Ok('g');
        } else if letter[0] == Line && letter[1] == Dot && letter[2] == Dot {
            return Ok('d');
        } else if letter[0] == Line && letter[1] == Dot && letter[2] == Line {
            return Ok('k');
        } else if letter[0] == Dot && letter[1] == Line && letter[2] == Dot {
            return Ok('r');
        } else if letter[0] == Dot && letter[1] == Dot && letter[2] == Dot {
            return Ok('s');
        } else if letter[0] == Dot && letter[1] == Line && letter[2] == Line {
            return Ok('w');
        } else if letter[0] == Dot && letter[1] == Dot && letter[2] == Line {
            return Ok('u');
        } else {
            // if letter[0] == Line && letter[1] == Line && letter[2] == Line
            return Ok('o');
        }
    } else if letter.len() == 4 {
        if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
            return Ok('b');
        } else if letter[0] == Line && letter[1] == Dot && letter[2] == Line && letter[3] == Dot {
            return Ok('c');
        } else if letter[0] == Dot && letter[1] == Dot && letter[2] == Line && letter[3] == Dot {
            return Ok('f');
        } else if letter[0] == Dot && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
            return Ok('h');
        } else if letter[0] == Dot && letter[1] == Line && letter[2] == Line && letter[3] == Line {
            return Ok('j');
        } else if letter[0] == Dot && letter[1] == Line && letter[2] == Dot && letter[3] == Dot {
            return Ok('l');
        } else if letter[0] == Dot && letter[1] == Line && letter[2] == Line && letter[3] == Dot {
            return Ok('p');
        } else if letter[0] == Line && letter[1] == Line && letter[2] == Dot && letter[3] == Line {
            return Ok('q');
        } else if letter[0] == Dot && letter[1] == Dot && letter[2] == Dot && letter[3] == Line {
            return Ok('v');
        } else if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Line {
            return Ok('x');
        } else if letter[0] == Line && letter[1] == Dot && letter[2] == Line && letter[3] == Line {
            return Ok('y');
        } else {
            // if letter[0] == Line && letter[1] == Line && letter[2] == Dot && letter[3] == Dot
            return Ok('z');
        }
        //numbers
    } else if letter.len() == 5 {
        if letter[0] == Dot
            && letter[1] == Line
            && letter[2] == Line
            && letter[3] == Line
            && letter[4] == Line
        {
            return Ok('1');
        } else if letter[0] == Dot
            && letter[1] == Dot
            && letter[2] == Line
            && letter[3] == Line
            && letter[4] == Line
        {
            return Ok('2');
        } else if letter[0] == Dot
            && letter[1] == Dot
            && letter[2] == Dot
            && letter[3] == Line
            && letter[4] == Line
        {
            return Ok('3');
        } else if letter[0] == Dot
            && letter[1] == Dot
            && letter[2] == Dot
            && letter[3] == Dot
            && letter[4] == Line
        {
            return Ok('4');
        } else if letter[0] == Dot
            && letter[1] == Dot
            && letter[2] == Dot
            && letter[3] == Dot
            && letter[4] == Dot
        {
            return Ok('5');
        } else if letter[0] == Line
            && letter[1] == Dot
            && letter[2] == Dot
            && letter[3] == Dot
            && letter[4] == Dot
        {
            return Ok('6');
        } else if letter[0] == Line
            && letter[1] == Line
            && letter[2] == Dot
            && letter[3] == Dot
            && letter[4] == Dot
        {
            return Ok('7');
        } else if letter[0] == Line
            && letter[1] == Line
            && letter[2] == Line
            && letter[3] == Dot
            && letter[4] == Dot
        {
            return Ok('8');
        } else if letter[0] == Line
            && letter[1] == Line
            && letter[2] == Line
            && letter[3] == Line
            && letter[4] == Dot
        {
            return Ok('9');
        } else {
            // if letter[0] == Line && letter[1] == Line && letter[2] == Line && letter[3] == Line && letter[4] == Line
            return Ok('0');
        }
    } else {
        Err(MorseError::InvalidMorseSequence)
    }
}

pub fn convert_from_bin(letter: &str) -> Result<Vec<MorseUnit>, MorseError> {
    let parts: Vec<&str> = letter.split('0').collect();
    let mut morse_letter = Vec::new();
    for unit in parts {
        match unit {
            "111" => morse_letter.push(Line),
            "1" => morse_letter.push(Dot),
            _ => return Err(MorseError::InvalidBinary),
        }
    }

    Ok(morse_letter)
}
