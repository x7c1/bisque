use std::fmt;
use std::fmt::Display;

#[derive(Debug, serde::Deserialize)]
pub struct RefreshToken(String);

impl Display for RefreshToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl RefreshToken {
    pub fn new(token: impl Into<String>) -> Self {
        Self(token.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
