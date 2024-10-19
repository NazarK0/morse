use crate::MorseUnit;
use crate::MorseUnit::{Dot, Line};

pub fn convert_from_en(ch: char) -> Vec<MorseUnit> {
    match ch {
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
        _ => {
            panic!("")
        }
    }
}

pub fn convert_from_en_bin(letter: &str) -> Vec<MorseUnit> {
    vec![Dot]
}