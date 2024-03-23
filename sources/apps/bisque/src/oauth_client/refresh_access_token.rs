use crate::oauth_client::{AccessToken, OAuthClient, RefreshToken};
use crate::{here, Result};

#[derive(Debug, serde::Deserialize)]
pub struct RefreshAccessTokenResponse {
    pub access_token: AccessToken,
    pub expires_in: u32,
    pub scope: String,
}

impl OAuthClient {
    /// https://developers.google.com/identity/protocols/oauth2/web-server#offline
    pub fn refresh_access_token(&self, refresh_token: &RefreshToken) -> Result<AccessToken> {
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

        let response = response
            .json::<RefreshAccessTokenResponse>()
            .map_err(here!())
            .inspect(|response| {
                println!("[refresh_access_token] Response body: {:#?}", response);
            })?;

        Ok(response.access_token)
    }
}
