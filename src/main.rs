mod models;

use std::io::{self, Write};
use models::{Expense, Category, MenuOption};
use chrono::{NaiveDate, Local};

fn main() {
    println!("rusty_expenses");
    println!("Type 'help' to see commands.");

    let mut expenses: Vec<Expense> = Vec::new();
    let mut next_id: u32 = 1;

    loop {
        let cmd = cmd_prompt();

        match MenuOption::from_str(&cmd) {
            Some(MenuOption::Help) => show_help(),
            Some(MenuOption::Add) => add_expense(&mut expenses, &mut next_id),
            Some(MenuOption::Exit) => {
                println!("Bye.");
                break;
            }
            None => println!("ERR: Unknown command. Type 'help'."),
        }
    }
}

// Utility functions
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

fn ask(prompt: &str) -> String {
    read_line(&format!("{} ", prompt))
}

// Command functions
fn show_help() {
    println!("COMMANDS");
    println!("\thelp\tShow this help");
    println!("\tadd\tAdd a new expense");
    println!("\texit\tExit program");
}

    
fn add_expense(expenses: &mut Vec<Expense>, next_id: &mut u32) {
    let date_str = ask("Date (YYYY-MM-DD)? (empty=today)");

    let date = if date_str.is_empty() {
        Local::now().date_naive()
    } else {
        match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => {
                println!("ERR: Invalid date. Example: 2026-01-10");
                return;
            }
        }
    };
    
    let cents_str = ask("Amount (in cents)? (empty=0)");
    let cents: i64 = if cents_str.is_empty() {
        0
    } else {
        match cents_str.parse::<i64>() {
            Ok(c) => c,
            Err(_) => {
                println!("ERR: Invalid amount. Example: 1000");
                return;
            }
        }
    };

    let category_str = ask("Category (food, transport, leisure, bills, other)? (empty=other)");
    let category = if category_str.is_empty() {
        Category::Other
    } else {
        match Category::from_str(&category_str) {
            Some(c) => c,
            None => {
                println!("ERR: Invalid category. Example: food, transport, leisure, bills, other");
                return;
            }
        }
    };
    let mut note = ask("Note: ");

    if note.is_empty() {
        note = String::from("(no note)");
    }

    let expense = Expense {
        id: *next_id,
        date: date,
        cents: cents,
        category: category,
        note: note,
    };

    expenses.push(expense);
    *next_id += 1;

    println!("Added expense.")
}
