use crate::MorseUnit;
use crate::MorseUnit::{Dot, Line, Whitespace};

pub fn from_int_char(letter: char) -> Vec<MorseUnit> {
    match letter.to_ascii_lowercase() {
        'a' => vec![Dot, Line],
        'b' => vec![Line, Dot, Dot, Dot],
        'c' => vec![Line, Dot, Line, Dot],
        'd' => vec![Line, Dot, Dot],
        'e' => vec![Dot],
        'f' => vec![Dot, Dot, Line, Dot],
        'g' => vec![Line, Line, Dot],
        'h' => vec![Dot, Dot, Dot, Dot],
        'i' => vec![Dot, Dot],
        'j' => vec![Dot, Line, Line, Line],
        'k' => vec![Line, Dot, Line],
        'l' => vec![Dot, Line, Dot, Dot],
        'm' => vec![Line, Line],
        'n' => vec![Line, Dot],
        'o' => vec![Line, Line, Line],
        'p' => vec![Dot, Line, Line, Dot],
        'q' => vec![Line, Line, Dot, Line],
        'r' => vec![Dot, Line, Dot],
        's' => vec![Dot, Dot, Dot],
        't' => vec![Line],
        'u' => vec![Dot, Dot, Line],
        'v' => vec![Dot, Dot, Dot, Line],
        'w' => vec![Dot, Line, Line],
        'x' => vec![Line, Dot, Dot, Line],
        'y' => vec![Line, Dot, Line, Line],
        'z' => vec![Line, Line, Dot, Dot],
        '1' => vec![Dot, Line, Line, Line, Line],
        '2' => vec![Dot, Dot, Line, Line, Line],
        '3' => vec![Dot, Dot, Dot, Line, Line],
        '4' => vec![Dot, Dot, Dot, Dot, Line],
        '5' => vec![Dot, Dot, Dot, Dot, Dot],
        '6' => vec![Line, Dot, Dot, Dot, Dot],
        '7' => vec![Line, Line, Dot, Dot, Dot],
        '8' => vec![Line, Line, Line, Dot, Dot],
        '9' => vec![Line, Line, Line, Line, Dot],
        '0' => vec![Line, Line, Line, Line, Line],
        ' ' => vec![Whitespace],
        _ => {
            panic!("")
        }
    }
}

pub fn into_int_char(letter: Vec<MorseUnit>) -> char {
    if letter.len() == 1 {
        match letter[0] {
            Dot => return 'e',
            Line => return 't',
            Whitespace => return ' ',
        }
    } else if letter.len() == 2 {
        if letter[0] == Dot && letter[1] == Line {
            return 'a';
        } else if letter[0] == Line && letter[1] == Dot {
            return 'n';
        } else if letter[0] == Dot && letter[1] == Dot {
            return 'i';
        } else {
            // letter[0] == Line && letter[1] == Line
            return 'm';
        }
    } else if letter.len() == 3 {
        if letter[0] == Line && letter[1] == Line && letter[2] == Dot {
            return 'g';
        } else if letter[0] == Line && letter[1] == Dot && letter[2] == Dot {
            return 'd';
        } else if letter[0] == Line && letter[1] == Dot && letter[2] == Line {
            return 'k';
        } else if letter[0] == Dot && letter[1] == Line && letter[2] == Dot {
            return 'r';
        } else if letter[0] == Dot && letter[1] == Dot && letter[2] == Dot {
            return 's';
        } else if letter[0] == Dot && letter[1] == Line && letter[2] == Line {
            return 'w';
        } else if letter[0] == Dot && letter[1] == Dot && letter[2] == Line {
            return 'u';
        } else {
            // if letter[0] == Line && letter[1] == Line && letter[2] == Line
            return 'o';
        }
    } else if letter.len() == 4 {
        if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
            return 'b';
        } else if letter[0] == Line && letter[1] == Dot && letter[2] == Line && letter[3] == Dot {
            return 'c';
        } else if letter[0] == Dot && letter[1] == Dot && letter[2] == Line && letter[3] == Dot {
            return 'f';
        } else if letter[0] == Dot && letter[1] == Dot && letter[2] == Dot && letter[3] == Dot {
            return 'h';
        } else if letter[0] == Dot && letter[1] == Line && letter[2] == Line && letter[3] == Line {
            return 'j';
        } else if letter[0] == Dot && letter[1] == Line && letter[2] == Dot && letter[3] == Dot {
            return 'l';
        } else if letter[0] == Dot && letter[1] == Line && letter[2] == Line && letter[3] == Dot {
            return 'p';
        } else if letter[0] == Line && letter[1] == Line && letter[2] == Dot && letter[3] == Line {
            return 'q';
        } else if letter[0] == Dot && letter[1] == Dot && letter[2] == Dot && letter[3] == Line {
            return 'v';
        } else if letter[0] == Line && letter[1] == Dot && letter[2] == Dot && letter[3] == Line {
            return 'x';
        } else if letter[0] == Line && letter[1] == Dot && letter[2] == Line && letter[3] == Line {
            return 'y';
        } else {
            // if letter[0] == Line && letter[1] == Line && letter[2] == Dot && letter[3] == Dot
            return 'z';
        }
        //numbers
    } else if letter.len() == 5 {
        if letter[0] == Dot
            && letter[1] == Line
            && letter[2] == Line
            && letter[3] == Line
            && letter[4] == Line
        {
            return '1';
        } else if letter[0] == Dot
            && letter[1] == Dot
            && letter[2] == Line
            && letter[3] == Line
            && letter[4] == Line
        {
            return '2';
        } else if letter[0] == Dot
            && letter[1] == Dot
            && letter[2] == Dot
            && letter[3] == Line
            && letter[4] == Line
        {
            return '3';
        } else if letter[0] == Dot
            && letter[1] == Dot
            && letter[2] == Dot
            && letter[3] == Dot
            && letter[4] == Line
        {
            return '4';
        } else if letter[0] == Dot
            && letter[1] == Dot
            && letter[2] == Dot
            && letter[3] == Dot
            && letter[4] == Dot
        {
            return '5';
        } else if letter[0] == Line
            && letter[1] == Dot
            && letter[2] == Dot
            && letter[3] == Dot
            && letter[4] == Dot
        {
            return '6';
        } else if letter[0] == Line
            && letter[1] == Line
            && letter[2] == Dot
            && letter[3] == Dot
            && letter[4] == Dot
        {
            return '7';
        } else if letter[0] == Line
            && letter[1] == Line
            && letter[2] == Line
            && letter[3] == Dot
            && letter[4] == Dot
        {
            return '8';
        } else if letter[0] == Line
            && letter[1] == Line
            && letter[2] == Line
            && letter[3] == Line
            && letter[4] == Dot
        {
            return '9';
        } else {
            // if letter[0] == Line && letter[1] == Line && letter[2] == Line && letter[3] == Line && letter[4] == Line
            return '0';
        }
    } else {
        panic!("")
    }
}

pub fn convert_from_bin(letter: &str) -> Vec<MorseUnit> {
    let parts: Vec<&str> = letter.split('0').collect();
    let mut morse_letter = Vec::new();
    for unit in parts {
        match unit {
            "111" => morse_letter.push(Line),
            "1" => morse_letter.push(Dot),
            _ => panic!("Wrond binary, cant parse"),
        }
    }

    morse_letter
}
