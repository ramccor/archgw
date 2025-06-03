pub mod openai;
pub mod groq;
pub mod deepseek;
pub mod common_types;

/// Supported LLM providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Groq,
    OpenAI,
    DeepSeek,
}
