use crate::argument_parser::Alphabet;

mod international;
use international::convert_from_int;

mod english;
use english::convert_from_en;
mod ukrainian;
use ukrainian::convert_from_ua;

use super::MorseUnit;

pub fn convert_char(language: Alphabet, ch: char) -> Vec<MorseUnit> {
    match language {
        Alphabet::International => convert_from_int(ch),
        Alphabet::Ukrainian => convert_from_ua(ch),
        Alphabet::English => convert_from_en(ch),
    }
}
