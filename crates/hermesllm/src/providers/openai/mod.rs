pub mod types;

use thiserror::Error;

use crate::providers::openai::types::{OpenAIRequest, OpenAIResponse};

#[derive(Debug, Error)]
pub enum OpenAIError {
    #[error("json error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, OpenAIError>;

impl TryFrom<&[u8]> for OpenAIRequest {
    type Error = OpenAIError;
    fn try_from(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(OpenAIError::from)
    }
}

impl TryFrom<&[u8]> for OpenAIResponse {
    type Error = OpenAIError;
    fn try_from(bytes: &[u8]) -> Result<Self> {
        serde_json::from_slice(bytes).map_err(OpenAIError::from)
    }
}
