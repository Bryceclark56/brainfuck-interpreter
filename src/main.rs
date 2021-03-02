use std::{char, io::{Write, stdin, stdout}, process::exit};

fn main() {
    let crate_version: &'static str = env!("CARGO_PKG_VERSION");
    println!("Brainfuck Interpreter v{}", crate_version);

    // Take input
    print!("> ");
    let _ = stdout().flush().unwrap();
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    let _ = stdin.read_line(&mut buffer).unwrap();

    // Remove trailing newline
    if buffer.ends_with('\n') {
        buffer.pop();
    }
    else if buffer.ends_with("\r\n") {
        buffer.pop();
        buffer.pop();
    }

    println!("Input: {}", &buffer); // Debugging purposes

    // Validate input
        // Check for valid characters
    const VALID_CHARS: [char; 8] = ['>', '<', '+', '-', '.', ',', '[', ']'];
    let char_check = buffer.into_bytes().iter()
        .map(|c| *c as char)
        .try_for_each(|c| {
            if !VALID_CHARS.contains(&c) {
                Err(format!("{} is not a valid Brainfuck command", c))
            }
            else {
                Ok(())
            }
        });

    if char_check.is_err() {
        eprintln!("{}", char_check.err().unwrap());
        exit(1);
    }
        // Check for valid structure (Every [ has a ])

    // Execute input as brainfuck commands

    // Display any output

    // Exit
}
