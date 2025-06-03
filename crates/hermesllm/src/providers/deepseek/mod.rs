pub mod types;

use thiserror::Error;

use crate::providers::deepseek::types::{DeepSeekRequest, DeepSeekResponse};

#[derive(Debug, Error)]
pub enum DeepSeekError {
    #[error("json error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, DeepSeekError>;

impl TryFrom<&[u8]> for DeepSeekRequest {
    type Error = DeepSeekError;
    fn try_from(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(DeepSeekError::from)
    }
}

impl TryFrom<&[u8]> for DeepSeekResponse {
    type Error = DeepSeekError;
    fn try_from(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(DeepSeekError::from)
    }
}
