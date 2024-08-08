use std::sync::Arc;

use crate::{
    error::{CoinError, DatabaseError},
    oauth::OAuthManager,
    utils::expiration_to_ts,
};
use firestore::{FirestoreDb, FirestoreDbOptions};
use firestore_db_and_auth::dto::SignInWithIdpRequest;
use gcloud_sdk::Token;
use reqwest::Response;
use serde::{Deserialize, Deserializer, Serialize};

async fn sign_in_by_oauth2(
    api_key: &str,
    access_token: String,
    request_uri: String,
) -> Result<gcloud_sdk::Token, CoinError> {
    let uri = "https://identitytoolkit.googleapis.com/v1/accounts:signInWithIdp?key=".to_owned()
        + api_key;

    let post_body = format!("access_token={}&providerId={}", access_token, "github.com",);
    let return_idp_credential = true;
    let return_secure_token = true;

    let json = &SignInWithIdpRequest {
        post_body,
        request_uri,
        return_idp_credential,
        return_secure_token,
    };

    let response: Response = reqwest::Client::new().post(&uri).json(&json).send().await?;

    let oauth_response: OAuthResponse = response.json().await?;

    // TODO - refresh token

    dbg!(&oauth_response);
    match oauth_response {
        OAuthResponse::SignInUpSuccess(r) => Ok(Token::new(
            "Bearer".into(),
            r.id_token.into(),
            expiration_to_ts(r.timestamp, r.expires_in)?,
        )),
        OAuthResponse::Error(_) => Err(DatabaseError::SignInError {
            code: None,
            message: None,
        })?,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OAuthResponse {
    Error(OAuthError),
    #[serde(untagged)]
    SignInUpSuccess(SignInUpSuccess),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthError {
    code: u64,
    message: String,
    errors: Vec<ErrorDetail>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetail {
    message: String,
    domain: String,
    reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInUpSuccess {
    pub federated_id: String,
    pub provider_id: String,
    pub local_id: String,
    pub email_verified: bool,
    pub email: Option<String>,
    pub oauth_access_token: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: String,
    pub display_name: String,
    pub id_token: String,
    pub photo_url: String,
    pub refresh_token: String,
    #[serde(deserialize_with = "deserialize_u64_from_string")]
    pub expires_in: u64,
    pub raw_user_info: String,
    #[serde(default = "timestamp")]
    pub timestamp: u64,
}

fn deserialize_u64_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<u64>().map_err(serde::de::Error::custom)
}

fn timestamp() -> u64 {
    chrono::Utc::now().timestamp() as u64
}

async fn retrieve_db_token(cm: Arc<OAuthManager>) -> gcloud_sdk::error::Result<gcloud_sdk::Token> {
    let gh_token = cm.access_token_or_refresh().await.unwrap();
    let api_key = todo!();
    let request_uri = "https://coin-ccc72.firebaseapp.com/__/auth/handler".to_string();

    let db_token = sign_in_by_oauth2(api_key, gh_token, request_uri)
        .await
        .unwrap();

    Ok(db_token)
}

pub async fn try_new(cm: &OAuthManager, project_id: String) -> Result<FirestoreDb, DatabaseError> {
    let cm = cm.arc_clone();
    // Create an instance
    let db = FirestoreDb::with_options_token_source(
        FirestoreDbOptions::new(project_id.clone()),
        gcloud_sdk::GCP_DEFAULT_SCOPES.clone(),
        gcloud_sdk::TokenSourceType::ExternalSource(Box::new(
            gcloud_sdk::ExternalJwtFunctionSource::new(move || retrieve_db_token(cm.clone())),
        )),
    )
    .await?;
    dbg!(&db);
    Ok(db)
}
