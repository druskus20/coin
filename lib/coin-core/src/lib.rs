pub mod auth;
pub mod currency;
pub mod currency_api;
mod db;
mod error;
mod expense;

type DateTime = chrono::DateTime<chrono::Utc>;

pub async fn init() -> Result<(), error::CoinError> {
    let mut cm = auth::CredentialsManager::try_init_with_github().await?;
    let access_token = cm.access_token_or_refresh().await?;
    let firebase = db::try_connect(access_token).await?;
    dbg!(firebase);

    Ok(())
}
