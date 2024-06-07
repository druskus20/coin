use std::collections::HashMap;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GithubDeviceOAuthError {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error("Field {} not found in response", .0)]
    FieldNotFoundError(String),
    #[error("Responded with error: {}", .0)]
    ResponseError(String),
    #[error("Awaiting response, too many attempts")]
    MaxAttemptsError,
    #[error("Awaiting response, too many attempts")]
    ExpiredAccessTokenError,
}

async fn send_request(
    url: impl AsRef<str>,
    body: String,
) -> Result<HashMap<String, serde_json::Value>, GithubDeviceOAuthError> {
    let client = reqwest::Client::new();
    let response = client
        .post(url.as_ref())
        .header("Accept", "application/json")
        .body(body)
        .send()
        .await?
        .json::<HashMap<String, serde_json::Value>>()
        .await?;

    if response.contains_key("error") && response.contains_key("error_description") {
        let err = extract_string(&response, "error_description")?;
        return Err(GithubDeviceOAuthError::ResponseError(err));
    } else if response.contains_key("error") {
        let err = extract_string(&response, "error")?;
        return Err(GithubDeviceOAuthError::ResponseError(err));
    }

    Ok(response)
}

fn extract_string(
    json_response: &HashMap<String, serde_json::Value>,
    key: &str,
) -> Result<String, GithubDeviceOAuthError> {
    Ok(json_response
        .get(key)
        .ok_or(GithubDeviceOAuthError::FieldNotFoundError(key.to_owned()))?
        .to_string())
}

//#[derive(Debug, Default, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
//pub struct Credential {
//    pub token: String,
//    pub expiry: String,
//    pub refresh_token: String,
//}

struct GithubDeviceOAuthFlow {
    client_id: String,
    host: String,
    scopes: String,
    verification_params: VerificationParams,
}

struct Credentials {
    access_token: String,
    access_token_expiration: String,
    refresh_token: String,
}

struct VerificationParams {
    device_code: String,
    user_code: String,
    verification_uri: String,
}

impl GithubDeviceOAuthFlow {
    pub async fn try_verify_device(
        client_id: String,
        host: String,
        scopes: String,
    ) -> Result<Self, GithubDeviceOAuthError> {
        let verification_params =
            Self::request_verification_params(&client_id, &host, &scopes).await?;

        Ok(Self {
            client_id,
            host,
            scopes,
            verification_params,
        })
    }

    async fn request_verification_params(
        client_id: &str,
        host: &str,
        scopes: &str,
    ) -> Result<VerificationParams, GithubDeviceOAuthError> {
        let r = send_request(
            format!("https:/{}/login/device/code", host),
            format!("client_id={}&scope={}", client_id, scopes),
        )
        .await?;

        let device_code = extract_string(&r, "device_code")?;
        let user_code = extract_string(&r, "user_code")?;
        let verification_uri = extract_string(&r, "verification_uri")?;

        Ok(VerificationParams {
            device_code,
            user_code,
            verification_uri,
        })
    }

    pub async fn try_authorize(&self) -> Result<Credentials, GithubDeviceOAuthError> {
        match self.request_access().await {
            Ok(credentials) => Ok(credentials),
            Err(GithubDeviceOAuthError::ExpiredAccessTokenError) => self.request_refresh().await,
            Err(other) => Err(other),
        }
    }

    async fn request_access(&self) -> Result<Credentials, GithubDeviceOAuthError> {
        let request_url = format!("https:/{}/login/oauth/access_token", self.host);
        let request_body = format!(
            "client_id={}&device_code={}&grant_type=urn:ietf:params:oauth:grant-type:device_code",
            self.client_id, self.verification_params.device_code
        );
        let r = send_request(request_url, request_body).await?;

        todo!()
    }

    async fn request_refresh(&self) -> Result<Credentials, GithubDeviceOAuthError> {
        todo!()
    }
}
