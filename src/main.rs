#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input_buf: String = String::new();
    let mut input_cmd = io::stdin().read_line(buf).unwrap();

    println!("{input_cmd}: command not found");

}