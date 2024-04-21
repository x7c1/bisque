use crate::error::Error;
use crate::oauth_client::{
    AccessToken, AuthCode, OAuthClient, RefreshAccessTokenResponse, RefreshToken,
};
use crate::{envs, here, Result};
use std::io;
use RefreshAccessTokenResponse::Success;

pub struct AccessTokenLoader {
    oauth_client: OAuthClient,
}

impl AccessTokenLoader {
    pub fn setup() -> Result<Self> {
        let oauth_client = OAuthClient::setup()?;
        Ok(Self { oauth_client })
    }

    pub fn load(&self) -> Result<AccessToken> {
        let refresh_token = self.find_refresh_token().transpose().unwrap_or_else(|| {
            println!("Refresh token is empty");
            self.retrieve_refresh_token()
        })?;
        let response = self.oauth_client.refresh_access_token(&refresh_token)?;
        if let Success { access_token, .. } = response {
            return Ok(access_token);
        }
        println!("Maybe refresh token is expired.");
        let refresh_token = self.retrieve_refresh_token()?;

        println!(
            "Use new refresh token then press any key.\n{}",
            refresh_token
        );
        let mut _key = String::new();
        io::stdin().read_line(&mut _key).map_err(here!())?;

        let response = self.oauth_client.refresh_access_token(&refresh_token)?;
        if let Success { access_token, .. } = response {
            return Ok(access_token);
        }
        println!("Failed to refresh access token.");
        Err(Error::RefreshAccessToken)
    }

    fn retrieve_refresh_token(&self) -> Result<RefreshToken> {
        println!("Go to the following link in your browser:");
        println!("{}", self.oauth_client.get_auth_url());

        println!("Enter the authorization code:");
        let auth_code = self.scan_auth_code()?;

        self.oauth_client.exchange_auth_code(&auth_code)
    }

    fn scan_auth_code(&self) -> Result<AuthCode> {
        let mut auth_code = String::new();
        io::stdin().read_line(&mut auth_code).map_err(here!())?;

        Ok(AuthCode::new(auth_code))
    }

    fn find_refresh_token(&self) -> Result<Option<RefreshToken>> {
        let token = envs::find("GOOGLE_REFRESH_TOKEN")?.map(RefreshToken::new);
        Ok(token)
    }
}
