use std::error::Error;

mod config;
use argument_parser::{Alphabet, ArgError};
pub use config::*;

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

    let morse_txt = match config.get_text() {
        Some(text) => txt_to_morse(&text, config.get_lang()),
        None => {
            match config.get_input_file_path() {
                Some(path) => {

                    let text = "";
                    txt_to_morse(text, config.get_lang())
                },
                None => {
                    panic!("No input text to convert")
                }
            }
        },
    };

    if config.get_beep() {
        morse_txt_to_beep(&morse_txt);
    }


    Ok(())
}

fn txt_to_morse(text: &str, language: Alphabet) -> String {
    let morse_txt: String = "".to_string();

    println!("text: {text}, language: {language}");

    morse_txt
}

fn morse_txt_to_beep(text: &str) {

}
