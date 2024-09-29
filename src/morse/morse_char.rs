use std::fmt;

use crate::argument_parser::Alphabet;

use super::MorseUnit;

pub struct MorseChar {
  morse: Vec<Box<dyn MorseUnit>>,
  alpha_char: char,
  language: Alphabet,
}

pub enum SpecialChars {
    Whitespace,
}

impl MorseChar {
    pub fn new(ch: char, language: Alphabet)-> MorseChar {
    MorseChar {
      morse: Vec::new(),
      alpha_char: ch,
      language,
    }

  }

    pub fn new_special(ch: SpecialChars, language: Alphabet)-> MorseChar {
    MorseChar {
      morse: Vec::new(),
      alpha_char: 'S',
      language,
    }
  }

    
}

impl fmt::Display for MorseChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "˙")
    }
}
