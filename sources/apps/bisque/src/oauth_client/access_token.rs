use std::fmt;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn reveal(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let visible_length = 8;
        let masked = {
            let start = self.0.len() - visible_length;
            format!("{:*>20}", &self.0[start..])
        };
        f.debug_tuple("AccessToken").field(&masked).finish()
    }
}
