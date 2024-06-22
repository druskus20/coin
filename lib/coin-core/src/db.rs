use crate::error::DatabaseError;
use firebase_rs::Firebase;

pub async fn try_connect(auth_key: &str) -> Result<Firebase, DatabaseError> {
    Ok(Firebase::auth(
        "https://myfirebase.firebaseio.com",
        auth_key,
    )?)
}
