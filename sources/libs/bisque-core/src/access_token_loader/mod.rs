mod session_store;
use session_store::SessionStore;

use crate::error::Error;

use crate::{envs, here, Result};
use bisque_google_drive::oauth::refresh_access_token::Response::Success;
use bisque_google_drive::oauth::{AccessToken, AuthCode, OAuthClient, RefreshToken};
use std::io;
use std::path::PathBuf;

pub struct AccessTokenLoader {
    oauth_client: OAuthClient,
    session_store: SessionStore,
}

impl AccessTokenLoader {
    pub fn setup(session_path: impl Into<PathBuf>) -> Result<Self> {
        let client_id = envs::require("GOOGLE_CLIENT_ID")?;
        let client_secret = envs::require("GOOGLE_CLIENT_SECRET")?;

        let oauth_client = OAuthClient::setup(client_id, client_secret).map_err(here!())?;
        let store = SessionStore::new(session_path);
        Ok(Self {
            oauth_client,
            session_store: store,
        })
    }

    pub fn load(&self) -> Result<AccessToken> {
        if let Some(access_token) = self.session_store.find_access_token()? {
            return Ok(access_token);
        } else {
            println!("Access token is empty or expired.");
        }
        let refresh_token = self.find_refresh_token().transpose().unwrap_or_else(|| {
            println!("Refresh token is empty.");
            self.retrieve_refresh_token()
        })?;
        let response = self
            .oauth_client
            .refresh_access_token(&refresh_token)
            .map_err(here!())?;

        if let Success(response) = response {
            let access_token = response.access_token.clone();
            self.session_store.save_response(response)?;
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

        let response = self
            .oauth_client
            .refresh_access_token(&refresh_token)
            .map_err(here!())?;

        if let Success(response) = response {
            let access_token = response.access_token.clone();
            self.session_store.save_response(response)?;
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

        self.oauth_client
            .exchange_auth_code(&auth_code)
            .map_err(here!())
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
