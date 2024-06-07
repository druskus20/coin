use github_device_flow::{Credential, DeviceFlowError};

use crate::error::{self, GithubAuthError};

pub(crate) struct GithubAuth {
    client_id: String,
    host: Option<String>,
    refresh_token: Option<String>,
}

impl GithubAuth {
    pub fn new(client_id: &str, host: Option<&str>, refresh_token: Option<&str>) -> Self {
        Self {
            client_id: client_id.to_string(),
            host: host.map(|s| s.to_string()),
            refresh_token: refresh_token.map(|s| s.to_string()),
        }
    }
    pub fn authorize(&self) -> Result<Credential, GithubAuthError> {
        Ok(github_device_flow::authorize(
            self.client_id.clone(),
            self.host.clone(),
        )?)
    }

    pub fn refresh(&self) -> Result<Credential, GithubAuthError> {
        if let Some(rt) = &self.refresh_token {
            Ok(github_device_flow::refresh(
                self.client_id.as_str(),
                rt.as_str(),
                self.host.clone(),
            )?)
        } else {
            Err(DeviceFlowError::GitHubError("No refresh token found".to_string()).into())
        }
    }
}
