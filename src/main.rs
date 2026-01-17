mod models;

use std::io::{self, Write};

fn main() {
    println!("rusty_expenses");
    println!("Type 'help' to see commands.");

    loop {
        let cmd = cmd_prompt();

        match cmd.as_str() {
            "help" => show_help(),
            "exit" => {
                println!("Bye.");
                break;
            }
            "" => continue,
            _ => println!("ERR: Unknown command. Type 'help'."),
        }
    }
}

fn show_help() {
    println!("COMMANDS");
    println!("\thelp\tShow this help");
    println!("\texit\tExit program");
}

fn read_line(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn cmd_prompt() -> String {
    read_line("> ")
}
