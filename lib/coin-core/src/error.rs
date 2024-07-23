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
    #[error("Environment variable not found error: {0}")]
    Environment(#[from] std::env::VarError),
    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),
    #[error("Invalid timestamp")]
    InvalidTimestamp,
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    FirestoreError(#[from] firestore::errors::FirestoreError),
    #[error(transparent)]
    GCDError(#[from] gcloud_sdk::error::Error),
}
