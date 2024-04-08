use crate::errors::Error;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

type SDb = surrealdb::Surreal<surrealdb::engine::local::Db>;

use surrealdb::engine::local::Mem;

#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    title: String,
    amount: u32,
    date: DateTime<Utc>,
}

pub(super) struct Db {
    db: SDb,
}

const EXPENSES_TABLE: &str = "expense";
impl Db {
    pub(super) async fn try_init() -> Result<Self, Error> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("coin").use_db("coin").await?;
        Ok(Self { db })
    }

    pub(super) async fn add_expense(&self, amount: u32) -> surrealdb::Result<()> {
        self.db
            .create::<Vec<Expense>>(EXPENSES_TABLE)
            .content(Expense {
                title: "Test expense".into(),
                amount,
                date: Utc::now().into(),
            })
            .await?;

        Ok(())
    }

    pub(super) async fn get_expenses(&self) -> surrealdb::Result<Vec<Expense>> {
        let expenses: Vec<Expense> = self.db.select(EXPENSES_TABLE).await?;
        Ok(expenses)
    }
}
