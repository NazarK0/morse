use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum ArgError {
    MissingOperands,
    MissingDelimeter(String),
    MissingInputData(String),
    InvalidArg(String),
    RedundantArg(String),
    UnsupportedLanguage(String),
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgError::MissingDelimeter(arg) => {
                write!(f, "Missing delimeter in '{arg}' arg")
            }
            ArgError::MissingOperands => write!(f, "Missing operands"),
            ArgError::MissingInputData(arg) => write!(f, "Missing input data in {arg} arg"),
            ArgError::InvalidArg(arg) => write!(f, "Invalid argument {arg}"),
            ArgError::RedundantArg(arg) => write!(f, "Redundant argument {arg}"),
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
