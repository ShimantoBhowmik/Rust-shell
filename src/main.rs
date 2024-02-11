use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Stdio;
use std::process::Child;
use std::env;

fn main(){
    let mut history: Vec<String> = Vec::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim_end().to_string();

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

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next()  {

            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek()
                        .map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
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
                command => {
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            let _ = final_command.wait();
        }

    }
}