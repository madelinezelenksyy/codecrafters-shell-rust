#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> u32 {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        match input.trim() {
            "exit 0" => break,
            &_ => {
                println!("{}: command not found", input.trim());
                input.clear();
            }
        }
    }
}
