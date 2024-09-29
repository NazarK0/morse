use crate::argument_parser::{parse_arg, Alphabet, ArgError, ConfigField};



#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    language: Alphabet,
    beep: bool,
    input_file_path: Option<String>,
    text: Option<String>,
    output_file_path: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, ArgError> {
        let mut language = Alphabet::International;
        let mut beep = false;
        let mut input_file_path = None;
        let mut text = None;
        let mut output_file_path = None;

        let mut input_set = false;

        if args.len() == 1 {
            return Err(ArgError::MissingOperands);
        }

        for arg in &args[1..] {
            let parsed_arg = parse_arg(arg)?;

            match parsed_arg.field {
                ConfigField::Language(lang) => language = lang,
                ConfigField::Beep => beep = true,
                ConfigField::InputFilePath(path) => {
                    if input_set {
                        return Err(ArgError::MultipleInputs);
                    }
                    input_set = true;
                    input_file_path = Some(path)
                }
                ConfigField::Text(txt) => {
                    if input_set {
                        return Err(ArgError::MultipleInputs);
                    }
                    input_set = true;
                    text = Some(txt)
                }
                ConfigField::OutputFilePath(path) => output_file_path = Some(path),
            }
        }

        Ok(Config {
            language,
            beep,
            input_file_path,
            output_file_path,
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

    #[test]
    fn beep() {
        let args = ["/morse".to_owned(), "-t=Hello".to_owned(), "-b".to_owned()];
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

    #[test]
    fn pass_text_from_file() {
        let args = ["/morse".to_owned(), "-i=Hello.txt".to_owned()];
        let config = Config::build(&args).unwrap();
        let expect_config = ExpectConfig {
            beep: false,
            input_file_path: Some("Hello.txt".to_owned()),
            language: Alphabet::International,
            output_file_path: None,
            text: None,
        };

        test_args(config, expect_config);
    }

    #[test]
    fn set_language() {
        // ua
        let args = [
            "/morse".to_owned(),
            "--text=Hello".to_owned(),
            "-l=ua".to_owned(),
        ];
        let config = Config::build(&args).unwrap();
        let expect_config = ExpectConfig {
            beep: false,
            input_file_path: None,
            language: Alphabet::Ukrainian,
            output_file_path: None,
            text: Some("Hello".to_owned()),
        };

        test_args(config, expect_config);

        // en
        let args = [
            "/morse".to_owned(),
            "--text=Hello".to_owned(),
            "-l=en".to_owned(),
        ];
        let config = Config::build(&args).unwrap();
        let expect_config = ExpectConfig {
            beep: false,
            input_file_path: None,
            language: Alphabet::English,
            output_file_path: None,
            text: Some("Hello".to_owned()),
        };

        test_args(config, expect_config);

        // int
        let args = [
            "/morse".to_owned(),
            "--text=Hello".to_owned(),
            "-l=int".to_owned(),
        ];
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
    fn output_to_file() {
        let args = [
            "/morse".to_owned(),
            "--text=Hello".to_owned(),
            "-o=result.txt".to_owned(),
        ];
        let config = Config::build(&args).unwrap();
        let expect_config = ExpectConfig {
            beep: false,
            input_file_path: None,
            language: Alphabet::International,
            output_file_path: Some("result.txt".to_owned()),
            text: Some("Hello".to_owned()),
        };

        test_args(config, expect_config);
    }

    #[test]
    fn pass_only_text() {
        let args = ["/morse".to_owned(), "-t=Hello".to_owned()];
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
    fn pass_long_text() {
        let args = ["/morse".to_owned(), "-t=Hello world".to_owned()];
        let config = Config::build(&args).unwrap();
        let expect_config = ExpectConfig {
            beep: false,
            input_file_path: None,
            language: Alphabet::International,
            output_file_path: None,
            text: Some("Hello world".to_owned()),
        };

        test_args(config, expect_config);
    }

    #[test]
    fn should_panic_no_arguments() {
        let args = ["/morse".to_owned()];
        let config = Config::build(&args);
        let e = config.unwrap_err();
        assert_eq!(e, ArgError::MissingOperands);
    }

    #[test]
    fn should_panic_multiple_inputs() {
        let args = [
            "/morse".to_owned(),
            "-t=Hello".to_owned(),
            "-i=Hello.txt".to_owned(),
        ];
        let config = Config::build(&args);
        let e = config.unwrap_err();
        assert_eq!(e, ArgError::MultipleInputs);
    }

    #[test]
    fn should_panic_missing_delimeter() {
        let args = ["/morse".to_owned(), "-t*Hello".to_owned()];
        let config = Config::build(&args);
        let e = config.unwrap_err();
        assert_eq!(e, ArgError::InvalidArg("-t*Hello".to_owned()));
    }

    #[test]
    fn should_panic_missing_data() {
        //text
        let args = ["/morse".to_owned(), "-t".to_owned()];
        let config = Config::build(&args);
        let e = config.unwrap_err();
        assert_eq!(e, ArgError::MissingInputData("-t".to_owned()));

        let args = ["/morse".to_owned(), "--text".to_owned()];
        let config = Config::build(&args);
        let e = config.unwrap_err();
        assert_eq!(e, ArgError::MissingInputData("--text".to_owned()));

        //input from file
        let args = ["/morse".to_owned(), "-i".to_owned()];
        let config = Config::build(&args);
        let e = config.unwrap_err();
        assert_eq!(e, ArgError::MissingInputData("-i".to_owned()));

        let args = ["/morse".to_owned(), "--input-file".to_owned()];
        let config = Config::build(&args);
        let e = config.unwrap_err();
        assert_eq!(e, ArgError::MissingInputData("--input-file".to_owned()));

        //language
        let args = ["/morse".to_owned(), "-t=Hello".to_owned(), "-l".to_owned()];
        let config = Config::build(&args);
        let e = config.unwrap_err();
        assert_eq!(e, ArgError::MissingInputData("-l".to_owned()));

        let args = [
            "/morse".to_owned(),
            "-t=Hello".to_owned(),
            "--language".to_owned(),
        ];
        let config = Config::build(&args);
        let e = config.unwrap_err();
        assert_eq!(e, ArgError::MissingInputData("--language".to_owned()));
    }

    #[test]
    fn should_panic_unsupported_language() {
        // ru
        let args = ["/morse".to_owned(), "-l=ru".to_owned()];
        let config = Config::build(&args);
        let e = config.unwrap_err();
        assert_eq!(e, ArgError::UnsupportedLanguage("ru".to_owned()));

        // it
        let args = ["/morse".to_owned(), "-l=it".to_owned()];
        let config = Config::build(&args);
        let e = config.unwrap_err();
        assert_eq!(e, ArgError::UnsupportedLanguage("it".to_owned()));
    }
}
