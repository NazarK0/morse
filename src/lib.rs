use std::error::Error;

mod config;
pub use config::*;

mod utils;
pub use utils::*;

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
    Ok(())
}
