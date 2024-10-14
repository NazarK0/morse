mod morse_char;
use std::cell::RefCell;

pub use morse_char::*;

mod morse_unit;
pub use morse_unit::MorseUnit;

mod morse_processors;
pub use morse_processors::*;

use crate::argument_parser::Alphabet;

#[derive(Debug, PartialEq, Clone)]
pub struct Morse {
    morse: Vec<MorseChar>,
    language: Alphabet,
    display_as: DisplayChars,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DisplayChars {
    pub dot: String,
    pub line: String,
    pub whitespace: String,
}

impl Morse {
    pub fn from_str(text: &str, language: Alphabet) -> Morse {
        let text = text.to_ascii_lowercase();
        let mut morse: Vec<MorseChar> = Vec::new();

        for ch in text.chars() {
            morse.push(MorseChar::new(ch, language));
        }

        Morse {
            morse,
            language,
            display_as: DisplayChars {
                dot: '.'.to_string(),
                line: '⚊'.to_string(),
                whitespace: ' '.to_string(),
            },
        }
    }

    pub fn to_beep(&self) {
        for m_char in &self.morse {
            m_char.to_beep();
        }
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

    pub fn to_bin_str(&self) -> &str {
        ""
    }
}

impl ToString for Morse {
    fn to_string(&self) -> String {
        let mut string = String::new();
        let morse = RefCell::new(self.morse.clone());
        for (idx, m_char) in morse.borrow_mut().iter_mut().enumerate() {
            m_char.dot_as(&self.display_as.dot);
            m_char.line_as(&self.display_as.line);
            m_char.whitespace_as(&self.display_as.whitespace);
            string.push_str(&m_char.to_string());

            // The space between letters is three units
            if idx < self.morse.len() - 1 {
                string.push_str("   ");
            }
        }

        string
    }
}
