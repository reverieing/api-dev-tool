use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Request timeout")]
    Timeout,
}

pub type Result<T> = std::result::Result<T, Error>;