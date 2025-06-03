pub mod openai;
pub mod groq;

/// Supported LLM providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Grok,
    OpenAI,
}
