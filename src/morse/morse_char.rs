use crate::argument_parser::Alphabet;

// use super::MorseUnit::Whitespace;
use super::{convert_char, DisplayChars, MorseUnit};

#[derive(Debug, PartialEq, Clone)]
pub struct MorseChar {
    m_char: Vec<MorseUnit>,
    alpha_char: char,
    language: Alphabet,
    display_as: DisplayChars,
}

pub enum SpecialChars {
    Whitespace,
}

impl MorseChar {
    pub fn new(ch: char, language: Alphabet) -> MorseChar {
        let m_char: Vec<MorseUnit> = convert_char(language, ch);

        MorseChar {
            m_char,
            alpha_char: ch,
            language,
            // default values that never use
            display_as: DisplayChars {
                dot: '.'.to_string(),
                line: '⚊'.to_string(),
                whitespace: ' '.to_string(),
            },
        }
    }

    pub fn to_beep(&self) {
        todo!()
    }

    pub fn to_alpha(&self) -> char {
        self.alpha_char
    }

    pub fn to_bin_str(&self) -> String {
        let mut string = String::new();
        for (idx, m_unit) in self.m_char.iter().enumerate() {
            match m_unit {
                MorseUnit::Dot => string.push_str("1"),
                MorseUnit::Line => string.push_str("111"),
                MorseUnit::Whitespace => string.push_str("0"),
            }


            // The space between parts of the same letter is one unit
            if idx < self.m_char.len() - 1 {
                string.push('0');
            }
        }

        string
    }

    pub fn get_language(&self) -> Alphabet {
        self.language
    }

    pub fn dot_as(&mut self, alias: &str) {
        self.display_as.dot = alias.to_string();
    }

    pub fn line_as(&mut self, alias: &str) {
        self.display_as.line = alias.to_string();
    }

    pub fn whitespace_as(&mut self, alias: &str) {
        self.display_as.whitespace = alias.to_string();
    }
}

impl ToString for MorseChar {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for (idx, m_unit) in self.m_char.iter().enumerate() {
            match m_unit {
                MorseUnit::Dot => string.push_str(&self.display_as.dot),
                MorseUnit::Line => string.push_str(&self.display_as.line),
                MorseUnit::Whitespace => string.push_str(&self.display_as.whitespace),
            }

            // println!("dot len:{}", self.display_as.dot.len());
            // println!("line len:{}", self.display_as.line.len());
            // println!("whitespace len:{}", self.display_as.whitespace.len());

            // The space between parts of the same letter is one unit
            if idx < self.m_char.len() - 1 {
                string.push(' ');
            }
        }

        string
    }
}
