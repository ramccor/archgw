//! hermesllm: A library for translating LLM API requests and responses
//! between Mistral, Grok, Gemini, and OpenAI-compliant formats.

pub mod providers;

#[cfg(test)]
mod tests {
    use crate::providers::openai::types::OpenAIRequest;

    #[test]
    fn openai_builder() {
        let request = OpenAIRequest::builder()
            .model("gpt-3.5-turbo")
            .temperature(0.7)
            .top_p(0.9)
            .n(1)
            .max_tokens(100)
            .stream(false)
            .stop(vec!["\n".to_string()])
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build()
            .expect("Failed to build OpenAIRequest");

        assert_eq!(request.base.model, "gpt-3.5-turbo");
        assert_eq!(request.base.temperature, Some(0.7));
        assert_eq!(request.base.top_p, Some(0.9));
        assert_eq!(request.base.n, Some(1));
        assert_eq!(request.base.max_tokens, Some(100));
        assert_eq!(request.base.stream, Some(false));
        assert_eq!(request.base.stop, Some(vec!["\n".to_string()]));
        assert_eq!(request.base.presence_penalty, Some(0.0));
        assert_eq!(request.base.frequency_penalty, Some(0.0));
    }
}
