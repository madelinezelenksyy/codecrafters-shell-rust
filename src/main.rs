#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
use std::process::Command;
use pathsearch::find_executable_in_path;
use std::env;
use std::path::{Path, PathBuf};
use dirs;

fn main() {
    loop {
        // Initial prompt
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let (command, args) = input.trim().split_once(' ').unwrap_or((input.trim(), ""));
        
        match command {
            "echo" => {
                println!("{}", args);
            } ,

            "exit" if args == "0" => process::exit(0),

            command if command.starts_with("type") => {
                type_command(args);
            },
            "pwd" => {
               let _ = print_working_directory();
            },
            "cd" => {
                let _ = change_directory(args);
            }
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

fn change_directory(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = if path.starts_with('~') {
        let home_dir = dirs::home_dir().ok_or("Home directory not found")?;
        let path = path.strip_prefix('~').unwrap_or(path); //Remove ~ symbol
        home_dir.join(path)
    } else {
        // If the path doesn't start with '~', proceed as usual
        let root = env::current_dir()?; //Get current directory
        if Path::new(path).is_absolute() {
            PathBuf::from(path) //If it's an absolute path
        } else {
            root.join(path) //For relative paths prefix with the current directory
        }
    };
    
    //If the path doesn't exist, throw an error
    if !path.exists() {
        println!("cd: {}: No such file or directory", path.display());
        return Err(From::from("Path does not exist"));
    }

    //If the path is not a directory, throw an error
    if !path.is_dir() {
        println!("cd: {}: Not a directory", path.display());
        return Err(From::from("Path is not a directory"));
    }

    env::set_current_dir(&path)?; //Change to desired directory
    Ok(())
}