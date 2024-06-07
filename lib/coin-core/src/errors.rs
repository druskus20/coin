use thiserror::Error;

pub type Result<T> = std::result::Result<T, CoinError>;

#[derive(Error, Debug)]
pub enum CoinError {}
