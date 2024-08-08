use expense::Expense;
use firestore::FirestoreDb;
use serde::{Deserialize, Serialize};

pub mod currency;
pub mod currency_api;
mod db;
pub mod error;
pub mod expense;
pub mod oauth;
pub(crate) mod utils;

type DateTime = chrono::DateTime<chrono::Utc>;
const EXPENSE_COLLECTION_NAME: &str = "expenses";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TestRecord {
    name: String,
    value: f64,
}

pub async fn init() -> Result<FirestoreDb, error::CoinError> {
    let cm = oauth::OAuthManager::try_init_with_github().await?;
    let project_id = &std::env::var("COIN_FIRESTORE_PROJECT_ID").map_err(|err| {
        error::CoinError::Environment {
            source: err,
            key: "COIN_FIRESTORE_PROJECT_ID".to_string(),
        }
    })?;
    let db = db::try_new(&cm, project_id.clone()).await?;
    dbg!(&db);
    Ok(db)
}

pub async fn add_expense(db: &FirestoreDb, expense: Expense) -> Result<(), error::CoinError> {
    let _ = db
        .fluent()
        .insert()
        .into(EXPENSE_COLLECTION_NAME)
        .document_id(&expense.id.to_string())
        .object(&expense)
        .execute()
        .await?;
    Ok(())
}
