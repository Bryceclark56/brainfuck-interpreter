use std::{char, convert::{TryFrom, TryInto}, error::Error, fmt::{self, write}, io::{Write, stdout}, process::exit};

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

    let commands = Command::parse_string(buffer);

    // Execute input as brainfuck commands

    // Display any output

    // Exit
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    IncrementPointer,
    DecrementPointer,
    //
    Increment,
    Decrement,
    //
    Output,
    Input,
    //
    LoopStart,
    LoopEnd,
}

impl Command {
    fn parse_string(s: &str) -> Result<Vec<Command>, BrainfuckError> {
        // Convert the characters into commands
        let commands = s.chars()
        .map(|c| Command::try_from(&c))
        .collect::<Result<Vec<Command>, BrainfuckError>>()?;

        // Check for matching square brackets
        let mut stack: Vec<Command> = Vec::new();
        for cmd in &commands {
            match cmd {
                Command::LoopStart => stack.push(*cmd),
                Command::LoopEnd => {
                    if *stack.last().unwrap_or_else(|| &&Command::Input) == Command::LoopStart {
                        stack.pop();
                    }

                    stack.push(*cmd);
                },
                _ => {}
            }
        }

        // I should say where in the input as well
        if stack.len() > 0 {
            let last_bracket = stack.last().unwrap_or_else(|| &&Command::Input);

            return Err(BrainfuckError::MissingBracketError(
                *last_bracket == Command::LoopEnd
            ));
        }


        Ok(commands)
    }
}

impl TryFrom<&char> for Command {
    type Error = BrainfuckError;

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        match c {
            '>' => Ok(Command::IncrementPointer),
            '<' => Ok(Command::DecrementPointer),
    
            '+' => Ok(Command::Increment),
            '-' => Ok(Command::Decrement),
    
            '.' => Ok(Command::Output),
            ',' => Ok(Command::Input),
    
            '[' => Ok(Command::LoopStart),
            ']' => Ok(Command::LoopEnd),

            _ => Err(BrainfuckError::InvalidCommandError(c.clone()))
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum BrainfuckError {
    InvalidCommandError(char),
    MissingBracketError(bool)
}

impl fmt::Display for BrainfuckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrainfuckError::InvalidCommandError(cmd) => write!(f, "Invalid command: {}", cmd),
            BrainfuckError::MissingBracketError(is_closing_bracket) => {
                if *is_closing_bracket {
                    write!(f, "Missing closing bracket")
                }
                else {
                    write!(f, "Missing opening bracket")
                }
            }
        }
    }
}

impl std::error::Error for BrainfuckError {}