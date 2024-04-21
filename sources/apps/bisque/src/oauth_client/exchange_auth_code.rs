use crate::here;
use crate::oauth_client::{AccessToken, AuthCode, OAuthClient, RefreshToken};

#[derive(Debug, serde::Deserialize)]
pub struct ExchangeAuthCodeResponse {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}

impl OAuthClient {
    /// https://developers.google.com/identity/protocols/oauth2/web-server#exchange-authorization-code
    pub fn exchange_auth_code(&self, auth_code: &AuthCode) -> crate::Result<RefreshToken> {
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("code", auth_code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", Self::REDIRECT_URI),
        ];
        let response = self
            .client
            .post(Self::TOKEN_URL)
            .form(&params)
            .send()
            .map_err(here!())
            .inspect(|response| {
                println!(
                    "[exchange_auth_code] Response status: {}",
                    response.status()
                );
            })?;

        let response = response
            .json::<ExchangeAuthCodeResponse>()
            .map_err(here!())
            .inspect(|response| {
                println!("[exchange_auth_code] Response body: {:#?}", response);
            })?;

        Ok(response.refresh_token)
    }
}
