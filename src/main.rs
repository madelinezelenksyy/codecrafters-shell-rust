#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
use pathsearch::find_executable_in_path;

fn main() {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let (command, args) = input.trim().split_once(' ').unwrap_or((input.trim(), ""));

        // println!("{command} and {args}");
        
        match command {
            "echo" => println!("{}", args),
            "exit" if args == "0" => process::exit(0),
            command if command.starts_with("type") => {
                type_command(args);
            },
            &_ => {
                println!("{}: command not found", input.trim());
                input.clear();
            }
        }
    }
}

fn type_command(argument: &str) {
    // println!("{argument}");
    let builtins: [&str; 3] = ["type", "exit", "echo"];
    if builtins.contains(&argument) {
        println!("{argument} is a shell builtin");
    }
    else if let Some(executable) = find_executable_in_path(argument) {
        println!("{} is {}", argument, executable.display());
    }
    else {
        println!("{argument}: not found");
    }
}
