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
            sound: Sound {
                frequency: 450.0,
                speed: 1.0
            }
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
