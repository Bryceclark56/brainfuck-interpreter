use std::{char, fmt, io::{Write, stdout}};

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

    let _commands = Command::parse_string(buffer);

    // Execute input as brainfuck commands


    // Display any output

    // Exit
}

struct Machine {
    data: Vec<u8>,
    pointer: usize,

    command_pointer: usize,
    commands: Vec<Command>,

    in_loop: bool,
    loop_start: usize,
}

impl Machine {
    // Ensures that all of the values up to the data pointer exist
    fn fill_to_pointer(&mut self) {
        if self.data.len() <= self.pointer  {
            let goal = self.pointer;
            let current = self.data.len();
            for _ in current..=goal {
                self.data.push(0);
            }
        }
    }

    /* Executes the command at commands[command_pointer]

    Execute is lazy with data initialization.
    Data is only initialized if it's modified or read from a new position */
    fn execute_command(&mut self) {
        match self.commands[self.command_pointer] {
            Command::IncrementPointer => self.pointer = self.pointer + 1,
            Command::DecrementPointer => self.pointer = self.pointer - 1,

            Command::Increment => {
                self.fill_to_pointer();
                self.data[self.pointer] = self.data[self.pointer] + 1;
            },
            Command::Decrement => {
                self.fill_to_pointer();
                self.data[self.pointer] = self.data[self.pointer] + 1;
            },

            Command::Output => print!("{}", self.output() as char),
            Command::Input => todo!(),

            Command::LoopStart => self.loop_start = self.command_pointer,
            Command::LoopEnd => {
                if self.data[self.pointer] != 0 {
                    self.command_pointer = self.loop_start;
                }
            },
        }
    }

    // This should NEVER panic
    fn output(&self) -> u8 {
        self.data[self.pointer]
    }
}

impl Default for Machine {
    fn default() -> Self {
        Machine {
            data: Vec::new(),
            pointer: 0,

            command_pointer: 0,
            commands: Vec::new(),
            in_loop: false,
            loop_start: 0,
        }
    }
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
        let valid_chars = ['>', '<', '+', '-', '.', ',', '[', ']'];
        // Convert the characters into commands
        let commands: Vec<Command> = s.chars()
        .filter(|c| valid_chars.contains(c))
        .map(|c| {
            match c {
                '>' => Command::IncrementPointer,
                '<' => Command::DecrementPointer,

                '+' => Command::Increment,
                '-' => Command::Decrement,

                '.' => Command::Output,
                ',' => Command::Input,

                '[' => Command::LoopStart,
                ']' => Command::LoopEnd,

                _ => Command::Output // Dummy value. This case should never be matched.
            }
        })
        .collect();

        // Check for matching square brackets
        let mut stack: Vec<Command> = Vec::new();
        for cmd in &commands {
            match cmd {
                Command::LoopStart => stack.push(*cmd),
                Command::LoopEnd => {
                    if stack.last() == Some(&Command::LoopStart) {
                        stack.pop();
                    }
                    else {
                        stack.push(*cmd);
                    }
                },
                _ => {}
            }
        }

        // I should say where in the input as well
        if stack.len() > 0 {
            let last_bracket = stack.last().unwrap_or_else(|| &&Command::Input);

            return Err(BrainfuckError::MissingBracketError(
                *last_bracket == Command::LoopStart
            ));
        }


        Ok(commands)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum BrainfuckError {
    //InvalidCommandError(char),
    MissingBracketError(bool), // True if missing closing bracket, false if missing opening bracket
}

impl fmt::Display for BrainfuckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            //BrainfuckError::InvalidCommandError(cmd) => write!(f, "Invalid command: {}", cmd),
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

impl fmt::Debug for BrainfuckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            //BrainfuckError::InvalidCommandError(cmd) => write!(f, "Invalid command: {}", cmd),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_string() -> Result<(), BrainfuckError> {
        let input_string = ">>> ++++[ >>+ <<- ] >>.";
        let expected = vec![
            Command::IncrementPointer,
            Command::IncrementPointer,
            Command::IncrementPointer,

            Command::Increment,
            Command::Increment,
            Command::Increment,
            Command::Increment,

            Command::LoopStart,
                Command::IncrementPointer,
                Command::IncrementPointer,

                Command::Increment,

                Command::DecrementPointer,
                Command::DecrementPointer,

                Command::Decrement,
            Command::LoopEnd,

            Command::IncrementPointer,
            Command::IncrementPointer,

            Command::Output
        ];

        let result = Command::parse_string(input_string)?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn parse_string_with_mismatching_brackets() {
        let input_string = ">>>++++>>+<<-]>>.";

        let result = Command::parse_string(input_string).unwrap_err();

        assert_eq!(result, BrainfuckError::MissingBracketError(false));


        let input_string = ">>>++++[>>+<<->>.";

        let result = Command::parse_string(input_string).unwrap_err();

        assert_eq!(result, BrainfuckError::MissingBracketError(true));



        let input_string = ">>>++++[>>+<<-]>>[-]+[[[-]]";

        let result = Command::parse_string(input_string).unwrap_err();

        assert_eq!(result, BrainfuckError::MissingBracketError(true));


        let input_string = ">>>++++[>>+<<-]>>[-]+[[-]]]";

        let result = Command::parse_string(input_string).unwrap_err();

        assert_eq!(result, BrainfuckError::MissingBracketError(false));
    }
}