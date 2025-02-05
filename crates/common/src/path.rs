use std::collections::{HashMap, HashSet};
use urlencoding;

use crate::configuration::Parameter;

pub fn replace_params_in_path(
    path: &str,
    tool_params: &HashMap<String, String>,
    prompt_target_params: &Vec<Parameter>,
) -> Result<String, String> {
    let mut result = String::new();
    let mut in_param = false;
    let mut current_param = String::new();
    let mut vars_replaced = HashSet::new();

    for c in path.chars() {
        if c == '{' {
            in_param = true;
        } else if c == '}' {
            in_param = false;
            let param_name = current_param.clone();
            if let Some(value) = tool_params.get(&param_name) {
                let value = urlencoding::encode(value);
                result.push_str(value.into_owned().as_str());
                vars_replaced.insert(param_name.clone());
            } else {
                return Err(format!("Missing value for parameter `{}`", param_name));
            }
            current_param.clear();
        } else if in_param {
            current_param.push(c);
        } else {
            result.push(c);
        }
    }

    // add the remaining params in path
    for (param_name, value) in tool_params.iter() {
        let value = urlencoding::encode(value);
        if !vars_replaced.contains(param_name) {
            vars_replaced.insert(param_name.clone());
            if result.contains("?") {
                result.push_str(&format!("&{}={}", param_name, value));
            } else {
                result.push_str(&format!("?{}={}", param_name, value));
            }
        }
    }

    // add default values
    for param in prompt_target_params.iter() {
        if !vars_replaced.contains(&param.name) && param.default.is_some() {
            if result.contains("?") {
                result.push_str(&format!(
                    "&{}={}",
                    param.name,
                    param.default.as_ref().unwrap()
                ));
            } else {
                result.push_str(&format!(
                    "?{}={}",
                    param.name,
                    param.default.as_ref().unwrap()
                ));
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::configuration::Parameter;

    #[test]
    fn test_replace_path() {
        let path = "/cluster.open-cluster-management.io/v1/managedclusters/{cluster_name}";
        let params = vec![
            ("cluster_name".to_string(), "test1".to_string()),
            ("hello".to_string(), "hello world".to_string()),
        ]
        .into_iter()
        .collect();
        let prompt_target_params = vec![Parameter {
            name: "country".to_string(),
            parameter_type: None,
            description: "test target".to_string(),
            required: None,
            enum_values: None,
            default: Some("US".to_string()),
            in_path: None,
            format: None,
        }];

        assert_eq!(
            super::replace_params_in_path(path, &params, &prompt_target_params),
            Ok(
                "/cluster.open-cluster-management.io/v1/managedclusters/test1?hello=hello%20world&country=US"
                    .to_string()
            )
        );

        let prompt_target_params = vec![];

        let path = "/cluster.open-cluster-management.io/v1/managedclusters";
        let params = vec![].into_iter().collect();
        assert_eq!(
            super::replace_params_in_path(path, &params, &prompt_target_params),
            Ok("/cluster.open-cluster-management.io/v1/managedclusters".to_string())
        );

        let path = "/foo/{bar}/baz";
        let params = vec![("bar".to_string(), "qux".to_string())]
            .into_iter()
            .collect();
        assert_eq!(
            super::replace_params_in_path(path, &params, &prompt_target_params),
            Ok("/foo/qux/baz".to_string())
        );

        let path = "/foo/{bar}/baz/{qux}";
        let params = vec![
            ("bar".to_string(), "qux".to_string()),
            ("qux".to_string(), "quux".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(
            super::replace_params_in_path(path, &params, &prompt_target_params),
            Ok("/foo/qux/baz/quux".to_string())
        );

        let path = "/foo/{bar}/baz/{qux}";
        let params = vec![("bar".to_string(), "qux".to_string())]
            .into_iter()
            .collect();
        assert_eq!(
            super::replace_params_in_path(path, &params, &prompt_target_params),
            Err("Missing value for parameter `qux`".to_string())
        );
    }
}
