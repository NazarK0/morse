use crate::argument_parser::Alphabet;

// use super::MorseUnit::EOW;
use super::{convert_char, DisplayAlias, MorseDisplayUnit};

#[derive(Debug, PartialEq, Clone)]
pub struct MorseChar {
    m_char: Vec<MorseDisplayUnit>,
    alpha_char: char,
    language: Alphabet,
}

pub enum SpecialChars {
    Whitespace,
}

impl MorseChar {
    pub fn new(ch: char, language: Alphabet, display_units_as: Option<DisplayAlias>) -> MorseChar {
        let m_char: Vec<MorseDisplayUnit> = convert_char(language, ch, display_units_as);

        MorseChar {
            m_char,
            alpha_char: ch,
            language,
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
