use std::fmt;

use super::MorseUnit;

pub struct MorseDot {}

impl fmt::Display for MorseDot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "˙")
    }
}

impl MorseUnit for MorseDot {
    
}
