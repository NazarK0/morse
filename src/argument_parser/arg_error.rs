use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum ArgError {
    MissingOperands,
    MissingInputData(String),
    InvalidArg(String),
    MultipleInputs,
    UnsupportedLanguage(String),
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgError::MissingOperands => write!(f, "Missing operands"),
            ArgError::MissingInputData(arg) => write!(f, "Missing input data in {arg} arg"),
            ArgError::InvalidArg(arg) => write!(f, "Invalid argument {arg}"),
            ArgError::MultipleInputs => write!(f, "Multiple inputs "),
            ArgError::UnsupportedLanguage(lang) => write!(f, "Unsupported language {lang}"),
        }
    }
}

impl Error for ArgError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
