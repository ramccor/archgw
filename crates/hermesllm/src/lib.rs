//! hermesllm: A library for translating LLM API requests and responses
//! between Mistral, Grok, Gemini, and OpenAI-compliant formats.

/// Supported LLM providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Mistral,
    Grok,
    Gemini,
    OpenAI,
}

/// OpenAI API request format (placeholder).
#[derive(Debug, Clone)]
pub struct OpenAIRequest {
    // Add OpenAI request fields here
    pub prompt: String,
    // ...
}

/// OpenAI API response format (placeholder).
#[derive(Debug, Clone)]
pub struct OpenAIResponse {
    // Add OpenAI response fields here
    pub completion: String,
    // ...
}

/// Mistral API request format (placeholder).
#[derive(Debug, Clone)]
pub struct MistralRequest {
    pub input: String,
    // ...
}

/// Mistral API response format (placeholder).
#[derive(Debug, Clone)]
pub struct MistralResponse {
    pub output: String,
    // ...
}

/// Grok API request format (placeholder).
#[derive(Debug, Clone)]
pub struct GrokRequest {
    pub message: String,
    // ...
}

/// Grok API response format (placeholder).
#[derive(Debug, Clone)]
pub struct GrokResponse {
    pub reply: String,
    // ...
}

/// Gemini API request format (placeholder).
#[derive(Debug, Clone)]
pub struct GeminiRequest {
    pub query: String,
    // ...
}

/// Gemini API response format (placeholder).
#[derive(Debug, Clone)]
pub struct GeminiResponse {
    pub answer: String,
    // ...
}

/// Trait for translating provider-specific requests to OpenAI format.
pub trait ToOpenAIRequest {
    fn to_openai(&self) -> OpenAIRequest;
}

/// Trait for translating OpenAI responses to provider-specific format.
pub trait FromOpenAIResponse: Sized {
    fn from_openai(resp: &OpenAIResponse) -> Self;
}

// Implementations for Mistral
impl ToOpenAIRequest for MistralRequest {
    fn to_openai(&self) -> OpenAIRequest {
        OpenAIRequest {
            prompt: self.input.clone(),
        }
    }
}
impl FromOpenAIResponse for MistralResponse {
    fn from_openai(resp: &OpenAIResponse) -> Self {
        MistralResponse {
            output: resp.completion.clone(),
        }
    }
}

// Implementations for Grok
impl ToOpenAIRequest for GrokRequest {
    fn to_openai(&self) -> OpenAIRequest {
        OpenAIRequest {
            prompt: self.message.clone(),
        }
    }
}
impl FromOpenAIResponse for GrokResponse {
    fn from_openai(resp: &OpenAIResponse) -> Self {
        GrokResponse {
            reply: resp.completion.clone(),
        }
    }
}

// Implementations for Gemini
impl ToOpenAIRequest for GeminiRequest {
    fn to_openai(&self) -> OpenAIRequest {
        OpenAIRequest {
            prompt: self.query.clone(),
        }
    }
}
impl FromOpenAIResponse for GeminiResponse {
    fn from_openai(resp: &OpenAIResponse) -> Self {
        GeminiResponse {
            answer: resp.completion.clone(),
        }
    }
}

// Optionally, add more conversion traits as needed for bidirectional translation.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mistral_to_openai_and_back() {
        let mistral_req = MistralRequest { input: "Hello".into() };
        let openai_req = mistral_req.to_openai();
        assert_eq!(openai_req.prompt, "Hello");

        let openai_resp = OpenAIResponse { completion: "Hi!".into() };
        let mistral_resp = MistralResponse::from_openai(&openai_resp);
        assert_eq!(mistral_resp.output, "Hi!");
    }
}
