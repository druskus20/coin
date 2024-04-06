use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] surrealdb::Error),
    #[error(transparent)]
    Environment(#[from] std::env::VarError),
}
