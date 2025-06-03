use serde::{Deserialize, Serialize};
use crate::providers::common_types::{ChatRequestBase, ChatResponseBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqRequest {
    #[serde(flatten)]
    pub base: ChatRequestBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqResponse {
    #[serde(flatten)]
    pub base: ChatResponseBase,
}

pub use crate::providers::common_types::{Message, Choice, Usage};
