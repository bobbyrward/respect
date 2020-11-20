use thiserror::Error;

pub type Result<T> = std::result::Result<T, RespectError>;

#[derive(Error, Debug)]
pub enum RespectError {
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl RespectError {
    pub fn unknown(message: &str) -> Self {
        Self::Unknown(message.to_string())
    }
}
