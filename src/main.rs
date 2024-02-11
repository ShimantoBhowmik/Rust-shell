use std::process::Command;
use std::io::{stdin};

fn main() {
    let mut input_command = String::new();
    stdin().read_line(&mut input_command).unwrap();
    let input_command = input_command.trim();
    Command::new(input_command)
        .spawn()
        .unwrap();

}
