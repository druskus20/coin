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

#[tracing::instrument]
pub(super) async fn sign_in_up_oauth(access_token: String) -> Result<GoogleCredentials, CoinError> {
    tracing::info!("Signing in/up with google oauth");
    let api_key = crate::utils::env_var("COIN_GOOGLE_API_KEY")?;
    let oauth_request_uri = crate::utils::env_var("COIN_GOOGLE_REQUEST_URI")?;
    let uri = "https://identitytoolkit.googleapis.com/v1/accounts:signInWithIdp?key=".to_owned()
        + api_key.as_str();

    let post_body = format!("access_token={}&providerId={}", access_token, "github.com",);
    let return_idp_credential = true;
    let return_secure_token = true;

    let json = &SignInWithIdpRequest {
        post_body,
        request_uri: oauth_request_uri,
        return_idp_credential,
        return_secure_token,
    };

    let response: Response = reqwest::Client::new().post(&uri).json(&json).send().await?;
    let oauth_response: OAuthResponse = response.json().await?;

    match oauth_response {
        OAuthResponse::SignInUpSuccess(r) => {
            tracing::info!("Successfully signed in/up with google oauth");
            Ok(GoogleCredentials::from(r))
        }
        OAuthResponse::Error(e) => Err(DatabaseError::SignInError {
            code: Some(e.code),
            message: Some(e.message),
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

// TODO: transform automatically into coin database error
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct GoogleCredentials {
    pub local_id: String,
    pub federated_id: String,
    pub oauth_access_token: String,
    pub id_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub timestamp: u64,
}

impl GoogleCredentials {
    pub fn try_to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    pub fn try_from_string(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

impl From<SignInUpSuccess> for GoogleCredentials {
    fn from(s: SignInUpSuccess) -> Self {
        Self {
            local_id: s.local_id,
            federated_id: s.federated_id,
            oauth_access_token: s.oauth_access_token,
            id_token: s.id_token,
            refresh_token: s.refresh_token,
            expires_in: s.expires_in,
            timestamp: s.timestamp,
        }
    }
}
