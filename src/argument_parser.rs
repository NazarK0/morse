use std::usize;

mod arg_error;
pub use arg_error::ArgError;

mod alphabet;
pub use alphabet::Alphabet;

const DELIMETER: char = '=';

const ARG_LANG_S: &str = "-l";
const ARG_LANG_L: &str = "--language";
const ARG_OUTPUT_S: &str = "-o";
const ARG_OUTPUT_L: &str = "--output-file";
const ARG_INPUT_CMD_S: &str = "-t";
const ARG_INPUT_CMD_L: &str = "--text";
const ARG_INPUT_FILE_S: &str = "-i";
const ARG_INPUT_FILE_L: &str = "--input-file";
const ARG_BEEP_S: &str = "-b";
const ARG_BEEP_L: &str = "--beep";

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
    let delimeter_idx = arg.find(DELIMETER).unwrap_or_else(|| usize::MAX);

    // Process args without data
    if delimeter_idx == usize::MAX {
        if arg == ARG_BEEP_S || arg == ARG_BEEP_L {
            return Ok(ParsedArg {
                field: ConfigField::Beep,
                arg: arg.to_string(),
            });
        }

        if arg == ARG_LANG_S
            || arg == ARG_LANG_L
            || arg == ARG_OUTPUT_S
            || arg == ARG_OUTPUT_L
            || arg == ARG_INPUT_CMD_S
            || arg == ARG_INPUT_CMD_L
            || arg == ARG_INPUT_FILE_S
            || arg == ARG_INPUT_FILE_L
        {
            return Err(ArgError::MissingInputData(arg.to_string()));
        }

        return Err(ArgError::InvalidArg(arg.to_string()));
    }

    let (arg, data) = arg.split_at(delimeter_idx);
    let data = data.replacen(DELIMETER, "", 1);

    let field = match arg {
        ARG_LANG_S | ARG_LANG_L => {
            let lang = match data.to_ascii_lowercase().as_str() {
                "int" => Alphabet::International,
                "en" => Alphabet::English,
                "ua" => Alphabet::Ukrainian,
                other => return Err(ArgError::UnsupportedLanguage(other.to_string())),
            };

            ConfigField::Language(lang)
        }
        ARG_OUTPUT_S | ARG_OUTPUT_L => ConfigField::OutputFilePath(data.to_string()),
        ARG_INPUT_CMD_S | ARG_INPUT_CMD_L => ConfigField::Text(data.to_string()),
        ARG_INPUT_FILE_S | ARG_INPUT_FILE_L => ConfigField::InputFilePath(data.to_string()),
        ARG_BEEP_S | ARG_BEEP_L => ConfigField::Beep,
        _ => return Err(ArgError::InvalidArg(arg.to_string())),
    };

    Ok(ParsedArg {
        field,
        arg: arg.to_string(),
    })
}
