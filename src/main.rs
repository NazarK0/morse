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

fn missing_delimeter(flag: &str) {
    input_error(&format!("Missing {flag}'=' operand"))
}



fn parse_cmd_args(args: Vec<String>) {
    const MISSING_EQ_MSG: &str = "Missing '=' operand";
    let mut input_set = false;
    let mut text: String;

    if args.len() == 1 {
        input_error("missing operands");
        return;
    }

    for arg in &args {
        if arg.starts_with("--help") {
            println!("Show help")
        } else if arg.starts_with("--version") {
            version();
            return;
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
                    let flag;

                    

                    if arg.starts_with("-t"){
                        flag = "-t";

                    } else {
                        flag = "--translate";
                    }

                    if !arg.contains("=") {
                        missing_delimeter(flag);
                        return;
                    }
                    
                    text = extract_data_from_arg(arg, flag);
                    println!("You specify input from command line: {text}")
                }
            }
        }
    }

    if !input_set {
        input_error("missing input data");
    }
}

fn version() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let version_msg: String = format!(
        "morse {VERSION}
Copyright (C) 2024 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Nazar Vanivskyi\n"
    );
    print!("{version_msg}");
}

fn extract_data_from_arg(arg: &str, flag: &str) -> String {
    arg.replace(&format!("{flag}="), "")
}

fn translate (text: &str) {
    // TODO
}
