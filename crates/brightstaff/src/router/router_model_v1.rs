use common::{
    api::open_ai::{ChatCompletionsRequest, Message},
    consts::{SYSTEM_ROLE, USER_ROLE},
};
use serde::{Deserialize, Serialize};
use tracing::info;

use super::router_model::{RouterModel, RoutingModelError};

pub const ARCH_ROUTER_V1_SYSTEM_PROMPT: &str = r#"
You are an advanced Routing Assistant designed to select the optimal route based on user requests.
Your task is to analyze conversations and match them to the most appropriate predefined route.
Review the available routes config:

# ROUTES CONFIG START
{routes}
# ROUTES CONFIG END

Examine the following conversation between a user and an assistant:

# CONVERSATION START
{conversation}
# CONVERSATION END

Your goal is to identify the most appropriate route that matches the user's LATEST intent. Follow these steps:

1. Carefully read and analyze the provided conversation, focusing on the user's latest request and the conversation scenario.
2. Check if the user's request and scenario matches any of the routes in the routing configuration (focus on the description).
3. Find the route that best matches.
4. Use context clues from the entire conversation to determine the best fit.
5. Return the best match possible. You only response the name of the route that best matches the user's request, use the exact name in the routes config.
6. If no route relatively close to matches the user's latest intent or user last message is thank you or greeting, return an empty route ''.

# OUTPUT FORMAT
Your final output must follow this JSON format:
{
  "route": "route_name" # The matched route name, or empty string '' if no match
}

Based on your analysis, provide only the JSON object as your final output with no additional text, explanations, or whitespace.
"#;

pub type Result<T> = std::result::Result<T, RoutingModelError>;

pub struct RouterModelV1 {
    llm_providers_with_usage_yaml: String,
    routing_model: String,
}

impl RouterModelV1 {
    pub fn new(llm_providers_with_usage_yaml: String, routing_model: String) -> Self {
        RouterModelV1 {
            llm_providers_with_usage_yaml,
            routing_model,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LlmRouterResponse {
    pub route: Option<String>,
}

impl RouterModel for RouterModelV1 {
    fn generate_request(&self, messages: &[Message]) -> ChatCompletionsRequest {
        let messages_str = messages
            .iter()
            .filter(|m| m.role != SYSTEM_ROLE)
            .map(|m| {
                let content_json_str = serde_json::to_string(&m.content).unwrap_or_default();
                format!("{}: {}", m.role, content_json_str)
            })
            .collect::<Vec<String>>()
            .join("\n");

        let message = ARCH_ROUTER_V1_SYSTEM_PROMPT
            .replace("{routes}", &self.llm_providers_with_usage_yaml)
            .replace("{conversation}", messages_str.as_str());

        ChatCompletionsRequest {
            model: self.routing_model.clone(),
            messages: vec![Message {
                content: Some(message),
                role: USER_ROLE.to_string(),
                model: None,
                tool_calls: None,
                tool_call_id: None,
            }],
            tools: None,
            stream: false,
            stream_options: None,
            metadata: None,
        }
    }

    fn parse_response(&self, content: &str) -> Result<Option<String>> {
        let router_resp_fixed = fix_json_response(content);
        info!(
            "router response (fixed): {}",
            router_resp_fixed.replace("\n", "\\n")
        );
        let router_response: LlmRouterResponse = serde_json::from_str(router_resp_fixed.as_str())?;

        let selecter_llm = router_response
            .route
            .map(|route| route.strip_suffix("()").unwrap_or_default().to_string())
            .unwrap();

        if selecter_llm.is_empty() {
            return Ok(None);
        }

        Ok(Some(selecter_llm))
    }
}

fn fix_json_response(body: &str) -> String {
    let mut updated_body = body.to_string();

    updated_body = updated_body.replace("'", "\"");

    if updated_body.contains("\\n") {
        updated_body = updated_body.replace("\\n", "");
    }

    if updated_body.starts_with("```json") {
        updated_body = updated_body
            .strip_prefix("```json")
            .unwrap_or(&updated_body)
            .to_string();
    }

    if updated_body.ends_with("```") {
        updated_body = updated_body
            .strip_suffix("```")
            .unwrap_or(&updated_body)
            .to_string();
    }

    updated_body
}

impl std::fmt::Debug for dyn RouterModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RouterModel")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_system_prompt_format() {
        let expected_prompt = r#"
You are an advanced Routing Assistant designed to select the optimal route based on user requests.
Your task is to analyze conversations and match them to the most appropriate predefined route.
Review the available routes config:

# ROUTES CONFIG START
route1: description1
route2: description2
# ROUTES CONFIG END

Examine the following conversation between a user and an assistant:

# CONVERSATION START
user: "Hello, I want to book a flight."
assistant: "Sure, where would you like to go?"
user: "seattle"
# CONVERSATION END

Your goal is to identify the most appropriate route that matches the user's LATEST intent. Follow these steps:

1. Carefully read and analyze the provided conversation, focusing on the user's latest request and the conversation scenario.
2. Check if the user's request and scenario matches any of the routes in the routing configuration (focus on the description).
3. Find the route that best matches.
4. Use context clues from the entire conversation to determine the best fit.
5. Return the best match possible. You only response the name of the route that best matches the user's request, use the exact name in the routes config.
6. If no route relatively close to matches the user's latest intent or user last message is thank you or greeting, return an empty route ''.

# OUTPUT FORMAT
Your final output must follow this JSON format:
{
  "route": "route_name" # The matched route name, or empty string '' if no match
}

Based on your analysis, provide only the JSON object as your final output with no additional text, explanations, or whitespace.
"#;

        let routes_yaml = "route1: description1\nroute2: description2";
        let routing_model = "test-model".to_string();
        let router = RouterModelV1::new(routes_yaml.to_string(), routing_model.clone());

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: Some("You are a helpful assistant.".to_string()),
                ..Default::default()
            },
            Message {
                role: "user".to_string(),
                content: Some("Hello, I want to book a flight.".to_string()),
                ..Default::default()
            },
            Message {
                role: "assistant".to_string(),
                content: Some("Sure, where would you like to go?".to_string()),
                ..Default::default()
            },
            Message {
                role: "user".to_string(),
                content: Some("seattle".to_string()),
                ..Default::default()
            },
        ];

        let req = router.generate_request(&messages);

        let prompt = req.messages[0].content.as_ref().unwrap();

        assert_eq!(expected_prompt, prompt);
    }
}
