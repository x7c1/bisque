use crate::models::FilePath;
use crate::{here, Result};
use bisque_google_drive::schemas::FileName;
use std::io;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct DirPath(FilePath);

impl DirPath {
    pub fn verify(path: impl AsRef<Path>) -> Result<Self> {
        let file_path = FilePath::verify(path)?;
        Ok(Self(file_path))
    }

    pub fn create_file(&self, name: &FileName, mut reader: impl Read) -> Result<()> {
        let new_path = self.0.as_ref().join(name.escape_for_file_system());
        let mut file = std::fs::File::create(new_path).map_err(here!())?;
        io::copy(&mut reader, &mut file).map_err(here!())?;
        Ok(())
    }
}
