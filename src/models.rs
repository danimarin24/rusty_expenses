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

#[derive(Debug)]
pub struct Expense {
    pub id: u32,
    pub date: NaiveDate,
    pub cents: i64,
    pub category: Category,
    pub note: String,
}
