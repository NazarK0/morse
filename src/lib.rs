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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{:?}", config);

    let mut morse = match config.get_text() {
        Some(text) => Morse::from_str(&text, config.get_lang()),
        None => match config.get_input_file_path() {
            Some(path) => {
                let text = "From file";
                Morse::from_str(text, config.get_lang())
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

    if config.get_beep() {
        morse.to_beep();
    }

    Ok(())
}
