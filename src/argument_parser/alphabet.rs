use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Alphabet {
    International,
    Custom(String),
}

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Alphabet::International => write!(f, "International"),
            Alphabet::Custom(alphabet) => write!(f, "{alphabet}"),
        }
    }
}
