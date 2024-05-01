use crate::Result;

#[derive(Clone, Debug)]
pub struct FileName(String);

impl FileName {
    pub fn new<S: Into<String>>(value: S) -> Result<Self> {
        Ok(Self(value.into()))
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn escape_for_drive_api(&self) -> String {
        self.0.replace('\\', "\\\\").replace('\'', "\\'")
    }

    /// According to the behavior of Google Drive web (browser),
    /// slash is replaced with underscore when file is downloaded.
    pub fn escape_for_file_system(&self) -> String {
        self.0.replace('/', "_")
    }
}
