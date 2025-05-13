use std::sync::Arc;

use bytes::Bytes;
use common::api::open_ai::ChatCompletionsRequest;
use common::consts::ARCH_PROVIDER_HINT_HEADER;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full, StreamBody};
use hyper::body::Body;
use hyper::header::{self};
use hyper::{Request, Response, StatusCode};
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::router::llm_router::RouterService;

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub async fn chat_completion(
    request: Request<hyper::body::Incoming>,
    router_service: Arc<RouterService>,
    llm_provider_endpoint: String,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let max = request.body().size_hint().upper().unwrap_or(u64::MAX);
    if max > 1024 * 1024 {
        let error_msg = format!("Request body too large: {} bytes", max);
        let mut too_large = Response::new(full(error_msg));
        *too_large.status_mut() = StatusCode::PAYLOAD_TOO_LARGE;
        return Ok(too_large);
    }

    let mut request_headers = request.headers().clone();

    info!(
        "Request headers: {}",
        request_headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or_default()))
            .collect::<Vec<String>>()
            .join(", ")
    );
    let chat_request_bytes = request.collect().await?.to_bytes();
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

    let trace_parent = request_headers
        .iter()
        .find(|(ty, _)| ty.as_str() == "traceparent")
        .map(|(_, value)| value.to_str().unwrap_or_default().to_string());

    let selected_llm = match router_service
        .determine_route(&chat_completion_request.messages, trace_parent.clone())
        .await
    {
        Ok(route) => route,
        Err(err) => {
            let err_msg = format!("Failed to determine route: {}", err);
            let mut internal_error = Response::new(full(err_msg));
            *internal_error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(internal_error);
        }
    };

    info!(
        "sending request to llm provider: {} with llm model: {:?}",
        llm_provider_endpoint, selected_llm
    );

    if let Some(trace_parent) = trace_parent {
        request_headers.insert(
            header::HeaderName::from_static("traceparent"),
            header::HeaderValue::from_str(&trace_parent).unwrap(),
        );
    }

    if let Some(selected_llm) = selected_llm {
        request_headers.insert(
            ARCH_PROVIDER_HINT_HEADER,
            header::HeaderValue::from_str(&selected_llm).unwrap(),
        );
    }

    let llm_response = match reqwest::Client::new()
        .post(llm_provider_endpoint)
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

    let response_headers = llm_response.headers().clone();

    if chat_completion_request.stream {
        // Create a channel to send data
        let (tx, rx) = mpsc::channel::<Bytes>(16);

        // Spawn a task to send data as it becomes available
        tokio::spawn(async move {
            let mut byte_stream = llm_response.bytes_stream();

            while let Some(item) = byte_stream.next().await {
                let item = match item {
                    Ok(item) => item,
                    Err(err) => {
                        //TODO: use mpsc to send result with error to the receiver
                        warn!("Error receiving chunk: {:?}", err);
                        break;
                    }
                };

                //TODO: send error to the receiver
                tx.send(item).await.unwrap();
            }
        });

        use bytes::Bytes;
        use hyper::body::Frame;
        use hyper::Response;
        use tokio_stream::wrappers::ReceiverStream;
        use tokio_stream::StreamExt;

        let stream = ReceiverStream::new(rx).map(|chunk| Ok::<_, hyper::Error>(Frame::data(chunk)));

        let stream_body = BoxBody::new(StreamBody::new(stream));

        let mut res = Response::builder();
        let headers = res.headers_mut().unwrap();

        for (header_name, header_value) in response_headers.iter() {
            headers.insert(header_name, header_value.clone());
        }

        Ok(res.body(stream_body).unwrap())
    } else {
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
}
