mod morse_char;
use std::{cell::RefCell, thread, time};

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
    sound: Sound,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DisplayChars {
    pub dot: String,
    pub line: String,
    pub whitespace: String,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Sound {
    pub frequency: f32,
    pub speed: f32,
}

impl Morse {
    pub fn from_str(text: &str, language: &Alphabet) -> Morse {
        let mut morse: Vec<MorseChar> = Vec::new();

        for letter in text.chars() {
            morse.push(MorseChar::from_char(letter, &language));
        }

        Morse {
            morse,
            language: language.clone(),
            display_as: DisplayChars {
                dot: '.'.to_string(),
                line: '⚊'.to_string(),
                whitespace: ' '.to_string(),
            },
            sound: Sound {
                frequency: 450.0,
                speed: 1.0,
            },
        }
    }

    pub fn from_bin(bin: &str, language: &Alphabet) -> Morse {
        let words: Vec<&str> = bin.split("0000000").collect();
        let mut morse: Vec<MorseChar> = Vec::new();

        for word in words {
            let letters: Vec<&str> = word.split("000").collect();

            for letter in letters {
                morse.push(MorseChar::from_bin(letter, &language));
            }
        }

        Morse {
            morse,
            language: language.clone(),
            display_as: DisplayChars {
                dot: '.'.to_string(),
                line: '⚊'.to_string(),
                whitespace: ' '.to_string(),
            },
            sound: Sound {
                frequency: 450.0,
                speed: 1.0,
            },
        }
    }

    pub fn to_beep(&self) {
        let morse = RefCell::new(self.morse.clone());
        for (idx, m_char) in morse.borrow_mut().iter_mut().enumerate() {
            m_char.frequency(self.sound.frequency);
            m_char.play_speed(self.sound.speed);

            m_char.to_beep();

            // The space between letters is three units
            if idx < self.morse.len() - 1 {
                thread::sleep(time::Duration::from_secs(3));
            }
        }
    }

    pub fn get_language(&self) -> Alphabet {
        self.language.clone()
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

    pub fn frequency(&mut self, frequency: f32) {
        self.sound.frequency = frequency;
    }
    pub fn play_speed(&mut self, speed: f32) {
        self.sound.speed = speed;
    }

    pub fn to_bin_str(&self) -> String {
        let mut string = String::new();

        for (idx, m_char) in self.morse.iter().enumerate() {
            string.push_str(&m_char.to_bin_str());

            // The space between letters is three units
            if idx < self.morse.len() - 1 {
                string.push_str("000");
            }
        }

        string
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

#[cfg(test)]
mod morse_tests {
    use super::*;

    #[test]
    fn create_from_text_str() {
        assert_eq!(
            Morse::from_str("Hello", &Alphabet::International).to_bin_str(),
            "1010101000100010111010100010111010100011101110111"
        );
    }

    #[test]
    fn create_from_binary_str() {
        const HELLO_BIN: &str = "1010101000100010111010100010111010100011101110111";
        assert_eq!(
            Morse::from_bin(HELLO_BIN, &Alphabet::International).to_bin_str(),
            HELLO_BIN
        );
    }

    #[test]
    fn get_language() {
        assert_eq!(
            Morse::from_str("hello", &Alphabet::International).get_language(),
            Alphabet::International
        );
        assert_eq!(
            Morse::from_bin("1", &Alphabet::International).get_language(),
            Alphabet::International
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            Morse::from_str("hi u", &Alphabet::International).to_string(),
            ". . . .   . .       . . ⚊"
        );
    }

    #[test]
    fn to_bin_str() {
        assert_eq!(
            Morse::from_str("hi u", &Alphabet::International).to_bin_str(),
            "101010100010100000001010111"
        );
    }
    #[test]
    fn set_aliases_for_whitespace_lines_and_dots() {
        let mut morse = Morse::from_str("hi u", &Alphabet::International);

        morse.dot_as("🔥");
        morse.line_as("➖");
        morse.whitespace_as("🚧");

        assert_eq!(
            morse.to_string(),
            "🔥 🔥 🔥 🔥   🔥 🔥   🚧   🔥 🔥 ➖"
        );
    }
}
