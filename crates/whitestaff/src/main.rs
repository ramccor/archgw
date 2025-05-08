use bytes::Bytes;
use common::api::open_ai::{ChatCompletionsRequest, ChatCompletionsResponse, Message};
use common::configuration::{Configuration, LlmProvider};
use common::consts::{ARCH_PROVIDER_HINT_HEADER, USER_ROLE};
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::{Body, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{header, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use opentelemetry::global::BoxedTracer;
use opentelemetry::trace::FutureExt;
use opentelemetry::{
    global,
    trace::{SpanKind, Tracer},
    Context,
};
use opentelemetry_http::HeaderExtractor;
use opentelemetry_sdk::{propagation::TraceContextPropagator, trace::SdkTracerProvider};
use opentelemetry_stdout::SpanExporter;
use types::types::LlmRouterResponse;
use std::env;
use std::sync::{Arc, OnceLock};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod consts2;
use consts2::SYSTEM_PROMPT_Z;
mod types;

const BIND_ADDRESS: &str = "0.0.0.0:9091";

fn get_tracer() -> &'static BoxedTracer {
    static TRACER: OnceLock<BoxedTracer> = OnceLock::new();
    TRACER.get_or_init(|| global::tracer("archgw/whitestaff"))
}

// Utility function to extract the context from the incoming request headers
fn extract_context_from_request(req: &Request<Incoming>) -> Context {
    global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(req.headers()))
    })
}

fn init_tracer() -> SdkTracerProvider {
    global::set_text_map_propagator(TraceContextPropagator::new());
    // Install stdout exporter pipeline to be able to retrieve the collected spans.
    // For the demonstration, use `Sampler::AlwaysOn` sampler to sample all traces.
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build();

    global::set_tracer_provider(provider.clone());
    provider
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

fn shorten_string(s: &str) -> String {
    if s.len() > 80 {
        format!("{}...", &s[..80])
    } else {
        s.to_string()
    }
}

async fn chat_completion(
    req: Request<hyper::body::Incoming>,
    arch_config: Arc<Configuration>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let max = req.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 1024 {
        let error_msg = format!("Request body too large: {} bytes", max);
        let mut too_large = Response::new(full(error_msg));
        *too_large.status_mut() = StatusCode::PAYLOAD_TOO_LARGE;
        return Ok(too_large);
    }

    let mut request_headers = req.headers().clone();

    info!(
        "Request headers: {}",
        request_headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or_default()))
            .collect::<Vec<String>>()
            .join(", ")
    );
    let chat_request_bytes = req.collect().await?.to_bytes();
    let chat_completion_request: ChatCompletionsRequest =
        match serde_json::from_slice(&chat_request_bytes) {
            Ok(request) => request,
            Err(err) => {
                let err_msg = format!("Failed to parse request body: {}", err);
                let mut bad_request = Response::new(full(err_msg));
                *bad_request.status_mut() = StatusCode::BAD_REQUEST;
                return Ok(bad_request);
            }
        };

    info!(
        "Received request: {}",
        &serde_json::to_string(&chat_completion_request).unwrap()
    );

    let llm_providers: Vec<LlmProvider> = chat_completion_request
        .metadata
        .as_ref()
        .and_then(|metadata| metadata.get("llm_providers"))
        .and_then(|providers| serde_json::from_str::<Vec<LlmProvider>>(providers).ok())
        .unwrap_or_default();

    info!(
        "llm_providers from request: {}...",
        shorten_string(&serde_json::to_string(&llm_providers).unwrap())
    );

    let llm_router_with_usage = arch_config
        .llm_providers
        .iter()
        .filter(|provider| provider.usage.is_some()).cloned()
        .collect::<Vec<LlmProvider>>();

    // convert the llm_providers to yaml string but only include name and usage
    let llm_providers_yaml = llm_router_with_usage
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
        "llm_providers from config: {}...",
        shorten_string(&llm_providers_yaml.replace("\n", "\\n"))
    );

    let message = SYSTEM_PROMPT_Z
        .replace("{routes}", &llm_providers_yaml)
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
        "router_request: {}...",
        shorten_string(&serde_json::to_string(&router_request).unwrap())
    );

    let trace_parent = request_headers
        .iter()
        .find(|(ty, _)| ty.as_str() == "traceparent")
        .map(|(_, value)| value.to_str().unwrap_or_default());

    let mut llm_route_request_headers = header::HeaderMap::new();
    llm_route_request_headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    // attach traceparent header to the llm router request
    if let Some(trace_parent) = trace_parent {
        llm_route_request_headers.insert(
            header::HeaderName::from_static("traceparent"),
            header::HeaderValue::from_str(trace_parent).unwrap(),
        );
    }

    llm_route_request_headers.insert(
        header::HeaderName::from_static("host"),
        header::HeaderValue::from_static("router_model_host"),
    );

    let res = match reqwest::Client::new()
        .post("http://localhost:9090/v1/chat/completions")
        .headers(llm_route_request_headers)
        .body(serde_json::to_string(&router_request).unwrap())
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            let err_msg = format!("Failed to send request: {}", err);
            let mut internal_error = Response::new(full(err_msg));
            *internal_error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(internal_error);
        }
    };

    let body = match res.text().await {
        Ok(body) => body,
        Err(err) => {
            let err_msg = format!("Failed to read response: {}", err);
            let mut internal_error = Response::new(full(err_msg));
            *internal_error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(internal_error);
        }
    };

    let chat_completion_response: ChatCompletionsResponse = match serde_json::from_str(&body) {
        Ok(response) => response,
        Err(err) => {
            let err_msg = format!("Failed to parse response: {}", err);
            let mut internal_error = Response::new(full(err_msg));
            *internal_error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(internal_error);
        }
    };

    info!(
        "chat_completion_response: {}",
        shorten_string(&serde_json::to_string(&chat_completion_response).unwrap())
    );

    let router_resp = chat_completion_response.choices[0]
        .message
        .content
        .as_ref()
        .unwrap();
    let router_resp_fixed = router_resp.replace("'", "\"");
    let router_response: LlmRouterResponse = match serde_json::from_str(router_resp_fixed.as_str())
    {
        Ok(response) => response,
        Err(err) => {
            let err_msg = format!("Failed to parse response: {}", err);
            let mut internal_error = Response::new(full(err_msg));
            *internal_error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(internal_error);
        }
    };

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
        info!(
            "no route selected for conversation: {}",
            shorten_string(conversation)
        );
    }

    info!("selecter_llm: {}", selecter_llm);

    if let Some(trace_parent) = trace_parent {
        request_headers.insert(
            header::HeaderName::from_static("traceparent"),
            header::HeaderValue::from_str(trace_parent).unwrap(),
        );
    }

    if !selecter_llm.is_empty() {
        request_headers.insert(
            ARCH_PROVIDER_HINT_HEADER,
            header::HeaderValue::from_str(&selecter_llm).unwrap(),
        );
    }

    let llm_response = match reqwest::Client::new()
        .post("http://localhost:12000/v1/chat/completions")
        .headers(request_headers)
        .body(chat_request_bytes)
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            let err_msg = format!("Failed to send request: {}", err);
            let mut internal_error = Response::new(full(err_msg));
            *internal_error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(internal_error);
        }
    };

    let body = match llm_response.text().await {
        Ok(body) => body,
        Err(err) => {
            let err_msg = format!("Failed to read response: {}", err);
            let mut internal_error = Response::new(full(err_msg));
            *internal_error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(internal_error);
        }
    };

    let mut ok_response = Response::new(full(body));
    *ok_response.status_mut() = StatusCode::OK;

    Ok(ok_response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _tracer_provider = init_tracer();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| BIND_ADDRESS.to_string());

    //loading arch_config.yaml file
    let arch_config_path =
        env::var("ARCH_CONFIG_PATH").unwrap_or_else(|_| "arch_config.yaml".to_string());
    info!("Loading arch_config.yaml from {}", arch_config_path);
    let arch_config =
        std::fs::read_to_string(&arch_config_path).expect("Failed to read arch_config.yaml");
    let config: Configuration =
        serde_yaml::from_str(&arch_config).expect("Failed to parse arch_config.yaml");
    let arch_config = Arc::new(config);
    info!(
        "arch_config: {:?}",
        shorten_string(&serde_json::to_string(arch_config.as_ref()).unwrap())
    );

    info!("Listening on http://{}", bind_address);
    let listener = TcpListener::bind(bind_address).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let peer_addr = stream.peer_addr()?;
        let io = TokioIo::new(stream);

        let arch_config = Arc::clone(&arch_config);

        let service = service_fn(move |req| {
            let arch_config = Arc::clone(&arch_config);
            let parent_cx = extract_context_from_request(&req);
            info!("parent_cx: {:?}", parent_cx);
            let tracer = get_tracer();
            let _span = tracer
                .span_builder("chat_completion")
                .with_kind(SpanKind::Server)
                .start_with_context(tracer, &parent_cx);

            async move {
                match (req.method(), req.uri().path()) {
                    (&Method::POST, "/v1/chat/completions") => {
                        info!(
                            "config: {:?}",
                            shorten_string(
                                &serde_json::to_string(&arch_config.llm_providers).unwrap()
                            )
                        );
                        chat_completion(req, arch_config)
                            .with_context(parent_cx)
                            .await
                    }
                    _ => {
                        let mut not_found = Response::new(empty());
                        *not_found.status_mut() = StatusCode::NOT_FOUND;
                        Ok(not_found)
                    }
                }
            }
        });

        tokio::task::spawn(async move {
            info!("Accepted connection from {:?}", peer_addr);
            if let Err(err) = http1::Builder::new()
                // .serve_connection(io, service_fn(chat_completion))
                .serve_connection(io, service)
                .await
            {
                info!("Error serving connection: {:?}", err);
            }
        });
    }
}
