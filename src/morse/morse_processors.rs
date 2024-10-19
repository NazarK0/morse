use crate::argument_parser::Alphabet;

mod international;
use international::{convert_from_int, convert_from_int_bin};

mod english;
use english::{convert_from_en, convert_from_en_bin};
mod ukrainian;
use ukrainian::{convert_from_ua, convert_from_ua_bin};

use super::MorseUnit;

pub fn convert_char(language: &Alphabet, letter: char) -> Vec<MorseUnit> {
    let letter = letter.to_ascii_lowercase();
    match language {
        Alphabet::International => convert_from_int(letter),
        Alphabet::Ukrainian => convert_from_ua(letter),
        Alphabet::English => convert_from_en(letter),
        Alphabet::Custom(_) => todo!(),
    }
}

pub fn convert_bin(language: &Alphabet, letter: &str) -> Vec<MorseUnit> {
    match language {
        Alphabet::International => convert_from_int_bin(letter),
        Alphabet::Ukrainian => convert_from_ua_bin(letter),
        Alphabet::English => convert_from_en_bin(letter),
        Alphabet::Custom(_) => todo!(),
    }
}
