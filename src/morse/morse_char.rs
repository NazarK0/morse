use std::{
    thread,
    time::{self, Duration},
};

use rodio::{source::SineWave, OutputStream, Sink, Source};

use crate::argument_parser::Alphabet;

// use super::MorseUnit::Whitespace;
use super::{convert_bin, convert_char, DisplayChars, MorseUnit, Sound};



#[derive(Debug, PartialEq, Clone)]
pub struct MorseChar {
    m_char: Vec<MorseUnit>,
    alpha_char: char,
    language: Alphabet,
    display_as: DisplayChars,
    sound: Sound,
}

pub enum SpecialChars {
    Whitespace,
}

impl MorseChar {
    pub fn from_char(letter: char, language: &Alphabet) -> MorseChar {
        let m_char: Vec<MorseUnit> = convert_char(&language, letter);

        MorseChar {
            m_char,
            alpha_char: letter,
            language: language.clone(),
            // default values that never use
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

    pub fn from_bin(letter: &str, language: &Alphabet) -> MorseChar {
        let m_char: Vec<MorseUnit> = convert_bin(language, letter);

        MorseChar {
            m_char,
            alpha_char: 'E',
            language: language.clone(),
            // default values that never use
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
        for (idx, m_unit) in self.m_char.iter().enumerate() {
            let _ = match m_unit {
                MorseUnit::Dot => {
                    play_sound(self.sound.frequency, 1, self.sound.speed);
                }
                MorseUnit::Line => {
                    play_sound(self.sound.frequency, 3, self.sound.speed);
                }
                MorseUnit::Whitespace => {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            };

            // The space between parts of the same letter is one unit
            if idx < self.m_char.len() - 1 {
                thread::sleep(time::Duration::from_secs(1));
            }
        }
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


fn play_sound( freq: f32, duration: u8, speed: f32) {
        // on linux require pkg-config libudev-dev libasound2-dev
        // _stream must live as long as the sink
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Add a dummy source of the sake of the example.
        let source = SineWave::new(freq)
            .take_duration(Duration::from_secs_f32((duration as f32) / speed))
            .amplify(0.20);
        sink.append(source);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();

    }

    #[cfg(test)]
mod morse_char_tests {
    use super::*;

    #[test]
    fn create_from_text_str() {
        assert_eq!(
            MorseChar::from_char('H', &Alphabet::International).to_bin_str(),
            "1010101"
        );
    }

    #[test]
    fn create_from_binary_str() {
        const H_BIN: &str = "1010101";
        assert_eq!(
            MorseChar::from_bin(H_BIN, &Alphabet::International).to_bin_str(),
            H_BIN
        );
    }

    #[test]
    fn get_language() {
        assert_eq!(
            MorseChar::from_char('R', &Alphabet::International).get_language(),
            Alphabet::International
        );
        assert_eq!(
            MorseChar::from_bin("1", &Alphabet::International).get_language(),
            Alphabet::International
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            MorseChar::from_char('u', &Alphabet::International).to_string(),
            ". . ⚊"
        );
    }

    #[test]
    fn to_bin_str() {
        assert_eq!(
            MorseChar::from_char('u', &Alphabet::International).to_bin_str(),
            "1010111"
        );
    }
    #[test]
    fn set_aliases_for_whitespace_lines_and_dots() {
        let mut morse = MorseChar::from_char('u', &Alphabet::International);

        morse.dot_as("🔥");
        morse.line_as("➖");

        assert_eq!(
            morse.to_string(),
            "🔥 🔥 ➖"
        );

        let mut morse = MorseChar::from_char(' ', &Alphabet::International);

        morse.whitespace_as("🚧");

        assert_eq!(
            morse.to_string(),
            "🚧"
        );
        
    }
}
