#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let trimmed_command = command.trim();
        let tokens:Vec<_> = trimmed_command.split_whitespace().collect();
        let cmd = tokens[0];
        let args = tokens[1..].join(" ");

        if cmd == "exit" {
            std::process::exit(0)
        }else if cmd == "echo"{
            println!("{:?}", args);
        }else{
            println!("{}: command not found", cmd);
        }
    }
}

fn print_prompt(){}
fn read_input(){}
fn command_formatter()-> &'static str{"s"} // private
fn print_not_found(){} // printer interface
