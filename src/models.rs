// models.rs
use chrono::NaiveDate;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Food,
    Transport,
    Leisure,
    Bills,
    Other,
}

impl Category {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "food" => Some(Category::Food),
            "transport" => Some(Category::Transport),
            "leisure" => Some(Category::Leisure),
            "bills" => Some(Category::Bills),
            "other" => Some(Category::Other),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Category::Food => "food",
            Category::Transport => "transport",
            Category::Leisure => "leisure",
            Category::Bills => "bills",
            Category::Other => "other",
        }
    }
}

#[derive(Debug)]
pub struct Expense {
    pub id: u32,
    pub date: NaiveDate,
    pub cents: i64,
    pub category: Category,
    pub note: String,
}

impl Display for Expense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expense: [id: {}, date: {}, cents: {}, category: {}, note: '{}']", self.id, self.date, self.cents, self.category.to_str(), self.note)
    }
}

#[derive(Debug)]
pub enum MenuOption {
    Help,
    Add,
    List,
    Exit,
}

impl MenuOption {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "help" => Some(MenuOption::Help),
            "add" => Some(MenuOption::Add),
            "list" => Some(MenuOption::List),
            "exit" => Some(MenuOption::Exit),
            _ => None,
        }
    }

    pub fn list_options() {
        println!("COMMANDS");
        println!("  help\tShow this help");
        println!("  add\tAdd a new expense");
        println!("  list\tList all expenses");
        println!("  exit\tExit program");
    }
}
