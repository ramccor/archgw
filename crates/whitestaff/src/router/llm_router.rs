use common::{
    api::open_ai::{ChatCompletionsRequest, ChatCompletionsResponse, Message},
    configuration::LlmProvider,
    consts::USER_ROLE,
};
use hyper::header;
use thiserror::Error;
use tracing::info;

use crate::{router::consts::ARCH_ROUTER_V1_SYSTEM_PROMPT, types::types::LlmRouterResponse};

// Domain Service example
pub struct RouterService {
    providers: Vec<LlmProvider>,
    providers_with_usage: Vec<LlmProvider>,
    router_url: String,
    client: reqwest::Client,
    llm_providers_with_usage_yaml: String,
}

#[derive(Debug, Error)]
pub enum RoutingError {
    #[error("Failed to send request: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, RoutingError>;

impl RouterService {
    pub fn new(providers: Vec<LlmProvider>, router_url: String) -> Self {
        let providers_with_usage = providers
            .iter()
            .filter(|provider| provider.usage.is_some())
            .cloned()
            .collect::<Vec<LlmProvider>>();

        // convert the llm_providers to yaml string but only include name and usage
        let llm_providers_with_usage_yaml = providers_with_usage
            .iter()
            .map(|provider| {
                format!(
                    "- name: {}()\n  description: {}",
                    provider.name,
                    provider.usage.as_ref().unwrap_or(&"".to_string())
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        info!(
            "llm_providers from config with usage: {}...",
            &llm_providers_with_usage_yaml.replace("\n", "\\n")
        );

        RouterService {
            providers,
            providers_with_usage,
            router_url,
            llm_providers_with_usage_yaml,
            client: reqwest::Client::new(),
        }
    }

    pub async fn determine_route(
        &self,
        chat_completion_request: &ChatCompletionsRequest,
    ) -> Result<String> {
        let message = ARCH_ROUTER_V1_SYSTEM_PROMPT
            .replace("{routes}", &self.llm_providers_with_usage_yaml)
            .replace(
                "{conversation}",
                &serde_json::to_string_pretty(&chat_completion_request.messages).unwrap(),
            );

        let router_request: ChatCompletionsRequest = ChatCompletionsRequest {
            model: "cotran2/llama-1b-4-26".to_string(),
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
        };

        info!(
            "router_request: {}",
            &serde_json::to_string(&router_request).unwrap()
        );

        // let trace_parent = request_headers
        //     .iter()
        //     .find(|(ty, _)| ty.as_str() == "traceparent")
        //     .map(|(_, value)| value.to_str().unwrap_or_default());

        let mut llm_route_request_headers = header::HeaderMap::new();
        llm_route_request_headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        // // attach traceparent header to the llm router request
        // if let Some(trace_parent) = trace_parent {
        //     llm_route_request_headers.insert(
        //         header::HeaderName::from_static("traceparent"),
        //         header::HeaderValue::from_str(trace_parent).unwrap(),
        //     );
        // }

        llm_route_request_headers.insert(
            header::HeaderName::from_static("host"),
            header::HeaderValue::from_static("router_model_host"),
        );

        let res = reqwest::Client::new()
            .post(&self.router_url)
            .headers(llm_route_request_headers)
            .body(serde_json::to_string(&router_request).unwrap())
            .send()
            .await?;

        let body = res.text().await?;

        let chat_completion_response: ChatCompletionsResponse = serde_json::from_str(&body)?;

        info!(
            "chat_completion_response: {}",
            &serde_json::to_string(&chat_completion_response).unwrap()
        );

        let router_resp = chat_completion_response.choices[0]
            .message
            .content
            .as_ref()
            .unwrap();
        let router_resp_fixed = router_resp.replace("'", "\"");
        let router_response: LlmRouterResponse = serde_json::from_str(router_resp_fixed.as_str())?;

        info!(
            "router_response json: {}",
            serde_json::to_string(&router_response).unwrap()
        );

        let selecter_llm = router_response
            .route
            .map(|route| route.strip_suffix("()").unwrap_or_default().to_string())
            .unwrap_or_default();

        if selecter_llm.is_empty() {
            let conversation = &serde_json::to_string(&chat_completion_request.messages).unwrap();
            info!("no route selected for conversation: {}", conversation);
        }

        info!("selecter_llm: {}", selecter_llm);

        Ok(self.router_url.clone())
    }
}
