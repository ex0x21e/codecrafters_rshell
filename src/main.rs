#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, ffi::OsStr, fs, os::unix::fs::PermissionsExt, path::PathBuf, process::Command};

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
            "type" => match args.as_str() {
                "echo" | "exit" | "type" => println!("{args} is a shell builtin"),
                _ => match search_exec(&args) {
                    Ok(path) => println!("{args} is {}", path.display()),
                    _ => println!("{args}: not found"),
                },
            },

            _ => {
                //here !
                let result = run_program(&cmd, &args);
                if let Ok(mut child) = result{
                    child.wait().unwrap();
                }else{
                    println!("{cmd}: command not found");
                }
            }
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

fn search_exec(command: &String) -> io::Result<PathBuf> {
    let env_path = match env::var("PATH") {
        Ok(val) => val,
        Err(_) => "can not read path".to_string(),
    };

    for paths in env::split_paths(&env_path) {
        for entry in fs::read_dir(paths)? {
            let entry = entry?; //unpacking entry
            let file_path = entry.path();

            if file_path.is_file() && file_path.file_name() == Some(OsStr::new(command)) {
                if is_exec(&file_path) {
                    return Ok(file_path);
                }
            }
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "command not found"))
}

fn run_program(command: &String, args: &String) -> io::Result<std::process::Child> {
    let arg_vec: Vec<_> = args.split_whitespace().collect();

    let child = Command::new(command)
        .args(arg_vec)
        .spawn();

    child
}

//helpers
fn is_exec(file: &PathBuf) -> bool {
    let file_meta = fs::metadata(file).unwrap();
    //check permission
    if file_meta.permissions().mode() & 0o111 != 0 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {

use super::*;

    #[test]
    fn test_run_program() {
        let result = run_program(&"gcc".to_string(), &"".to_string());
        if let Ok(mut result) = result{
            result.wait().unwrap();
        } // ждем завершения, чтобы не оставить зомби-процесс
    }
}
