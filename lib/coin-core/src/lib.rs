use db::Db;
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

#[tracing::instrument]
pub async fn init() -> Result<Db, error::CoinError> {
    let cm = oauth::OAuthManager::try_init_github_google().await?;
    let project_id = utils::env_var("COIN_FIRESTORE_PROJECT_ID")?;
    let db = Db::try_new(&cm, project_id.clone()).await?;
    Ok(db)
}

pub async fn add_expense(db: &Db, expense: Expense) -> Result<(), error::CoinError> {
    let _ = db
        .firestore_db
        .fluent()
        .insert()
        .into(format!("{}/EXPENSE_COLLECTION_NAME", db.user_id).as_str())
        .document_id(&expense.id.to_string())
        .object(&expense)
        .execute()
        .await?;
    Ok(())
}
