use std::error::Error;

mod config;
use argument_parser::Alphabet;
pub use config::*;


mod utils;
pub use utils::*;

pub mod argument_parser;

use morse_lib::Morse;


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



pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{:?}", config);

    let mut morse = match config.get_text() {
        Some(text) => {
            if config.get_lang() == Alphabet::International {
                Morse::from_str(&text)
            } else {
                // let mut  morse = Morse::new("config_path");
                // morse.parse_text(&text);

                // morse
                panic!("Unsupported language")
            }
        }
        None => match config.get_input_file_path() {
            Some(path) => {
                let text = "From file";
                Morse::from_str(text)
            }
            None => {
                panic!("No input text to convert")
            }
        },
    };

    // morse.dot_as("🔥");
    // morse.line_as("➖");
    // morse.whitespace_as("🚧");

    // morse.dot_as("ABA");
    // morse.line_as("GEGo");
    // morse.whitespace_as("STOP");

    // morse.dot_as("☢️");
    // morse.line_as("🐛");
    // morse.whitespace_as("🚧");

    println!("{}", morse.to_string());
    println!("{}", morse.to_bin_str());

    morse.frequency(500.0);
    morse.play_speed(5.0);
    // morse.to_beep();

    if config.get_beep() {
        morse.to_beep();
    }

    Ok(())
}
