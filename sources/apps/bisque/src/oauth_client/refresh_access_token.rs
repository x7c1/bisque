use crate::oauth_client::{AccessToken, OAuthClient, RefreshToken};
use crate::{here, Result};

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum RefreshAccessTokenResponse {
    Success {
        access_token: AccessToken,
        expires_in: u32,
        scope: String,
    },
    BadRequest {
        error: String,
        error_description: String,
    },
}

impl OAuthClient {
    /// https://developers.google.com/identity/protocols/oauth2/web-server#offline
    pub fn refresh_access_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<RefreshAccessTokenResponse> {
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("refresh_token", refresh_token.as_str()),
            ("grant_type", "refresh_token"),
        ];
        let response = self
            .client
            .post(Self::TOKEN_URL)
            .form(&params)
            .send()
            .map_err(here!())
            .inspect(|response| {
                println!(
                    "[refresh_access_token] Response status: {}",
                    response.status()
                );
            })?;

        let body = response.text().map_err(here!())?;
        let response = serde_json::from_str::<RefreshAccessTokenResponse>(&body)
            .map_err(here!())
            .inspect(|response| {
                println!("[refresh_access_token] Response body: {:#?}", response);
            })
            .inspect_err(|_| {
                println!("[refresh_access_token] Unexpected response: {}", body);
            })?;

        Ok(response)
    }
}
