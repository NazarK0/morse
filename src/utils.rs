pub fn input_error(message: &str) {
    println!("morse: {}", message);
    println!("Try 'morse --help' for more information.");
}

pub fn missing_delimeter(arg: &str) {
    input_error(&format!("Missing {arg}'=' operand"))
}

pub fn version() {
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
