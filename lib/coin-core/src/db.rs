use std::sync::Arc;

use crate::{
    error::{CoinError, DatabaseError},
    oauth::OAuthManager,
    utils,
};
use firestore::{FirestoreDb, FirestoreDbOptions};
use gcloud_sdk::google;

pub struct Db {
    pub firestore_db: FirestoreDb,
    pub user_id: String,
}

impl Db {
    pub async fn try_new(om: &OAuthManager, project_id: String) -> Result<Self, CoinError> {
        let om = om.arc_clone();
        let (_, google_credentials) = om.credentials_or_refresh().await?;
        let firestore_db = FirestoreDb::with_options_token_source(
            FirestoreDbOptions::new(project_id.clone()),
            gcloud_sdk::GCP_DEFAULT_SCOPES.clone(),
            gcloud_sdk::TokenSourceType::ExternalSource(Box::new(
                gcloud_sdk::ExternalJwtFunctionSource::new(move || retrieve_db_token(om.clone())),
            )),
        )
        .await?;

        Ok(Self {
            firestore_db,
            user_id: google_credentials.local_id,
        })
    }
}

async fn retrieve_db_token(om: Arc<OAuthManager>) -> gcloud_sdk::error::Result<gcloud_sdk::Token> {
    let (_, google_credentials) = om.credentials_or_refresh().await.unwrap(); // TODO: do not unwrap
    let expiry =
        utils::expiration_to_ts(google_credentials.timestamp, google_credentials.expires_in)
            .unwrap(); // TODO: Do not unwrap

    Ok(gcloud_sdk::Token {
        token_type: "Bearer".into(),
        token: google_credentials.id_token.into(),
        expiry,
    })
}
