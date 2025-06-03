pub mod types;

use thiserror::Error;

use crate::providers::groq::types::{GroqRequest, GroqResponse};

#[derive(Debug, Error)]
pub enum GroqError {
    #[error("json error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, GroqError>;

impl TryFrom<&[u8]> for GroqRequest {
    type Error = GroqError;
    fn try_from(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(GroqError::from)
    }
}

impl TryFrom<&[u8]> for GroqResponse {
    type Error = GroqError;
    fn try_from(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(GroqError::from)
    }
}
