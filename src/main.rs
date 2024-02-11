use std::process::{Command, Stdio, Child};
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::env;

fn main() {
    let mut history: Vec<String> = Vec::new();
    loop {
        print!("r@shell > ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim_end().to_string();

        if input.trim().is_empty() {
            continue;
        }

        //support for multiline commands
        while input.ends_with('\\') {
            input.pop();
            print!("> ");
            stdout().flush().unwrap();

            let mut next_input = String::new();
            stdin().read_line(&mut next_input).unwrap();
            input.push_str(next_input.trim_end());
        }

        if !input.trim().is_empty() {
            history.push(input.trim().to_string());
        }

        //support for && and ;
        let command_blocks = input.trim().split_inclusive(|c| c == '&' || c == ';');
        for block in command_blocks {
            let block = block.trim_matches(|c| c == '&' || c == ';').trim();
            if block.is_empty() {
                continue;
            }

            let mut previous_command: Option<Child> = None;
            let mut commands = block.split(" | ").peekable();
            let mut block_success = true;

            while let Some(command) = commands.next() {
                let mut parts = command.trim().split_whitespace();
                let command = parts.next().unwrap();
                let args = parts;

                match command {
                    "cd" => {
                        let new_dir = args.peekable().peek().map_or("/", |x| *x);
                        let root = Path::new(new_dir);
                        if let Err(e) = env::set_current_dir(&root) {
                            eprintln!("{}", e);
                            block_success = false;
                        }
                        previous_command = None;
                    },
                    "exit" => return,
                    "history" => {
                        for (index, command) in history.iter().enumerate() {
                            println!("{}. {}", index + 1, command);
                        }
                        previous_command = None;
                    },
                    _ => {
                        let stdin = previous_command
                            .take()
                            .map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));

                        let stdout = if commands.peek().is_some() {
                            Stdio::piped()
                        } else {
                            Stdio::inherit()
                        };

                        let child = Command::new(command)
                            .args(args)
                            .stdin(stdin)
                            .stdout(stdout)
                            .spawn();

                        match child {
                            Ok(child) => { previous_command = Some(child); },
                            Err(e) => {
                                eprintln!("No such command.");
                                eprintln!("Error: {}", e);
                                block_success = false;
                                break;
                            },
                        }
                    }
                }
            }

            if let Some(mut final_command) = previous_command {
                if let Ok(exit_status) = final_command.wait() {
                    if !exit_status.success() {
                        block_success = false;
                    }
                } else {
                    block_success = false;
                }
            }

            if block.ends_with("&&") && !block_success {
                break;
            }
        }
    }
}
