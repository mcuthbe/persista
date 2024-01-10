use std::{error::Error, fmt};

use clipboard_win::ErrorCode;

pub enum PersistaError {
    WinCrate(ErrorCode),
}

impl fmt::Display for PersistaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PersistaError::WinCrate(error) => write!(f, "Error: {:?}", error),
        }
    }
}

impl fmt::Debug for PersistaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PersistaError::WinCrate(error) => write!(f, "Error: {:?}", error),
        }
    }
}

impl Error for PersistaError {}

impl From<ErrorCode> for PersistaError {
    fn from(error: ErrorCode) -> Self {
        PersistaError::WinCrate(error)
    }
}
