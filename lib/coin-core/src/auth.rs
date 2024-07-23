use std::sync::Arc;
use tokio::sync::Mutex;

use chrono::{DateTime, Utc};
use github_device_oauth::{Credentials, DeviceFlow};

use crate::error::CoinError;
use keyring::Entry;

const HOST: &str = "github.com";
const SCOPES: &str = "read:user";
const APP_NAME: &str = "coin-dev";

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

async fn store_credentials_keyring(credentials: Credentials) -> Result<(), CoinError> {
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

async fn read_credentials_keyring() -> Result<Credentials, CoinError> {
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

#[derive(Debug, Clone)]
pub struct CredentialsManager {
    credentials: Arc<Mutex<Credentials>>,
}

impl CredentialsManager {
    pub fn arc_clone(&self) -> Arc<Self> {
        Arc::new(self.clone())
    }

    pub async fn try_init_with_github() -> Result<Self, CoinError> {
        let credentials = match read_credentials_keyring().await {
            Ok(credentials) => refresh_or_authorize_github(Some(credentials.refresh_token)).await?,
            Err(CoinError::KeyringNoEntry) => refresh_or_authorize_github(None).await?,
            Err(err) => return Err(err),
        };
        store_credentials_keyring(credentials.clone()).await?;
        Ok(CredentialsManager {
            credentials: Arc::new(Mutex::new(credentials)),
        })
    }

    pub async fn access_token_or_refresh(&self) -> Result<(String, DateTime<Utc>), CoinError> {
        let mut credentials = self.credentials.lock().await;
        if is_expired(&credentials) {
            let new_credentials = refresh_or_authorize_github(None).await?;
            *credentials = new_credentials.clone();
            store_credentials_keyring(new_credentials).await?;
        }
        let expiration = expiration_datetime(&credentials)?;
        Ok((credentials.access_token.clone(), expiration))
    }
}

const EXPIRATION_MARGIN: i64 = 120;
fn expiration_datetime(credentials: &Credentials) -> Result<DateTime<Utc>, CoinError> {
    let ts = credentials.timestamp as i64;
    let expires_in = credentials.expires_in as i64;
    DateTime::from_timestamp(ts + expires_in, 0).ok_or(CoinError::InvalidTimestamp)
}
fn is_expired(credentials: &Credentials) -> bool {
    let ts = credentials.timestamp as i64;
    let expires_in = credentials.expires_in as i64;
    let now = chrono::Utc::now().timestamp();
    ts + expires_in < now + EXPIRATION_MARGIN
}
