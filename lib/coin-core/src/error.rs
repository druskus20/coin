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
    #[error(transparent)]
    Environment(#[from] std::env::VarError),
    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error(transparent)]
    UrlParseError(#[from] firebase_rs::UrlParseError),
    #[error(transparent)]
    RequestError(#[from] firebase_rs::RequestError),
    #[error(transparent)]
    ServerEventError(#[from] firebase_rs::ServerEventError),
}
