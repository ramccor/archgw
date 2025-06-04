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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StreamChoice {
    pub index: u32,
    pub delta: Message,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatCompletionStreamResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
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
