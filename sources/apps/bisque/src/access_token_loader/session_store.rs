use crate::oauth_client::{AccessToken, RefreshAccessTokenSuccessResponse};
use crate::{here, Result};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

pub struct SessionStore {
    cache_path: PathBuf,
}

impl SessionStore {
    pub fn new(cache_path: impl Into<PathBuf>) -> Self {
        Self {
            cache_path: cache_path.into(),
        }
    }
    pub fn find_access_token(&self) -> Result<Option<AccessToken>> {
        let Some(content) = self.load_cache()? else {
            return Ok(None);
        };
        let response = serde_json::from_slice::<RefreshAccessTokenSuccessResponse>(&content)
            .map_err(here!())?;

        println!("Session found. {:#?}", response);
        Ok(Some(response.access_token))
    }
    pub fn save_response(&self, response: &RefreshAccessTokenSuccessResponse) -> Result<()> {
        let content = serde_json::to_vec_pretty(response).map_err(here!())?;
        fs::write(&self.cache_path, content).map_err(here!())?;
        Ok(())
    }
    fn load_cache(&self) -> Result<Option<Vec<u8>>> {
        let error = match fs::read(&self.cache_path) {
            Ok(content) => return Ok(Some(content)),
            Err(cause) => cause,
        };
        match error.kind() {
            ErrorKind::NotFound => Ok(None),
            _ => Err(error).map_err(here!())?,
        }
    }
}
