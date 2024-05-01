use crate::Error::{CannotAccessFile, FileNameNotSpecified};
use crate::Result;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FilePath {
    pub file_name: String,
    inner: PathBuf,
}

impl FilePath {
    pub fn verify(path: impl AsRef<Path>) -> Result<Self> {
        let _ = fs::metadata(&path).map_err(|cause| CannotAccessFile {
            path: path.as_ref().to_path_buf(),
            cause,
        })?;

        let inner = PathBuf::from(path.as_ref());
        let Some(name) = inner.file_name() else {
            return Err(FileNameNotSpecified {
                path: path.as_ref().to_path_buf(),
            });
        };
        Ok(Self {
            file_name: name.to_string_lossy().to_string(),
            inner,
        })
    }
}

impl AsRef<Path> for FilePath {
    fn as_ref(&self) -> &Path {
        self.inner.as_ref()
    }
}
