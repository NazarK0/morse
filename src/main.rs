use std::env;

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

    parse_cmd_args(args);
}

fn input_error(message: &str) {
    println!("morse: {}", message);
    println!("Try 'morse --help' for more information.");
}

fn parse_cmd_args(args: Vec<String>) {
    let mut input_set = false;
    if args.len() == 1 {
        input_error("missing operands");
        return;
    }

    for arg in &args {
        if arg.starts_with("--help") {
            println!("Show help")
        } else if arg.starts_with("--version") {
            println!("Show version")
        } else {
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
                    println!("You specify input from command line")
                }
            }
        }
    }

    if !input_set {
        input_error("missing input data");
    }
}
