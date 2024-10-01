mod morse_char;
pub use morse_char::*;

mod morse_unit;
pub use morse_unit::MorseUnit;

use crate::argument_parser::Alphabet;

pub struct Morse {
    morse: Vec<MorseChar>,
    language: Alphabet,
}

impl Morse {
    pub fn from_str(text: &str, language: Alphabet) -> Morse {
        let mut morse: Vec<MorseChar> = Vec::new();
        let text = text.to_ascii_lowercase();
        let words: Vec<&str> = text.split_ascii_whitespace().collect();

        for word in words {
            for ch in word.chars() {
                morse.push(MorseChar::new(ch, language));
            }
            morse.push(MorseChar::new_special(SpecialChars::Whitespace));
        }

        Morse { morse, language }
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
