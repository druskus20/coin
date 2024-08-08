use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoinError {
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error(transparent)]
    GithubAuth(#[from] github_device_oauth::DeviceFlowError),
    #[error(transparent)]
    Keyring(#[from] keyring::Error),
    #[error("No entry found in keyring")]
    KeyringNoEntry,
    #[error("Credentials serialization error: {0}")]
    CredentialsSerialization(#[from] serde_json::Error),
    #[error("Environment variable error: {key}")]
    Environment {
        source: std::env::VarError,
        key: String,
    },
    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),
    #[error("Invalid timestamp")]
    InvalidTimestamp,
    #[error("Firestore error: {0}")]
    FirestoreError(#[from] firestore::errors::FirestoreError),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    FirestoreError(#[from] firestore::errors::FirestoreError),
    #[error(transparent)]
    GCDError(#[from] gcloud_sdk::error::Error),
    #[error("Sign in error")]
    SignInError {
        code: Option<i32>,
        message: Option<String>,
    },
    #[error("Unknown")]
    UnknownError,
}
