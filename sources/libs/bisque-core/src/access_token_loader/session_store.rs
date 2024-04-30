use crate::{here, Result};
use bisque_google_drive::oauth::refresh_access_token::SuccessResponse;
use bisque_google_drive::oauth::AccessToken;
use chrono::{Duration, Utc};
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
        let session = serde_json::from_slice::<Session>(&content).map_err(here!())?;
        println!("Session found. {:#?}", session);

        if session.is_expired()? {
            println!("Session is expired.");
            return Ok(None);
        }
        Ok(Some(session.response.access_token))
    }
    pub fn save_response(&self, response: SuccessResponse) -> Result<()> {
        let session = Session {
            response: response.clone(),
            created_at: Utc::now().to_rfc3339(),
        };
        let content = serde_json::to_vec_pretty(&session).map_err(here!())?;
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

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Session {
    response: SuccessResponse,
    created_at: String,
}

impl Session {
    fn is_expired(&self) -> Result<bool> {
        let created_at = chrono::DateTime::parse_from_rfc3339(&self.created_at).map_err(here!())?;
        let expired_at = created_at + Duration::seconds(self.response.expires_in.into());
        let now = Utc::now();
        Ok(expired_at < now)
    }
}
