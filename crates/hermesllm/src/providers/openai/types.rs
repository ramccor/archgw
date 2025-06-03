use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MultiPartContentType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image_url")]
    ImageUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultiPartContent {
    pub text: Option<String>,
    #[serde(rename = "type")]
    pub content_type: MultiPartContentType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ContentType {
    Text(String),
    MultiPart(Vec<MultiPartContent>),
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Text(text) => write!(f, "{}", text),
            ContentType::MultiPart(multi_part) => {
                let text_parts: Vec<String> = multi_part
                    .iter()
                    .filter_map(|part| {
                        if part.content_type == MultiPartContentType::Text {
                            part.text.clone()
                        } else if part.content_type == MultiPartContentType::ImageUrl {
                            // skip image URLs or their data in text representation
                            None
                        } else {
                            panic!("Unsupported content type: {:?}", part.content_type);
                        }
                    })
                    .collect();
                let combined_text = text_parts.join("\n");
                write!(f, "{}", combined_text)
            }
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: Option<ContentType>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamOptions {
  pub include_usage: bool,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionsRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub n: Option<u32>,
    pub max_tokens: Option<u32>,
    pub stream: Option<bool>,
    pub stop: Option<Vec<String>>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub stream_options: Option<StreamOptions>,
}

impl Default for ChatCompletionsRequest {
    fn default() -> Self {
        ChatCompletionsRequest {
            model: String::new(),
            messages: Vec::new(),
            temperature: None,
            top_p: None,
            n: None,
            max_tokens: None,
            stream: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            stream_options: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatCompletionsResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Debug, Clone)]
pub struct OpenAIRequestBuilder {
    model: String,
    messages: Vec<Message>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    n: Option<u32>,
    max_tokens: Option<u32>,
    stream: Option<bool>,
    stop: Option<Vec<String>>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    stream_options: Option<StreamOptions>,
}

impl OpenAIRequestBuilder {
    pub fn new(model: impl Into<String>, messages: Vec<Message>) -> Self {
        Self {
            model: model.into(),
            messages,
            temperature: None,
            top_p: None,
            n: None,
            max_tokens: None,
            stream: None,
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            stream_options: None,
        }
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

    pub fn stream_options(mut self, include_usage: bool) -> Self {
        self.stream = Some(true);
        self.stream_options = Some(StreamOptions { include_usage });
        self
    }

    pub fn build(self) -> Result<ChatCompletionsRequest, &'static str> {
        let request = ChatCompletionsRequest {
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
            stream_options: self.stream_options,
        };
        Ok(request)
    }
}

impl ChatCompletionsRequest {
    pub fn builder(model: impl Into<String>, messages: Vec<Message>) -> OpenAIRequestBuilder {
        OpenAIRequestBuilder::new(model, messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_type_display() {
        let text_content = ContentType::Text("Hello, world!".to_string());
        assert_eq!(text_content.to_string(), "Hello, world!");

        let multi_part_content = ContentType::MultiPart(vec![
            MultiPartContent {
                text: Some("This is a text part.".to_string()),
                content_type: MultiPartContentType::Text,
            },
            MultiPartContent {
                text: Some("https://example.com/image.png".to_string()),
                content_type: MultiPartContentType::ImageUrl,
            },
        ]);
        assert_eq!(multi_part_content.to_string(), "This is a text part.");
    }

    #[test]
    fn test_chat_completions_request_text_type_array() {
        const CHAT_COMPLETIONS_REQUEST: &str = r#"
        {
          "model": "gpt-3.5-turbo",
          "messages": [
            {
              "role": "user",
              "content": [
                {
                  "type": "text",
                  "text": "What city do you want to know the weather for?"
                },
                {
                  "type": "text",
                  "text": "hello world"
                }
              ]
            }
          ]
        }
        "#;

        let chat_completions_request: ChatCompletionsRequest =
            serde_json::from_str(CHAT_COMPLETIONS_REQUEST).unwrap();
        assert_eq!(chat_completions_request.model, "gpt-3.5-turbo");
        if let Some(ContentType::MultiPart(multi_part_content)) =
            chat_completions_request.messages[0].content.as_ref()
        {
            assert_eq!(multi_part_content.len(), 2);
            assert_eq!(
                multi_part_content[0].content_type,
                MultiPartContentType::Text
            );
            assert_eq!(
                multi_part_content[0].text,
                Some("What city do you want to know the weather for?".to_string())
            );
            assert_eq!(
                multi_part_content[1].content_type,
                MultiPartContentType::Text
            );
            assert_eq!(multi_part_content[1].text, Some("hello world".to_string()));
        } else {
            panic!("Expected MultiPartContent");
        }
    }
}
