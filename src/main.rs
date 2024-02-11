use std::process::Command;
use std::io::{stdin, stdout, Write};
use std::path::Path;

fn main() {
    loop {
        print!("@ ");
        stdout().flush().unwrap();

        let mut input_command = String::new();
        stdin().read_line(&mut input_command).unwrap();
        let mut input = input_command.trim().split_whitespace();
        let command = input.next().unwrap();
        let args = input;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = std::env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,
            command =>{
                let child = Command::new(command)
                    .args(args)
                    .spawn();
                
                //error handle
                match child {
                    Ok(mut child) => { 
                        let _ = child.wait(); 
                    },
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
