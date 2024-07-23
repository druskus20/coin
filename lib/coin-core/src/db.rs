use std::sync::Arc;

use crate::{
    auth::CredentialsManager,
    error::{CoinError, DatabaseError},
};
use firestore::{FirestoreDb, FirestoreDbOptions};
use gcloud_sdk::{error::ErrorKind, Token};

async fn my_token(cm: Arc<CredentialsManager>) -> gcloud_sdk::error::Result<gcloud_sdk::Token> {
    let c = cm.access_token_or_refresh().await;
    match c {
        Ok((access_token, expires_in)) => Ok(Token::new(
            "Bearer".to_string(),
            access_token.into(),
            expires_in,
        )),

        Err(_) => Err(ErrorKind::TokenSource.into()),
    }
}
//async fn retrieve_access_token(cm: &mut CredentialsManager) -> gcloud_sdk::
//    let t = cm.access_token_or_refresh().await?.to_string();
//    Ok(t)
//}

pub async fn try_new(
    cm: &CredentialsManager,
    project_id: &str,
) -> Result<FirestoreDb, DatabaseError> {
    let cm = cm.arc_clone();
    // Create an instance
    let db = FirestoreDb::with_options_token_source(
        FirestoreDbOptions::new(project_id.to_string()),
        gcloud_sdk::GCP_DEFAULT_SCOPES.clone(),
        gcloud_sdk::TokenSourceType::ExternalSource(Box::new(
            gcloud_sdk::ExternalJwtFunctionSource::new(move || my_token(cm.clone())),
        )),
    )
    .await?;
    Ok(db)
}
