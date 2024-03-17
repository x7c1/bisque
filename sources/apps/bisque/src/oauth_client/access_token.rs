use std::fmt;
use std::fmt::Display;

#[derive(Debug, serde::Deserialize)]
pub struct AccessToken(String);

impl Display for AccessToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}
