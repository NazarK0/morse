use morse::{argument_parser::ArgError, input_error, run, version, Config};
use std::{env, process};

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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        if args[1].starts_with("--help") {
            println!("Show help");
            return;
        }

        if args[1].starts_with("--version") {
            version();
            return;
        }
    }

    let config = Config::build(&args).unwrap_or_else(|err| {
        match err {
            ArgError::MissingOperands => input_error("Missing operands"),
            ArgError::MissingInputData(arg) => {
                input_error(&format!("Arg '{arg }' missing input data"))
            }
            ArgError::InvalidArg(arg) => {
                let message = format!("Invalid arg '{arg}'");

                if arg.contains('-') {
                    input_error(&message);
                } else {
                    input_error(&format!(
                        "{message}. Maybe you missing quotes in the data string"
                    ));
                }
            }
            ArgError::MultipleInputs => input_error(&format!("Multiple inputs")),
            ArgError::UnsupportedLanguage(lang) => {
                input_error(&format!("Unsupported language '{lang}'"))
            }
        }

        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(2)
    }
}
