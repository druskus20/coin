use crate::{error::CoinError, DateTime};

const EXPIRATION_MARGIN: i64 = 120;
pub(crate) fn expiration_to_ts(timestamp: u64, expires_in: u64) -> Result<DateTime, CoinError> {
    DateTime::from_timestamp((timestamp + expires_in) as i64, 0).ok_or(CoinError::InvalidTimestamp)
}

pub(crate) fn is_expired(timestamp: u64, expires_in: u64) -> bool {
    let now = chrono::Utc::now().timestamp();
    ((timestamp + expires_in) as i64) < now + EXPIRATION_MARGIN
}

pub(crate) fn env_var(key: &str) -> Result<String, CoinError> {
    std::env::var(key).map_err(|err| CoinError::Environment {
        source: err,
        key: key.to_string(),
    })
}
