use crate::argument_parser::Alphabet;

mod international;
use international::convert_from_int;

mod english;
use english::convert_from_en;
mod ukrainian;
use ukrainian::convert_from_ua;

use super::{DisplayAlias, MorseDisplayUnit, MorseUnit};



pub fn convert_char(
    language: Alphabet,
    ch: char,
    display_str: Option<DisplayAlias>,
) -> Vec<MorseDisplayUnit> {
    let dot;
    let line;
    let eow;

    match display_str {
        Some(display) => {
            dot = MorseDisplayUnit::new(MorseUnit::Dot, display.dot.unwrap_or_else(|| '․'));
            line = MorseDisplayUnit::new(MorseUnit::Line, display.line.unwrap_or_else(|| '⚊'));
            eow = MorseDisplayUnit::new(MorseUnit::EOW, display.eow.unwrap_or_else(|| ' '));
        }
        None => {
            dot = MorseDisplayUnit::new(MorseUnit::Dot, '․');
            line = MorseDisplayUnit::new(MorseUnit::Line, '⚊');
            // The space between words is seven units(<three before> <char> <three after>)
            eow = MorseDisplayUnit::new(MorseUnit::EOW, ' '); 
        }
    }

    match language {
        Alphabet::International => convert_from_int(ch)
            .iter()
            .map(|m_ch| match m_ch {
                MorseUnit::Dot => dot.clone(),
                MorseUnit::Line => line.clone(),
                MorseUnit::EOW => eow.clone(),
            })
            .collect(),
        Alphabet::Ukrainian => convert_from_ua(ch)
            .iter()
            .map(|m_ch| match m_ch {
                MorseUnit::Dot => dot.clone(),
                MorseUnit::Line => line.clone(),
                MorseUnit::EOW => eow.clone(),
            })
            .collect(),
        Alphabet::English => convert_from_en(ch)
            .iter()
            .map(|m_ch| match m_ch {
                MorseUnit::Dot => dot.clone(),
                MorseUnit::Line => line.clone(),
                MorseUnit::EOW => eow.clone(),
            })
            .collect(),
    }
}
