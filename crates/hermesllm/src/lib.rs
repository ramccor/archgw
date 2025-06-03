//! hermesllm: A library for translating LLM API requests and responses
//! between Mistral, Grok, Gemini, and OpenAI-compliant formats.

pub mod providers;

#[cfg(test)]
mod tests {
    use crate::providers::openai::types::OpenAIRequest;

    #[test]
    fn openai_builder() {
        let request = OpenAIRequest::builder("gpt-3.5-turbo")
            .temperature(0.7)
            .top_p(0.9)
            .n(1)
            .max_tokens(100)
            .stream(false)
            .stop(vec!["\n".to_string()])
            .presence_penalty(0.0)
            .frequency_penalty(0.0)
            .build();

        assert_eq!(request.model, "gpt-3.5-turbo");
        assert_eq!(request.temperature, Some(0.7));
        assert_eq!(request.top_p, Some(0.9));
        assert_eq!(request.n, Some(1));
        assert_eq!(request.max_tokens, Some(100));
        assert_eq!(request.stream, Some(false));
        assert_eq!(request.stop, Some(vec!["\n".to_string()]));
        assert_eq!(request.presence_penalty, Some(0.0));
        assert_eq!(request.frequency_penalty, Some(0.0));
    }
}
