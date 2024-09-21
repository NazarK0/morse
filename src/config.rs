mod config_error;
use std::{collections::HashSet, process};

pub use config_error::ConfigError;

const DELIMETERS_IN_ARG_FLAG: [char; 2] = ['=', ' '];

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Alphabet {
    International,
    Ukrainian,
    English,
}

pub struct Config {
    language: Alphabet,
    beep: bool,
    input_file_path: Option<String>,
    text: Option<String>,
    output_file_path: Option<String>,
}

struct ParsedArg {
    field: ConfigField,
    flag: String,
    data: Option<String>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ConfigField {
    Language,
    Beep,
    InputFilePath,
    Text,
    OutputFilePath,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, ConfigError> {
        let mut input_set = false;
        let text: Option<String> = None;

        let valid_flags = HashSet::from([
            "-l",
            "--language",
            "-o",
            "--output-file",
            "-t",
            "--text",
            "-i",
            "--input-file",
            "-b",
            "--beep",
        ]);

        if args.len() == 1 {
            return Err(ConfigError::MissingOperands);
        }

        for arg in &args[1..] {
            let parsed_arg = Self::extract_data_from_arg(arg);

            match parsed_arg {
                Some(parsed) => {
                    println!("Valid flag {}", parsed.flag);
                }
                None => {
                    println!("Invalid flag");
                }
            }

            if arg.starts_with("-l") || arg.starts_with("--language") {
                println!("You specify language")
            }

            if arg.starts_with("-o") || arg.starts_with("--output-file") {
                println!("You specify output to file")
            }

            if arg.starts_with("-i") || arg.starts_with("--input-file") {
                if input_set {
                    println!(
                        "Redundant operand. You specify input from file and from command line."
                    )
                } else {
                    input_set = true;

                    //  if arg.starts_with("-i") {
                    //     flag = "-i";
                    // } else {
                    //     flag = "--input-file";
                    // }
                }
            }

            if arg.starts_with("-b") || arg.starts_with("--beep") {
                println!("You specify emit Morse code beep")
            }

            if arg.starts_with("-t") || arg.starts_with("--translate") {
                if input_set {
                    println!(
                        "Redundant operand. You specify input from file and from command line."
                    )
                } else {
                    input_set = true;
                    let flag;

                    if arg.starts_with("-t") {
                        flag = "-t";
                    } else {
                        flag = "--translate";
                    }

                    if !arg.contains("=") {
                        return Err(ConfigError::MissingDelimeterFlag(String::from(flag)));
                    }

                    Self::extract_data_from_arg(arg);
                    // println!("You specify input from command line: {text}")
                }
            }
        }

        if !input_set {
            return Err(ConfigError::MissingInputData);
        }

        Ok(Config {
            language: Alphabet::International,
            beep: true,
            input_file_path: None,
            output_file_path: None,
            text,
        })
    }

    // getters
    pub fn get_lang(&self) -> Alphabet {
        self.language
    }

    pub fn get_beep(&self) -> bool {
        self.beep
    }

    pub fn get_input_file_path(&self) -> Option<String> {
        self.input_file_path.clone()
    }

    pub fn get_output_file_path(&self) -> Option<String> {
        self.output_file_path.clone()
    }

    pub fn get_text(&self) -> Option<String> {
        self.text.clone()
    }

    // private methods
    fn extract_data_from_arg(arg: &str) -> Option<ParsedArg> {
        let arg_split: Vec<&str> = arg.split(&DELIMETERS_IN_ARG_FLAG).collect();
        // println!("{:?}", arg_split);

        let field = match arg_split[0] {
            "-l" | "--text" => Some(ConfigField::Text),
            "-o" | "--output-file" => Some(ConfigField::OutputFilePath),
            "-t" | "--text" => Some(ConfigField::Text),
            "-i" | "--input-file" => Some(ConfigField::InputFilePath),
            "-b" | "--beep" => Some(ConfigField::Beep),
            _ => None,
        };

        match field {
            Some(_field) => {
                if arg_split.len() == 2 {
                    return Some(ParsedArg {
                        field: _field,
                        flag: arg_split[0].to_owned(),
                        data: Some(arg_split[1].to_owned()),
                    });
                } else {
                    return Some(ParsedArg {
                        field: _field,
                        flag: arg_split[0].to_owned(),
                        data: None,
                    });
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    struct ExpectConfig {
        language: Alphabet,
        beep: bool,
        input_file_path: Option<String>,
        text: Option<String>,
        output_file_path: Option<String>,
    }

    #[test]
    fn mute_beep() {
        let args = ["-t=Hello".to_owned(), "-b=false".to_owned()];
        let config = Config::build(&args).unwrap();
        let expect_config = ExpectConfig {
            beep: false,
            input_file_path: None,
            language: Alphabet::International,
            output_file_path: None,
            text: Some("Hello".to_owned()),
        };

        test_args(config, expect_config);
    }

    #[test]
    fn pass_text_from_file() {
        let args = ["-i=Hello.txt".to_owned()];
        let config = Config::build(&args).unwrap();
        let expect_config = ExpectConfig {
            beep: true,
            input_file_path: Some("Hello.txt".to_owned()),
            language: Alphabet::International,
            output_file_path: None,
            text: None,
        };

        test_args(config, expect_config);
    }

    #[test]
    fn set_language() {
        let args = ["--text=Hello".to_owned(), "-l=ua".to_owned()];
        let config = Config::build(&args).unwrap();
        let expect_config = ExpectConfig {
            beep: true,
            input_file_path: None,
            language: Alphabet::Ukrainian,
            output_file_path: None,
            text: Some("Hello".to_owned()),
        };

        test_args(config, expect_config);
    }

    #[test]
    fn output_to_file() {
        let args = ["--text=Hello".to_owned(), "-o=result.txt".to_owned()];
        let config = Config::build(&args).unwrap();
        let expect_config = ExpectConfig {
            beep: true,
            input_file_path: None,
            language: Alphabet::Ukrainian,
            output_file_path: Some("result.txt".to_owned()),
            text: Some("Hello".to_owned()),
        };

        test_args(config, expect_config);
    }

    #[test]
    fn pass_only_text() {
        let args = ["-t=Hello".to_owned()];
        let config = Config::build(&args).unwrap();
        let expect_config = ExpectConfig {
            beep: true,
            input_file_path: None,
            language: Alphabet::International,
            output_file_path: None,
            text: Some("Hello".to_owned()),
        };

        test_args(config, expect_config);
    }

    fn test_args(config: Config, expect_config: ExpectConfig) {
        assert_eq!(config.get_beep(), expect_config.beep);
        assert_eq!(config.get_input_file_path(), expect_config.input_file_path);
        assert_eq!(config.get_lang(), expect_config.language);
        assert_eq!(
            config.get_output_file_path(),
            expect_config.output_file_path
        );
        assert_eq!(config.get_text(), expect_config.text);
    }
}
