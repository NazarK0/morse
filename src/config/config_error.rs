use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum ConfigError {
    MissingOperands,
    MissingDelimeterFlag(String),
    MissingInputData,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingDelimeterFlag(flag) => {
                write!(f, "Missing delimeter in flag '{flag}'")
            }
            ConfigError::MissingOperands => write!(f, "Missing operands"),
            ConfigError::MissingInputData => write!(f, "Missing input data"),
        }
    }
}

impl Error for ConfigError {
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
