#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");

        io::stdout().flush().unwrap();

        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let trimmed_command = command.trim();

        if trimmed_command == "exit" {
            std::process::exit(0)
        }
        println!("{}: command not found", trimmed_command);
    }
}
