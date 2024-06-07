use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoinError {
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error(transparent)]
    GithubAuth(#[from] GithubAuthError),
    #[error(transparent)]
    Environment(#[from] std::env::VarError),
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

#[derive(Error, Debug)]
pub enum GithubAuthError {
    #[error(transparent)]
    UrlParseError(#[from] github_device_flow::DeviceFlowError),
}
