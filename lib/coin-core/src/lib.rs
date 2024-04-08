mod db;
mod errors;

use errors::Result;

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
