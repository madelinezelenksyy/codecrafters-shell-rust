#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

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
        
        match command {
            "echo" => println!("{}", args),
            "exit" if args == "0" => process::exit(0),
            "type" => match args {
                "exit" | "echo" | "type" => println!("{} is a shell builtin", args),
                _ => println!("{}: not found", args),
            },
            &_ => {
                println!("{}: command not found", input.trim());
                input.clear();
            }
        }
    }
}
