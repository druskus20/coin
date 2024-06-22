use crate::currency::Currency;
use crate::DateTime;

pub struct Expense {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub datetime: DateTime,
    pub category: String,
    pub cost: Amount,
}

pub struct Amount {
    pub id: i32,
    pub datetime: DateTime,
    pub base_currency: Currency,
    pub currency: Currency,
    pub amount_from_base: f64,
    pub amount_to_base: f64,
}
