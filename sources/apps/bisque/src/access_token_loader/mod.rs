use crate::oauth_client::{AccessToken, AuthCode, OAuthClient, RefreshToken};
use crate::Result;
use std::env::VarError;
use std::{env, io};

pub struct AccessTokenLoader {
    oauth_client: OAuthClient,
}

impl AccessTokenLoader {
    pub fn setup() -> Result<Self> {
        let oauth_client = OAuthClient::setup()?;
        Ok(Self { oauth_client })
    }

    pub fn load(&self) -> Result<AccessToken> {
        let refresh_token = self.retrieve_refresh_token()?;
        let access_token = self.oauth_client.refresh_access_token(&refresh_token)?;
        Ok(access_token)
    }

    fn retrieve_refresh_token(&self) -> Result<RefreshToken> {
        if let Some(token) = self.find_refresh_token()? {
            return Ok(token);
        }
        println!("refresh token is empty");
        println!("Go to the following link in your browser:");
        println!("{}", self.oauth_client.get_auth_url());

        println!("Enter the authorization code:");
        let auth_code = self.scan_auth_code()?;

        self.oauth_client.exchange_auth_code(&auth_code)
    }
    fn scan_auth_code(&self) -> Result<AuthCode> {
        let mut auth_code = String::new();
        io::stdin().read_line(&mut auth_code)?;
        Ok(AuthCode::new(auth_code))
    }

    fn find_refresh_token(&self) -> Result<Option<RefreshToken>> {
        let refresh_token = match env::var("GOOGLE_REFRESH_TOKEN") {
            Ok(token) if token.is_empty() => None,
            Ok(token) => Some(RefreshToken::new(token)),
            Err(VarError::NotPresent) => None,
            Err(other) => return Err(other.into()),
        };
        Ok(refresh_token)
    }
}
