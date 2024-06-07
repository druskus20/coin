mod auth;
pub mod currency;
pub mod currency_api;
mod db;
mod error;
mod expense;

type DateTime = chrono::DateTime<chrono::Utc>;

pub fn init() -> Result<(), error::CoinError> {
    let client_id = std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID not set");
    let auth = auth::GithubAuth::new(client_id.as_str(), None, None);
    let token = auth.authorize()?;
    dbg!(token);

    todo!()
}
