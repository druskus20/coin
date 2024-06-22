use github_device_oauth::{Credentials, DeviceFlow};

use crate::error::CoinError;
use keyring::Entry;

const HOST: &str = "github.com";
const SCOPES: &str = "read:user";
const APP_NAME: &str = "coin";

async fn refresh_or_authorize_github(
    refresh_token: Option<String>,
) -> Result<Credentials, CoinError> {
    #[cfg(debug_assertions)]
    match &refresh_token {
        Some(refresh_token) => println!(
            "Refreshing access token with refresh token {}",
            refresh_token
        ),
        None => println!("Authorizing access token"),
    }
    let client_id = std::env::var("COIN_GITHUB_CLIENT_ID")?;
    Ok(
        DeviceFlow::new(client_id, HOST.to_string(), SCOPES.to_string())
            .refresh_or_authorize(refresh_token)
            .await?,
    )
}

async fn store_credentials(credentials: Credentials) -> Result<(), CoinError> {
    #[cfg(debug_assertions)]
    println!("Storing credentials");
    let username = whoami::username();
    let entry = Entry::new(APP_NAME, &username)?;
    tokio::task::spawn_blocking(move || -> Result<(), CoinError> {
        entry.set_password(&credentials.try_to_string()?)?;
        Ok(())
    })
    .await??;

    Ok(())
}

async fn read_credentials() -> Result<Credentials, CoinError> {
    let username = whoami::username();
    let entry = Entry::new(APP_NAME, &username)?;

    let password = tokio::task::spawn_blocking(move || -> Result<String, CoinError> {
        match entry.get_password() {
            Ok(password) => Ok(password),
            Err(keyring::Error::NoEntry) => Err(CoinError::KeyringNoEntry),
            Err(err) => Err(err.into()),
        }
    })
    .await??;

    Ok(Credentials::try_from_string(&password)?)
}

pub struct CredentialsManager {
    credentials: Credentials,
}

impl CredentialsManager {
    pub async fn try_init_with_github() -> Result<Self, CoinError> {
        let credentials = match read_credentials().await {
            Ok(credentials) => refresh_or_authorize_github(Some(credentials.refresh_token)).await?,
            Err(CoinError::KeyringNoEntry) => refresh_or_authorize_github(None).await?,
            Err(err) => return Err(err),
        };
        store_credentials(credentials.clone()).await?;
        Ok(CredentialsManager { credentials })
    }

    pub async fn access_token_or_refresh(&mut self) -> Result<&str, CoinError> {
        if is_expired(&self.credentials) {
            self.credentials = refresh_or_authorize_github(None).await?;
            store_credentials(self.credentials.clone()).await?;
        }
        Ok(&self.credentials.access_token)
    }
}

const EXPIRATION_MARGIN: i64 = 120;
fn is_expired(credentials: &Credentials) -> bool {
    let ts = credentials.timestamp as i64;
    let expires_in = credentials.expires_in as i64;
    let now = chrono::Utc::now().timestamp();
    ts + expires_in < now + EXPIRATION_MARGIN
}
