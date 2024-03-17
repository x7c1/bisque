use std::fmt;
use std::fmt::Display;

#[derive(Debug, serde::Deserialize)]
pub struct AuthCode(String);

impl Display for AuthCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl AuthCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into().trim().into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
