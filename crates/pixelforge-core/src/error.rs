use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("invalid ASE file: {0}")]
    InvalidAse(String),
    #[error("unsupported aseprite version: {0}")]
    UnsupportedVersion(u16),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
