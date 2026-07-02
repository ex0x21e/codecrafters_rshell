#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let trimmed_command = command.trim();
        let args:Vec<_> = trimmed_command.split(" ").skip(1).collect();
        

        if trimmed_command == "exit" {
            std::process::exit(0)
        }else if trimmed_command == "echo"{
            println!("{:?}", args);
        }else{
            println!("{}: command not found", trimmed_command);
        }
    }
}

fn print_prompt(){}
fn read_input(){}
fn command_formatter()-> &'static str{"s"} // private
fn print_not_found(){} // printer interface
