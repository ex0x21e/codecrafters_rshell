use std::{ffi::OsStr, fs, os::unix::fs::PermissionsExt, path::PathBuf};
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        //print prompt
        print_prompt().unwrap();

        //read input
        let input = match read_input() {
            Ok(input) if input.trim().is_empty() => continue,
            Ok(input) => input,
            Err(_) => continue,
        };

        //tokenizer
        let (cmd, args) = tokenizer(&input);

        //executor
        match cmd.as_str() {
            "exit" => std::process::exit(0),
            "echo" => println!("{args}"),
            "type" => {
                match args.as_str() {
                    "echo" | "exit" | "type" => println!("{args} is a shell builtin"),
                    _ => {
                        match search_exec(&args) {
                            Ok(path) => println!("{args} is {}", path.display()),
                            _ => println!("{args}: not found")
                        }
                    },
                }
            }
            _ => println!("{cmd}: command not found"),
        }
    }
}

fn print_prompt() -> io::Result<()> {
    print!("$ ");
    io::stdout().flush()?;
    Ok(())
}
fn read_input() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn tokenizer(input: &String) -> (String, String) {
    let tokens: Vec<_> = input.split_ascii_whitespace().collect();
    //это наверое должна быть отдельная функция
    let cmd = tokens[0].to_string();
    let args = tokens[1..].join(" ");
    (cmd, args)
}


fn search_exec(command: &String)-> io::Result<PathBuf>{
    let env_path =  env!("PATH");
    let path_list:Vec<_> =  env_path.split(":").collect();

    for dir in &path_list {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_path = entry.path();

            if file_path.is_file() && file_path.file_name() == Some(OsStr::new(&command)){
                if let Ok(metadata) = fs::metadata(&file_path){
                    if metadata.permissions().mode() & 0o111 !=0{
                        return Ok(file_path);
                    }
                }
                
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "command not found"))
}