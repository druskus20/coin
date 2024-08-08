use serde::{Deserialize, Serialize};

use crate::currency::Currency;
use crate::DateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub datetime: DateTime,
    pub category: String,
    pub amount: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub id: i32,
    pub datetime: DateTime,
    pub base_currency: Currency,
    pub currency: Currency,
    pub amount_from_base: f64,
    pub amount_to_base: f64,
}
