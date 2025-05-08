// use std::rc::Rc;
// use std::time::Duration;

// use common::api::open_ai::{ChatCompletionsRequest, Message};
// use common::configuration::LlmProvider;
// use common::consts::{ARCH_INTERNAL_CLUSTER_NAME, ARCH_UPSTREAM_HOST_HEADER};
// use common::errors::ServerError;
// use common::http::{CallArgs, Client};
// use log::{info, warn};
// use proxy_wasm::traits::HttpContext;
// use proxy_wasm::types::Action;

// use crate::llm_routing_consts::SYSTEM_PROMPT;
// use crate::stream_context::{CallContext, StreamContext};

// pub trait Routing {
//     fn route(&self) -> Action;
// }

// impl Routing for StreamContext {
//     fn route(&self) -> Action {
//         let usage_based_providers = self
//             .llm_providers
//             .iter()
//             .filter(|(_, provider)| provider.usage.is_some())
//             .map(|(_, provider)| provider.clone())
//             .collect::<Vec<Rc<LlmProvider>>>();

//         info!(
//             "usage based providers found: {}",
//             usage_based_providers
//                 .iter()
//                 .map(|provider| provider.name.clone())
//                 .collect::<Vec<String>>()
//                 .join(", ")
//         );

//         if usage_based_providers.is_empty() {
//             self.set_http_request_body(
//                 0,
//                 self.request_size.unwrap(),
//                 self.request_body.as_ref().unwrap().as_bytes(),
//             );
//             return Action::Continue;
//         }

//         let llm_routes_str = r#"- name: gpt-4o
//   description: simple requests, basic fact retrieval, easy to answer
// - name: o4-mini()
//   description: complex reasoning problem, require multi step answer"#;

//         let chat_completions_request_messages_str =
//             serde_json::to_string(&self.chat_completion_request.as_ref().unwrap().messages)
//                 .expect("failed to serialize llm routing request messages");

//         let system_prompt_formatted = SYSTEM_PROMPT
//             .replace("{routes}", llm_routes_str)
//             .replace("{conversation}", &chat_completions_request_messages_str);

//         let message = Message {
//             role: "user".to_string(),
//             content: Some(system_prompt_formatted),
//             model: None,
//             tool_calls: None,
//             tool_call_id: None,
//         };

//         let llm_routing_request = ChatCompletionsRequest {
//             model: "cotran2/llama-1b-4-26".to_string(),
//             messages: vec![message],
//             tools: None,
//             stream: false,
//             stream_options: None,
//             metadata: None,
//         };

//         let llm_routing_request_str = serde_json::to_string(&llm_routing_request)
//             .expect("failed to serialize llm routing request");

//         let headers = vec![
//             (":method", "POST"),
//             (ARCH_UPSTREAM_HOST_HEADER, "gcp_hosted_outer_llm"),
//             (":path", "/v1/chat/completions"),
//             (":authority", "gcp_hosted_outer_llm"),
//             ("content-type", "application/json"),
//             ("x-envoy-max-retries", "3"),
//             ("x-envoy-upstream-rq-timeout-ms", "5000"),
//         ];

//         let call_args = CallArgs::new(
//             ARCH_INTERNAL_CLUSTER_NAME,
//             "/v1/chat/completions",
//             headers,
//             llm_routing_request_str.as_bytes().into(),
//             vec![],
//             Duration::from_secs(5),
//         );

//         if let Err(e) = self.http_call(call_args, CallContext {}) {
//             warn!("failed to call LLM provider: {}", e);
//             self.send_server_error(ServerError::HttpDispatch(e), None);
//         }

//         Action::Pause
//     }
// }
