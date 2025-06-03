use serde::{Deserialize, Serialize};
use crate::providers::common_types::{ChatRequestBase, ChatResponseBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekRequest {
    #[serde(flatten)]
    pub base: ChatRequestBase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekResponse {
    #[serde(flatten)]
    pub base: ChatResponseBase,
}

// Re-export for convenience
pub use crate::providers::common_types::{Message, Choice, Usage};
