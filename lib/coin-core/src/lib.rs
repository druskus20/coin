mod db;
mod errors;

use errors::Result;
use wasm_bindgen::{prelude::wasm_bindgen, JsError};
use web_sys::console;

pub struct Coin {
    db: db::Db,
}

impl Coin {
    pub async fn try_new() -> Result<Coin> {
        let db = db::Db::try_init().await?;
        Ok(Self { db })
    }

    pub async fn add_expense(&self, amount: u32) -> Result<()> {
        self.db.add_expense(amount).await?;
        Ok(())
    }

    pub async fn get_expenses(&self) -> Result<Vec<db::Expense>> {
        let expenses = self.db.get_expenses().await?;
        Ok(expenses)
    }
}

#[wasm_bindgen]
pub async fn run() -> std::result::Result<(), JsError> {
    let coin = Coin::try_new().await?;
    coin.add_expense(10).await?;
    coin.add_expense(20).await?;
    let expenses = coin.get_expenses().await?;
    for expense in expenses {
        console::log_1(&format!("Expense: {}", expense.amount).into());
    }
    Ok(())
}
