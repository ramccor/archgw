use common::configuration::{HttpMethod, Parameter};
use std::collections::HashMap;

use serde_yaml::Value;

// only add params that are of string, number and bool type
pub fn filter_tool_params(tool_params: &HashMap<String, Value>) -> HashMap<String, String> {
    tool_params
        .iter()
        .filter(|(_, value)| value.is_number() || value.is_string() || value.is_bool())
        .map(|(key, value)| match value {
            Value::Number(n) => (key.clone(), n.to_string()),
            Value::String(s) => (key.clone(), s.clone()),
            Value::Bool(b) => (key.clone(), b.to_string()),
            Value::Null => todo!(),
            Value::Sequence(_) => todo!(),
            Value::Mapping(_) => todo!(),
            Value::Tagged(_) => todo!(),
        })
        .collect::<HashMap<String, String>>()
}

pub fn compute_request_path_body(
    endpoint_path: &str,
    tool_params: &HashMap<String, Value>,
    prompt_target_params: &[Parameter],
    http_method: &HttpMethod,
) -> Result<(String, Option<String>), String> {
    let tool_url_params = filter_tool_params(tool_params);
    let (path_with_params, query_string, additional_params) = common::path::replace_params_in_path(
        endpoint_path,
        &tool_url_params,
        prompt_target_params,
    )?;

    let (path, body) = match http_method {
        HttpMethod::Get => (format!("{}?{}", path_with_params, query_string), None),
        HttpMethod::Post => {
            let mut additional_params = additional_params;
            if !query_string.is_empty() {
                query_string.split("&").for_each(|param| {
                    let mut parts = param.split("=");
                    let key = parts.next().unwrap();
                    let value = parts.next().unwrap();
                    additional_params.insert(key.to_string(), value.to_string());
                });
            }
            let body = serde_json::to_string(&additional_params).unwrap();
            (path_with_params, Some(body))
        }
    };

    Ok((path, body))
}
