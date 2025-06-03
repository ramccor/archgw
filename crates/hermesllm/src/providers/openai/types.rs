use serde::{Deserialize, Serialize};
use crate::providers::common_types::{ChatRequestBase, ChatResponseBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIRequest {
    #[serde(flatten)]
    pub base: ChatRequestBase,
}
#[derive(Debug, Default, Clone)]
pub struct OpenAIRequestBuilder {
    model: Option<String>,
    messages: Option<Vec<crate::providers::common_types::Message>>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    n: Option<u32>,
    max_tokens: Option<u32>,
    stream: Option<bool>,
    stop: Option<Vec<String>>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
}

impl OpenAIRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn messages(mut self, messages: Vec<crate::providers::common_types::Message>) -> Self {
        self.messages = Some(messages);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn build(self) -> Result<OpenAIRequest, &'static str> {
        let model = self.model.ok_or("model is required")?;
        let base = crate::providers::common_types::ChatRequestBase {
            model,
            messages: self.messages,
            temperature: self.temperature,
            top_p: self.top_p,
            n: self.n,
            max_tokens: self.max_tokens,
            stream: self.stream,
            stop: self.stop,
            presence_penalty: self.presence_penalty,
            frequency_penalty: self.frequency_penalty,
        };
        Ok(OpenAIRequest { base })
    }
}

impl OpenAIRequest {
    pub fn builder() -> OpenAIRequestBuilder {
        OpenAIRequestBuilder::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIResponse {
    #[serde(flatten)]
    pub base: ChatResponseBase,
}

pub use crate::providers::common_types::{Message, Choice, Usage};
