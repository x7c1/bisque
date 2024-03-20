mod access_token;
pub use access_token::AccessToken;

mod exchange_auth_code;
mod refresh_access_token;

mod refresh_token;
pub use refresh_token::RefreshToken;

mod auth_code;
pub use auth_code::AuthCode;

use crate::Result;

/// https://developers.google.com/identity/protocols/oauth2/web-server
pub struct OAuthClient {
    client: reqwest::blocking::Client,
    client_id: String,
    client_secret: String,
}

impl OAuthClient {
    const TOKEN_URL: &'static str = "https://oauth2.googleapis.com/token";
    const AUTH_URL: &'static str = "https://accounts.google.com/o/oauth2/auth";
    const REDIRECT_URI: &'static str = "urn:ietf:wg:oauth:2.0:oob";

    pub fn setup() -> Result<Self> {
        let client_id = std::env::var("GOOGLE_CLIENT_ID")?;
        let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")?;

        Ok(Self {
            client: reqwest::blocking::Client::new(),
            client_id,
            client_secret,
        })
    }
    pub fn get_auth_url(&self) -> String {
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope=https://www.googleapis.com/auth/drive",
            Self::AUTH_URL,
            self.client_id,
            Self::REDIRECT_URI
        )
    }
}
