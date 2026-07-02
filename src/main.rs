#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let trimmed_command:Vec<&str> = command.trim().split(" ").collect();
        

        if trimmed_command[0] == "exit" {
            std::process::exit(0)
        }else if trimmed_command[0] == "echo"{
            println!("{}", trimmed_command[1]);
        }else{
            println!("{}: command not found", trimmed_command[0]);
        }
    }
}

fn print_prompt(){}
fn read_input(){}
fn command_formatter()-> &'static str{"s"} // private
fn print_not_found(){} // printer interface
