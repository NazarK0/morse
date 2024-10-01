mod morse_char;
pub use morse_char::*;

mod morse_unit;
pub use morse_unit::{MorseDisplayUnit, MorseUnit, DisplayAlias};

mod morse_processors;
pub use morse_processors::*;

use crate::argument_parser::Alphabet;

#[derive(Debug, PartialEq, Clone)]
pub struct Morse {
    morse: Vec<MorseChar>,
    language: Alphabet,
    display_units_as: Option<DisplayAlias>,
}

impl Morse {
    pub fn new(language: Alphabet, display_units_as: Option<DisplayAlias>) -> Morse {
        Morse {
            morse: Vec::new(),
            language,
            display_units_as,
        }
    }
    pub fn from_str(
        text: &str,
        language: Alphabet,
        display_units_as: Option<DisplayAlias>,
    ) -> Morse {
        let text = text.to_ascii_lowercase();
        let mut morse: Vec<MorseChar> = Vec::new();

        for ch in text.chars() {
            morse.push(MorseChar::new(ch, language, display_units_as));
        }

        Morse {
            morse,
            language,
            display_units_as,
        }
    }

    pub fn to_beep(&self) {
        for m_char in &self.morse {
            m_char.to_beep();
        }
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for (idx, m_char) in self.morse.iter().enumerate() {
            string.push_str(&m_char.to_string());

            // The space between letters is three units
            if idx < self.morse.len() - 1 {
                string.push_str("   ");
            }
        }

        string
    }

    pub fn get_language(&self) -> Alphabet {
        self.language
    }
}
