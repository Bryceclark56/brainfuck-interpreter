use std::{char, cmp::Ordering, io::{Write, stdout}, process::exit};

fn main() {
    let crate_version: &'static str = env!("CARGO_PKG_VERSION");
    println!("Brainfuck Interpreter v{}", crate_version);

    // Take input
    print!("> ");
    let _ = stdout().flush().unwrap();
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    let _ = stdin.read_line(&mut buffer).unwrap();

    let buffer = buffer.trim(); // Remove trailing newline and any whitespaces

    println!("Input: {}", &buffer); // Debugging purposes

    //Validate input
    // Check for valid characters
    const VALID_CHARS: &[char; 8] = &['>', '<', '+', '-', '.', ',', '[', ']'];

    for c in buffer.chars() {
        if VALID_CHARS.iter().any(|vc| char::cmp(&c, vc) == Ordering::Equal) {
            eprintln!(" '{}' is not a valid Brainfuck command", &c);
            exit(1); //TODO: Don't exit, just re-prompt
        }
    }

    // Check for valid structure (Every '[' has a ']' )
    let mut loop_stack = buffer.chars()
        .filter(|c| ['[', ']'].contains(&c))
        .collect::<Vec<char>>();

    if !loop_stack.is_empty() {
        if loop_stack.pop().unwrap() == '[' {
            eprintln!("Missing closing loop bracket");
        }
        else {
            eprintln!("Missing opening loop bracket");
        }
    }

    // Execute input as brainfuck commands

    // Display any output

    // Exit
}
