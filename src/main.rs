#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
use std::process::Command;
use pathsearch::find_executable_in_path;
use std::env;

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
            "pwd" => {
               let _ = print_working_directory();
            },
            &_ => {
            
                if !run_external_commands(command, args) {
                    println!("{}: command not found", input.trim());
                    input.clear();
                }
            }
        }
    }
}

fn type_command(argument: &str){
    // println!("{argument}");
    let builtins: [&str; 4] = ["type", "exit", "echo", "pwd"];
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

fn run_external_commands(command: &str, args: &str) -> bool {
    // Try to run an external command using `Command`
    if let Some(executable) = find_executable_in_path(command) {
        let output = Command::new(command)
            .arg(args) // Pass the arguments to the command
            .output(); // Run the command and capture the output

        match output {
            Ok(output) => {
                // Print standard output
                if !output.stdout.is_empty() {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
                // Print standard error
                if !output.stderr.is_empty() {
                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                }
                true
            }
            Err(_) => {
                // If the command fails to run
                println!("Error executing command");
                false
            }
        }
    } else {
        // Command not found in the system's executable path
        false
    }
}

fn print_working_directory() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}