use std::usize;

use crate::Alphabet;

mod arg_error;
pub use arg_error::ArgError;

const DELIMETER: char = '=';

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
    println!("arg func: {arg}");
    // let arg_split: Vec<&str> = arg.split(&DELIMETER).collect();
    let delimeter_idx = arg
        .find(DELIMETER)
        .unwrap_or_else(|| usize::MAX);

    println!("delim_idx: {delimeter_idx}");

    // Process args without data
    if delimeter_idx == usize::MAX {
        if arg == "-b" || arg == "--beep" {
            return Ok(ParsedArg {
                field: ConfigField::Beep,
                arg: arg.to_string(),
            });
        }

        if arg == "-l"
            || arg == "--language"
            || arg == "-o"
            || arg == "--output-file"
            || arg == "-t"
            || arg == "--text"
            || arg == "-i"
            || arg == "--input-file"
        {
            return Err(ArgError::MissingInputData(arg.to_string()));
        }

        return Err(ArgError::InvalidArg(arg.to_string()));
    }

    let (arg, data) = arg.split_at(delimeter_idx);
    let data = data.replacen(DELIMETER, "", 1);

    println!("argument: {arg} data: {}", data.len());

    let field = match arg {
        "-l" | "--language" => {
            let lang = match data.to_ascii_lowercase().as_str() {
                "int" => Alphabet::International,
                "en" => Alphabet::English,
                "ua" => Alphabet::Ukrainian,
                other => return Err(ArgError::UnsupportedLanguage(other.to_string())),
            };

            ConfigField::Language(lang)
        }
        "-o" | "--output-file" => ConfigField::OutputFilePath(data.to_string()),
        "-t" | "--text" => ConfigField::Text(data.to_string()),
        "-i" | "--input-file" => ConfigField::InputFilePath(data.to_string()),
        "-b" | "--beep" => ConfigField::Beep,
        _ => return Err(ArgError::InvalidArg(arg.to_string())),
    };

    Ok(ParsedArg {
        field,
        arg: arg.to_string(),
    })
}
