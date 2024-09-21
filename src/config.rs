mod config_error;
pub use config_error::ConfigError;

#[derive(Debug, Copy, Clone)]
pub enum Alphabet {
    International,
    Ukrainian,
    English,
}

pub struct Config {
    language: Alphabet,
    beep: bool,
    input_file: Option<String>,
    text: Option<String>,
    output_file: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, ConfigError> {
        let mut input_set = false;
        let mut text: String;

        if args.len() == 1 {
            return Err(ConfigError::MissingOperands);
        }

        for arg in args {
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
                    println!("You specify input from file")
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

                    text = Self::extract_data_from_arg(arg, flag);
                    println!("You specify input from command line: {text}")
                }
            }
        }

        if !input_set {
            return Err(ConfigError::MissingInputData);
        }

        Ok(Config {
            language: Alphabet::International,
            beep: true,
            input_file: None,
            output_file: None,
            text: Some(String::from("SOS")),
        })
    }

    // getters
    pub fn get_lang(&self) -> Alphabet {
        self.language
    }

    pub fn get_beep(&self) -> bool {
        self.beep
    }

    pub fn get_input_file(&self) -> Option<String> {
        self.input_file.clone()
    }

    pub fn get_output_fle(&self) -> Option<String> {
        self.output_file.clone()
    }

    pub fn get_text(&self) -> Option<String> {
        self.text.clone()
    }

    // private methods
    fn extract_data_from_arg(arg: &str, flag: &str) -> String {
        arg.replace(&format!("{flag}="), "")
    }
}
