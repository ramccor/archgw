use serde::{Deserialize, Serialize};

/// Represents a request to the OpenAI API (compatible with both chat and completion endpoints).
///
/// Fields are based on the OpenAI API schema:
/// https://platform.openai.com/docs/api-reference/chat/create
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIRequest {
    /// The model to use (e.g., "gpt-3.5-turbo", "gpt-4").
    pub model: String,
    /// The list of messages for chat endpoints (use None for completion).
    pub messages: Option<Vec<Message>>,
    /// Sampling temperature to use (higher values = more random).
    pub temperature: Option<f32>,
    /// Nucleus sampling parameter.
    pub top_p: Option<f32>,
    /// How many completions to generate for each prompt/message.
    pub n: Option<u32>,
    /// Maximum number of tokens to generate.
    pub max_tokens: Option<u32>,
    /// Whether to stream back partial progress.
    pub stream: Option<bool>,
    /// Up to 4 sequences where the API will stop generating further tokens.
    pub stop: Option<Vec<String>>,
    /// Penalizes new tokens based on whether they appear in the text so far.
    pub presence_penalty: Option<f32>,
    /// Penalizes new tokens based on their frequency in the text so far.
    pub frequency_penalty: Option<f32>,
}

/// Builder for `OpenAIRequest`.
#[derive(Debug, Default, Clone)]
pub struct OpenAIRequestBuilder {
    model: String,
    messages: Option<Vec<Message>>,
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
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            ..Default::default()
        }
    }

    pub fn messages(mut self, messages: Vec<Message>) -> Self {
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

    pub fn build(self) -> OpenAIRequest {
        OpenAIRequest {
            model: self.model,
            messages: self.messages,
            temperature: self.temperature,
            top_p: self.top_p,
            n: self.n,
            max_tokens: self.max_tokens,
            stream: self.stream,
            stop: self.stop,
            presence_penalty: self.presence_penalty,
            frequency_penalty: self.frequency_penalty,
        }
    }
}

impl OpenAIRequest {
    pub fn builder(model: impl Into<String>) -> OpenAIRequestBuilder {
        OpenAIRequestBuilder::new(model)
    }
}

/// Represents a message in the OpenAI chat API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message sender ("system", "user", or "assistant").
    pub role: String,
    /// The content of the message.
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenAIResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
