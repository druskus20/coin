use crate::{auth::GithubAuth, error::DatabaseError};
use firebase_rs::Firebase;

fn try_connect(auth_key: &str) -> Result<Firebase, DatabaseError> {
    Ok(Firebase::auth(
        "https://myfirebase.firebaseio.com",
        auth_key,
    )?)
}
