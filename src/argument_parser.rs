use crate::Alphabet;

mod arg_error;
pub use arg_error::ArgError;

const DELIMETERS_IN_ARG_FLAG: [char; 2] = ['=', ' '];

pub struct ParsedArg {
    pub field: ConfigField,
    pub arg: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConfigField {
    Language(Alphabet),
    Beep,
    InputFilePath(String),
    Text(String),
    OutputFilePath(String),
}

pub fn parse_arg(arg: &str) -> Result<ParsedArg, ArgError> {
    let arg_split: Vec<&str> = arg.split(&DELIMETERS_IN_ARG_FLAG).collect();
    let arg = arg_split[0].to_owned();
    println!("{:?}", arg_split);

    let field = match arg_split[0] {
        "-l" | "--language" => {
            if arg_split.len() != 2 {
                return Err(ArgError::MissingInputData(arg));
            }

            let lang  = match arg_split[1].to_ascii_lowercase().as_str() {
                "int" => Alphabet::International,
                "en" => Alphabet::English,
                "ua" => Alphabet::Ukrainian,
                other => return Err(ArgError::UnsupportedLanguage(other.to_owned()))
            };

            ConfigField::Language(lang)
        }
        "-o" | "--output-file" => {
            if arg_split.len() != 2 {
                return Err(ArgError::MissingInputData(arg));
            }
            ConfigField::OutputFilePath(arg_split[1].to_owned())
        }
        "-t" | "--text" => {
            if arg_split.len() != 2 {
                return Err(ArgError::MissingInputData(arg));
            }
            ConfigField::Text(arg_split[1].to_owned())
        }
        "-i" | "--input-file" => {
            if arg_split.len() != 2 {
                return Err(ArgError::MissingInputData(arg.clone()));
            }

            ConfigField::InputFilePath(arg_split[1].to_owned())
        }
        "-b" | "--beep" => ConfigField::Beep,
        _ => return Err(ArgError::InvalidArg(arg)),
    };

    Ok(ParsedArg { field, arg })
}
