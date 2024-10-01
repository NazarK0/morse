use crate::argument_parser::Alphabet;

use super::MorseUnit;
use super::MorseUnit::{Dot, Line, EOW};

pub struct MorseChar {
    m_char: Vec<MorseUnit>,
    alpha_char: char,
    language: Alphabet,
}

pub enum SpecialChars {
    Whitespace,
}

impl MorseChar {
    pub fn new(ch: char, language: Alphabet) -> MorseChar {
        let mut m_char: Vec<MorseUnit> = Vec::new();
        match language {
            Alphabet::International => {
                m_char = Self::convert_from_int(ch);
            }
            Alphabet::Ukrainian => {}
            Alphabet::English => {}
        };

        MorseChar {
            m_char,
            alpha_char: ch,
            language,
        }
    }

    pub fn new_special(ch: SpecialChars) -> MorseChar {
        match ch {
            SpecialChars::Whitespace => MorseChar {
                m_char: vec![EOW],
                alpha_char: ' ',
                language: Alphabet::International,
            },
        }
    }

    pub fn to_beep(&self) {
        todo!()
    }

    pub fn to_alpha(&self) -> char {
        self.alpha_char
    }

    pub fn get_language(&self) -> Alphabet {
        self.language
    }

    fn convert_from_int(ch: char) -> Vec<MorseUnit> {
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
}

impl ToString for MorseChar {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for (idx, m_unit) in self.m_char.iter().enumerate() {
            string.push_str(&m_unit.to_string());

            // The space between parts of the same letter is one unit
            if idx < self.m_char.len() - 1 {
                string.push(' ');
            }
        }

        string
    }
}
