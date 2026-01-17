// models.rs
use chrono::NaiveDate;

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
}

#[derive(Debug)]
pub struct Expense {
    pub id: u32,
    pub date: NaiveDate,
    pub cents: i64,
    pub category: Category,
    pub note: String,
}
