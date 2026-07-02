#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {

        //print prompt
        print_prompt().unwrap();

        //read input
        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        //tokenizer
        let tokens:Vec<_> = command.split_whitespace().collect();
        let cmd = tokens[0];
        let args = tokens[1..].join(" ");

        //executor
        if cmd == "exit" {
            std::process::exit(0)
        }else if cmd == "echo"{
            println!("{}", args);
        }else{
            println!("{}: command not found", cmd);
        }
    }
}

fn print_prompt()-> io::Result<()>{
    print!("$ ");
    io::stdout().flush()?;
    Ok(())
}
fn read_input(){}
fn command_formatter()-> &'static str{"s"} // private
fn print_not_found(){} // printer interface
