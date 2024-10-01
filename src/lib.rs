use std::error::Error;

mod config;
pub use config::*;

mod morse;
pub use morse::*;

mod utils;
pub use utils::*;

pub mod argument_parser;

// Usage morse [OPTION] [DATA]
// Convert text into Morse code
// Mandatory arguments to long options are mandatory for short options too.
// -l  --language=ISO 639-1 LANGUAGE CODE (Default us)
// -o  --output-file=PATHNAME
// -i  --input-file=PATHNAME
// -b  --beep
// -t  --translate=TEXT
//
// --help           display this help and exit
// --version        output version information and exit

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     parse_cmd_args(args);
// }

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{:?}", config);

    let display_units_as = DisplayAlias::new(Some('🔥'), Some('➖'), Some('🚧'));

    let morse = match config.get_text() {
        Some(text) => Morse::from_str(&text, config.get_lang(), Some(display_units_as)),
        None => match config.get_input_file_path() {
            Some(path) => {
                let text = "From file";
                Morse::from_str(text, config.get_lang(), Some(display_units_as))
            }
            None => {
                panic!("No input text to convert")
            }
        },
    };

    println!("{}", morse.to_string());

    if config.get_beep() {
        morse.to_beep();
    }

    Ok(())
}
