use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Alphabet {
    International,
    Ukrainian,
    English,
}

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Alphabet::International => write!(f, "International"),
            Alphabet::Ukrainian => write!(f, "Ukrainian"),
            Alphabet::English => write!(f, "English"),
        }
    }
}
