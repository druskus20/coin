mod google;

use firestore_db_and_auth::FirebaseAuthBearer;
use std::sync::Arc;
use tokio::sync::Mutex;

use chrono::{DateTime, Utc};
use github_device_oauth::{Credentials as GithubCredentials, DeviceFlow};

use crate::{
    error::CoinError,
    utils::{self, is_expired},
};
use keyring::Entry;

use self::google::GoogleCredentials;

const HOST: &str = "github.com";
const SCOPES: &str = "read:user";
const APP_NAME: &str = "coinv";

#[derive(Debug, Clone)]
pub(crate) struct OAuthManager {
    pub idp_credentials: Arc<Mutex<GithubCredentials>>, // Todo remove arc?
    pub google_credentials: GoogleCredentials,
}

impl OAuthManager {
    pub fn arc_clone(&self) -> Arc<Self> {
        Arc::new(self.clone())
    }

    #[tracing::instrument]
    pub async fn try_init_github_google() -> Result<Self, CoinError> {
        tracing::info!("Trying to initialize OAuthManager with Github");
        let (gh_credentials, google_credentials) = match read_credentials_keyring().await {
            Ok((gh_credentials, _)) => {
                tracing::info!("Crednetials found in keyring");
                let gh_credentials =
                    refresh_or_authorize_github(Some(gh_credentials.refresh_token)).await?;
                let google_credentials =
                    google::sign_in_up_oauth(gh_credentials.access_token.clone()).await?;

                (gh_credentials, google_credentials)
            }
            Err(CoinError::KeyringNoEntry) => {
                tracing::info!("No credentials found in keyring");
                let gh_credentials = refresh_or_authorize_github(None).await?;
                let google_credentials =
                    google::sign_in_up_oauth(gh_credentials.access_token.clone()).await?;
                (gh_credentials, google_credentials)
            }

            Err(err) => return Err(err),
        };

        store_credentials_keyring(gh_credentials.clone(), google_credentials.clone()).await?;
        Ok(OAuthManager {
            idp_credentials: Arc::new(Mutex::new(gh_credentials)),
            google_credentials,
        })
    }

    #[tracing::instrument]
    pub async fn credentials_or_refresh(
        &self,
    ) -> Result<(GithubCredentials, GoogleCredentials), CoinError> {
        let mut gh_credentials = self.idp_credentials.lock().await;
        let google_credentials = self.google_credentials.clone();

        // refresh
        if is_expired(google_credentials.timestamp, google_credentials.expires_in)
            || is_expired(gh_credentials.timestamp, gh_credentials.expires_in)
        {
            let new_credentials = refresh_or_authorize_github(None).await?;
            *gh_credentials = new_credentials.clone();
            let google_credentials =
                google::sign_in_up_oauth(gh_credentials.access_token.clone()).await?;
            store_credentials_keyring(gh_credentials.clone(), google_credentials.clone()).await?;
        }

        Ok((gh_credentials.clone(), google_credentials.clone()))
    }
}

#[tracing::instrument]
async fn refresh_or_authorize_github(
    refresh_token: Option<String>,
) -> Result<GithubCredentials, CoinError> {
    match &refresh_token {
        Some(_) => tracing::info!("Refreshing with access token"),
        None => tracing::info!("No refresh token. Authorizing with device flow"),
    }
    let client_id = utils::env_var("COIN_GITHUB_CLIENT_ID")?;
    let credentials = DeviceFlow::new(client_id, HOST.to_string(), SCOPES.to_string())
        .refresh_or_authorize(refresh_token)
        .await?;
    tracing::info!("Authorization successful");
    Ok(credentials)
}

async fn store_credentials_keyring(
    gh_credentials: GithubCredentials,
    google_credentials: GoogleCredentials,
) -> Result<(), CoinError> {
    tracing::info!("Storing credentials in keyring");
    let username = whoami::username();

    let entry = Entry::new(&format!("{APP_NAME}_github"), &username)?;
    tokio::task::spawn_blocking(move || -> Result<(), CoinError> {
        entry.set_password(&gh_credentials.try_to_string()?)?;
        Ok(())
    })
    .await??;
    let entry = Entry::new(&format!("{APP_NAME}_google"), &username)?;
    tokio::task::spawn_blocking(move || -> Result<(), CoinError> {
        entry.set_password(&google_credentials.try_to_string()?)?;
        Ok(())
    })
    .await??;

    Ok(())
}

async fn read_credentials_keyring() -> Result<(GithubCredentials, GoogleCredentials), CoinError> {
    tracing::info!("Reading credentials from keyring");
    let username = whoami::username();

    let entry = Entry::new(&format!("{APP_NAME}_github"), &username)?;
    let gh_credentials = tokio::task::spawn_blocking(move || -> Result<String, CoinError> {
        match entry.get_password() {
            Ok(password) => Ok(password),
            Err(keyring::Error::NoEntry) => Err(CoinError::KeyringNoEntry),
            Err(err) => Err(err.into()),
        }
    })
    .await??;

    let entry = Entry::new(&format!("{APP_NAME}_google"), &username)?;
    let google_credentials = tokio::task::spawn_blocking(move || -> Result<String, CoinError> {
        match entry.get_password() {
            Ok(password) => Ok(password),
            Err(keyring::Error::NoEntry) => Err(CoinError::KeyringNoEntry),
            Err(err) => Err(err.into()),
        }
    })
    .await??;

    Ok((
        GithubCredentials::try_from_string(&gh_credentials)?,
        GoogleCredentials::try_from_string(&google_credentials)?,
    ))
}
