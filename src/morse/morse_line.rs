use std::fmt;

use super::MorseUnit;

pub struct MorseLine {}

impl fmt::Display for MorseLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-")
    }
}

impl MorseUnit for MorseLine {
    
}
