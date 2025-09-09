use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub(crate) enum Error {
    #[error("Tauri Error: {0}")]
    TauriError(#[from] tauri::Error),

    #[error("Tauri Updater Error: {0}")]
    TauriUpdaterError(#[from] tauri_plugin_updater::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::error::Error),

    #[error("IO Error: {0}")]
    WebbrowserError(#[from] std::io::Error),

    // generic error just in case no other error is applicable
    #[error("Error: {0}")]
    Msg(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::Msg(s.to_owned())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Msg(s)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
