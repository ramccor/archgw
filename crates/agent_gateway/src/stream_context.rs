use crate::metrics::Metrics;
use crate::tools::compute_request_path_body;
use common::api::open_ai::{
    to_server_events, ArchState, ChatCompletionStreamResponse, ChatCompletionsRequest,
    ChatCompletionsResponse, Message, ToolCall,
};
use common::configuration::{Agent, Endpoint, Overrides, Tool, Tracing};
use common::consts::{
    API_REQUEST_TIMEOUT_MS, ARCH_FC_MODEL_NAME, ARCH_INTERNAL_CLUSTER_NAME,
    ARCH_UPSTREAM_HOST_HEADER, ASSISTANT_ROLE, REQUEST_ID_HEADER, SYSTEM_ROLE, TOOL_ROLE,
    TRACE_PARENT_HEADER, USER_ROLE,
};
use common::errors::ServerError;
use common::http::{CallArgs, Client};
use common::stats::Gauge;
use derivative::Derivative;
use http::StatusCode;
use log::{debug, info, warn};
use proxy_wasm::traits::*;
use serde_yaml::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub enum ResponseHandlerType {
    ArchFC,
    FunctionCall,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct StreamCallContext {
    pub response_handler_type: ResponseHandlerType,
    pub user_message: Option<String>,
    pub prompt_target_name: Option<String>,
    #[derivative(Debug = "ignore")]
    pub request_body: ChatCompletionsRequest,
    pub similarity_scores: Option<Vec<(String, f64)>>,
    pub upstream_cluster: Option<String>,
    pub upstream_cluster_path: Option<String>,
    pub agent: Option<Agent>,
}

pub struct StreamContext {
    pub endpoints: Rc<Option<HashMap<String, Endpoint>>>,
    pub overrides: Rc<Option<Overrides>>,
    pub metrics: Rc<Metrics>,
    pub callouts: RefCell<HashMap<u32, StreamCallContext>>,
    pub context_id: u32,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_response: Option<String>,
    pub arch_state: Option<Vec<ArchState>>,
    pub request_body_size: usize,
    pub user_prompt: Option<Message>,
    pub streaming_response: bool,
    pub is_chat_completions_request: bool,
    pub chat_completions_request: Option<ChatCompletionsRequest>,
    pub request_id: Option<String>,
    pub start_upstream_llm_request_time: u128,
    pub time_to_first_token: Option<u128>,
    pub traceparent: Option<String>,
    pub agents: Rc<HashMap<String, Agent>>,
    pub agent: Option<Agent>,
    pub tools: Rc<HashMap<String, Tool>>,
    pub _tracing: Rc<Option<Tracing>>,
    pub arch_fc_response: Option<String>,
}

impl StreamContext {
    pub fn new(
        context_id: u32,
        metrics: Rc<Metrics>,
        endpoints: Rc<Option<HashMap<String, Endpoint>>>,
        overrides: Rc<Option<Overrides>>,
        tracing: Rc<Option<Tracing>>,
        agents: Rc<HashMap<String, Agent>>,
        tools: Rc<HashMap<String, Tool>>,
    ) -> Self {
        StreamContext {
            context_id,
            metrics,
            endpoints,
            callouts: RefCell::new(HashMap::new()),
            chat_completions_request: None,
            tool_calls: None,
            tool_call_response: None,
            arch_state: None,
            request_body_size: 0,
            streaming_response: false,
            user_prompt: None,
            is_chat_completions_request: false,
            overrides,
            request_id: None,
            traceparent: None,
            _tracing: tracing,
            start_upstream_llm_request_time: 0,
            time_to_first_token: None,
            arch_fc_response: None,
            agents,
            tools,
            agent: None,
        }
    }

    pub fn send_server_error(&self, error: ServerError, override_status_code: Option<StatusCode>) {
        self.send_http_response(
            override_status_code
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                .as_u16()
                .into(),
            vec![],
            Some(format!("{error}").as_bytes()),
        );
    }

    fn _trace_arch_internal(&self) -> bool {
        match self._tracing.as_ref() {
            Some(tracing) => match tracing.trace_arch_internal.as_ref() {
                Some(trace_arch_internal) => *trace_arch_internal,
                None => false,
            },
            None => false,
        }
    }

    pub fn arch_fc_response_handler(
        &mut self,
        body: Vec<u8>,
        mut callout_context: StreamCallContext,
    ) {
        let body_str = String::from_utf8(body).unwrap();
        info!("on_http_call_response: model server response received");
        debug!("response body: {}", body_str);

        let model_server_response: ChatCompletionsResponse = match serde_json::from_str(&body_str) {
            Ok(arch_fc_response) => arch_fc_response,
            Err(e) => {
                warn!(
                    "error deserializing llm response: {}, body: {}",
                    e, body_str
                );
                return self.send_server_error(ServerError::Deserialization(e), None);
            }
        };

        //TODO: try to avoid clone
        let message = model_server_response
            .choices
            .first()
            .map(|choice| choice.message.clone())
            .unwrap();

        self.tool_calls = message.tool_calls;

        if self.tool_calls.as_ref().is_some() && self.tool_calls.as_ref().unwrap().len() > 1 {
            warn!(
                "multiple tool calls not supported yet, tool_calls count found: {}",
                self.tool_calls.as_ref().unwrap().len()
            );
        }

        if self.tool_calls.is_none() || self.tool_calls.as_ref().unwrap().is_empty() {
            // this means llm model didn't need additional data from tool calls and is ready to respond back to user

            let direct_response_str = if self.streaming_response {
                let chunks = vec![
                    ChatCompletionStreamResponse::new(
                        self.arch_fc_response.clone(),
                        Some(ASSISTANT_ROLE.to_string()),
                        Some(ARCH_FC_MODEL_NAME.to_string()),
                        None,
                    ),
                    ChatCompletionStreamResponse::new(
                        Some(
                            model_server_response.choices[0]
                                .message
                                .content
                                .as_ref()
                                .unwrap()
                                .clone(),
                        ),
                        None,
                        Some(format!("{}-Chat", ARCH_FC_MODEL_NAME.to_owned())),
                        None,
                    ),
                ];

                to_server_events(chunks)
            } else {
                body_str
            };

            self.tool_calls = None;
            return self.send_http_response(
                StatusCode::OK.as_u16().into(),
                vec![],
                Some(direct_response_str.as_bytes()),
            );
        }

        // update prompt target name from the tool call response
        callout_context.prompt_target_name =
            Some(self.tool_calls.as_ref().unwrap()[0].function.name.clone());

        self.schedule_api_call_request(callout_context);
    }

    fn schedule_api_call_request(&mut self, mut callout_context: StreamCallContext) {
        // Construct messages early to avoid mutable borrow conflicts

        let tool_name = self.tool_calls.as_ref().unwrap()[0].function.name.clone();
        let tool = self.tools.get(&tool_name).unwrap().clone();
        let tool_params = self.tool_calls.as_ref().unwrap()[0]
            .function
            .arguments
            .clone();
        let endpoint_details = tool.endpoint.as_ref().unwrap();
        let endpoint_path: String = endpoint_details
            .path
            .as_ref()
            .unwrap_or(&String::from("/"))
            .to_string();

        let http_method = endpoint_details.method.clone().unwrap_or_default();
        let prompt_target_params = tool.parameters.clone().unwrap_or_default();

        let mut tool_params_json: Option<HashMap<String, Value>> = None;

        if let Some(params) = tool_params.as_ref() {
            match serde_json::from_str::<HashMap<String, Value>>(params.as_str()) {
                Ok(params_json) => tool_params_json = Some(params_json),
                Err(e) => {
                    log::warn!(
                        "error deserializing tool params: {}, body str: {}",
                        e,
                        String::from_utf8(params.as_bytes().to_vec()).unwrap()
                    );
                    return self.send_server_error(
                        ServerError::Deserialization(e),
                        Some(StatusCode::BAD_REQUEST),
                    );
                }
            };
        }

        //TODO: fixme hack adilhafeez
        let (path, api_call_body) = match compute_request_path_body(
            &endpoint_path,
            &tool_params_json,
            &prompt_target_params,
            &http_method,
        ) {
            Ok((path, body)) => (path, body),
            Err(e) => {
                return self.send_server_error(
                    ServerError::BadRequest {
                        why: format!("error computing api request path or body: {}", e),
                    },
                    Some(StatusCode::BAD_REQUEST),
                );
            }
        };

        debug!("on_http_call_response: api call body {:?}", api_call_body);

        let timeout_str = API_REQUEST_TIMEOUT_MS.to_string();

        let http_method_str = http_method.to_string();
        let mut headers: HashMap<_, _> = [
            (ARCH_UPSTREAM_HOST_HEADER, endpoint_details.name.as_str()),
            (":method", &http_method_str),
            (":path", &path),
            (":authority", endpoint_details.name.as_str()),
            ("content-type", "application/json"),
            ("x-envoy-max-retries", "3"),
            ("x-envoy-upstream-rq-timeout-ms", timeout_str.as_str()),
        ]
        .into_iter()
        .collect();

        if self.request_id.is_some() {
            headers.insert(REQUEST_ID_HEADER, self.request_id.as_ref().unwrap());
        }

        if self.traceparent.is_some() {
            headers.insert(TRACE_PARENT_HEADER, self.traceparent.as_ref().unwrap());
        }

        // override http headers that are set in the prompt target
        let http_headers = endpoint_details.http_headers.clone().unwrap_or_default();
        for (key, value) in http_headers.iter() {
            headers.insert(key.as_str(), value.as_str());
        }

        let call_args = CallArgs::new(
            ARCH_INTERNAL_CLUSTER_NAME,
            &path,
            headers.into_iter().collect(),
            api_call_body.as_deref().map(|s| s.as_bytes()),
            vec![],
            Duration::from_secs(5),
        );

        info!(
            "on_http_call_response: dispatching api call to developer endpoint: {}, path: {}, method: {}",
            endpoint_details.name, path, http_method_str
        );

        callout_context.upstream_cluster = Some(endpoint_details.name.to_owned());
        callout_context.upstream_cluster_path = Some(path.to_owned());
        callout_context.agent = self.agent.clone();
        callout_context.response_handler_type = ResponseHandlerType::FunctionCall;

        if let Err(e) = self.http_call(call_args, callout_context) {
            self.send_server_error(ServerError::HttpDispatch(e), Some(StatusCode::BAD_REQUEST));
        }
    }

    pub fn api_call_response_handler(&mut self, body: Vec<u8>, callout_context: StreamCallContext) {
        let http_status = self
            .get_http_call_response_header(":status")
            .unwrap_or(StatusCode::OK.as_str().to_string());
        info!(
            "on_http_call_response: developer api call response received: status code: {}",
            http_status
        );
        if http_status != StatusCode::OK.as_str() {
            warn!(
                "api server responded with non 2xx status code: {}",
                http_status
            );
            return self.send_server_error(
                ServerError::Upstream {
                    host: callout_context.upstream_cluster.unwrap(),
                    path: callout_context.upstream_cluster_path.unwrap(),
                    status: http_status.clone(),
                    body: String::from_utf8(body).unwrap(),
                },
                Some(StatusCode::from_str(http_status.as_str()).unwrap()),
            );
        }
        self.tool_call_response = Some(String::from_utf8(body).unwrap());
        debug!(
            "response body: {}",
            self.tool_call_response.as_ref().unwrap()
        );

        let mut messages = self.construct_llm_messages(&callout_context);

        let user_message = match messages.pop() {
            Some(user_message) => user_message,
            None => {
                return self.send_server_error(
                    ServerError::NoMessagesFound {
                        why: "no user messages found".to_string(),
                    },
                    None,
                );
            }
        };

        let final_prompt = format!(
            "{}\ncontext: {}",
            user_message.content.unwrap(),
            self.tool_call_response.as_ref().unwrap()
        );

        // add original user prompt
        messages.push({
            Message {
                role: USER_ROLE.to_string(),
                content: Some(final_prompt),
                model: None,
                tool_calls: None,
                tool_call_id: None,
            }
        });

        let chat_completions_request: ChatCompletionsRequest = ChatCompletionsRequest {
            model: callout_context.request_body.model,
            messages,
            tools: None,
            stream: callout_context.request_body.stream,
            stream_options: callout_context.request_body.stream_options,
            metadata: None,
        };

        let llm_request_str = match serde_json::to_string(&chat_completions_request) {
            Ok(json_string) => json_string,
            Err(e) => {
                return self.send_server_error(ServerError::Serialization(e), None);
            }
        };
        info!("on_http_call_response: sending request to upstream llm");
        debug!("request body: {}", llm_request_str);

        self.start_upstream_llm_request_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        self.set_http_request_body(0, self.request_body_size, &llm_request_str.into_bytes());
        self.resume_http_request();
    }

    fn filter_out_arch_messages(&self, messages: &[Message]) -> Vec<Message> {
        messages
            .iter()
            .filter(|m| {
                !(m.role == TOOL_ROLE
                    || m.content.is_none()
                    || (m.tool_calls.is_some() && !m.tool_calls.as_ref().unwrap().is_empty()))
            })
            .cloned()
            .collect()
    }

    fn construct_llm_messages(&mut self, callout_context: &StreamCallContext) -> Vec<Message> {
        let mut messages: Vec<Message> = Vec::new();

        if let Some(agent) = callout_context.agent.as_ref() {
            if let Some(system_prompt) = agent.system_prompt.as_ref() {
                let system_prompt_message = Message {
                    role: SYSTEM_ROLE.to_string(),
                    content: Some(system_prompt.clone()),
                    model: None,
                    tool_calls: None,
                    tool_call_id: None,
                };
                messages.push(system_prompt_message);
            }
        }

        messages.append(
            &mut self.filter_out_arch_messages(callout_context.request_body.messages.as_ref()),
        );
        messages
    }
}

impl Client for StreamContext {
    type CallContext = StreamCallContext;

    fn callouts(&self) -> &RefCell<HashMap<u32, Self::CallContext>> {
        &self.callouts
    }

    fn active_http_calls(&self) -> &Gauge {
        &self.metrics.active_http_calls
    }
}

#[cfg(test)]
mod test {
    use common::api::open_ai::{ChatCompletionsResponse, Choice, Message, ToolCall};

    use crate::stream_context::check_intent_matched;

    #[test]
    fn test_intent_matched() {
        let model_server_response = ChatCompletionsResponse {
            choices: vec![Choice {
                message: Message {
                    content: Some("".to_string()),
                    tool_calls: Some(vec![]),
                    role: "assistant".to_string(),
                    model: None,
                    tool_call_id: None,
                },
                finish_reason: None,
                index: None,
            }],
            usage: None,
            model: "arch-fc".to_string(),
            metadata: None,
        };

        assert!(!check_intent_matched(&model_server_response));

        let model_server_response = ChatCompletionsResponse {
            choices: vec![Choice {
                message: Message {
                    content: Some("hello".to_string()),
                    tool_calls: Some(vec![]),
                    role: "assistant".to_string(),
                    model: None,
                    tool_call_id: None,
                },
                finish_reason: None,
                index: None,
            }],
            usage: None,
            model: "arch-fc".to_string(),
            metadata: None,
        };

        assert!(check_intent_matched(&model_server_response));

        let model_server_response = ChatCompletionsResponse {
            choices: vec![Choice {
                message: Message {
                    content: Some("".to_string()),
                    tool_calls: Some(vec![ToolCall {
                        id: "1".to_string(),
                        function: common::api::open_ai::FunctionCallDetail {
                            name: "test".to_string(),
                            arguments: None,
                        },
                        tool_type: common::api::open_ai::ToolType::Function,
                    }]),
                    role: "assistant".to_string(),
                    model: None,
                    tool_call_id: None,
                },
                finish_reason: None,
                index: None,
            }],
            usage: None,
            model: "arch-fc".to_string(),
            metadata: None,
        };

        assert!(check_intent_matched(&model_server_response));
    }
}
