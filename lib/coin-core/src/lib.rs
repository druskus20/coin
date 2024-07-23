use std::{future::Future, sync::Arc};

use auth::CredentialsManager;
use error::DatabaseError;
use serde::{Deserialize, Serialize};

pub mod auth;
pub mod currency;
pub mod currency_api;
mod db;
mod error;
mod expense;

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TestRecord {
    name: String,
    value: f64,
}

pub async fn init() -> Result<(), error::CoinError> {
    let cm = auth::CredentialsManager::try_init_with_github().await?;
    let database_id = &std::env::var("COIN_FIRESTORE_PROJECT_ID")?;
    let db = db::try_new(&cm, database_id).await?;
    dbg!(db);
    Ok(())
}
