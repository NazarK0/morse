use std::ops::Deref;

use crate::argument_parser::Alphabet;

use super::MorseUnit;

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
                m_char: vec![MorseUnit::EOW],
                alpha_char: ' ',
                language: Alphabet::International,
            },
        }
    }

    pub fn to_beep(&self) {}


    fn convert_from_int(ch: char) -> Vec<MorseUnit> {
        let mut m_char: Vec<MorseUnit> = Vec::new();

        match ch {
            'a' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
            }
            'b' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
            }
            'c' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
            }
            'd' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
            }
            'e' => {
                m_char.push(MorseUnit::Dot);
            }
            'f' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
            }
            'g' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
            }
            'h' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
            }
            'i' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
            }
            'j' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
            }
            'k' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
            }
            'l' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
            }
            'm' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
            }
            'n' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
            }
            'o' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
            }
            'p' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
            }
            'q' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
            }
            'r' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
            }
            's' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
            }
            't' => {
                m_char.push(MorseUnit::Line);
            }
            'u' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
            }
            'v' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
            }
            'w' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
            }
            'x' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
            }
            'y' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
            }
            'z' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
            }
            '1' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
            }
            '2' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
            }
            '3' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
            }
            '4' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Line);
            }
            '5' => {
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
            }
            '6' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
            }
            '7' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
            }
            '8' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
                m_char.push(MorseUnit::Dot);
            }
            '9' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Dot);
            }
            '0' => {
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
                m_char.push(MorseUnit::Line);
            }
            _ => {}
        }

        m_char
    }
}

impl Deref for MorseChar {
    type Target = Vec<MorseUnit>;

    fn deref(&self) -> &Self::Target {
        &self.m_char
    }
}

impl ToString for MorseChar {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for (idx,m_unit) in self.m_char.iter().enumerate() {
            string.push_str(&m_unit.to_string());

            // The space between parts of the same letter is one unit
            if idx < self.m_char.len() - 1 {
                string.push(' ');
            }
        }

        string
    }
}
