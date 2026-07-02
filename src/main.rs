#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {

        //print prompt
        print_prompt().unwrap();

        //read input
        let command = read_input().unwrap();

        //tokenizer
        let (cmd, args) = tokenizer(command);

        //executor
        if cmd == "exit" {
            std::process::exit(0)
        }else if cmd == "echo"{
            println!("{}", args);
        }else if cmd == "type"{
            if args == "echo"{
                println!("{args} is a shell builtin")
            }else if args == "exit"{
                println!("{args} is a shell builtin")
            }else if args == "type"{
                println!("{args} is a shell builtin")
            }else{
                println!("{cmd} invalid_command")
            }
        }
        else{
            println!("{}: command not found", cmd);
        }
    }
}

fn print_prompt()-> io::Result<()>{
    print!("$ ");
    io::stdout().flush()?;
    Ok(())
}
fn read_input()-> io::Result<String>{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn tokenizer(input:String)->(String, String){
    let tokens:Vec<_> = input.split_ascii_whitespace().collect();
    let cmd = tokens[0].to_string();
    let args = tokens[1..].join(" ");
    (cmd, args)
}

fn command_formatter()-> &'static str{"s"} // private
fn print_not_found(){} // printer interface
