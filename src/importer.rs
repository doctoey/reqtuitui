use std::collections::HashMap;
use std::str::FromStr;

use uuid::Uuid;

use crate::models::{ApiRequest, BodyType, HttpMethod, RequestBody};

pub fn parse_curl(curl_string: &str) -> Result<ApiRequest, String> {
    // Find where the actual curl command starts (ignoring things like `$ ` or `> ` prefixes)
    let curl_start = curl_string.find("curl").ok_or_else(|| "Input does not appear to be a valid curl command".to_string())?;
    
    // Extract everything from 'curl' onwards and trim any trailing whitespace/newlines.
    // curl-parser pest grammar fails if there are trailing newlines.
    let cleaned = curl_string[curl_start..].trim();

    // Pre-process the string to handle smart quotes (sometimes inserted by OS / rich text editors)
    let sanitized = cleaned
        .replace('‘', "'")
        .replace('’', "'")
        .replace('“', "\"")
        .replace('”', "\"");

    // Parse the curl string using the curl-parser crate
    let parsed = curl_parser::ParsedRequest::from_str(&sanitized)
        .map_err(|e| format!("Failed to parse curl: {}", e))?;

    let http_method = match parsed.method.as_str() {
        "POST" => HttpMethod::POST,
        "PUT" => HttpMethod::PUT,
        "DELETE" => HttpMethod::DELETE,
        "PATCH" => HttpMethod::PATCH,
        "GET" => HttpMethod::GET,
        _ => HttpMethod::GET, // default
    };

    let mut headers = HashMap::new();
    for (name, value) in parsed.headers.iter() {
        if let Ok(v) = value.to_str() {
            headers.insert(name.to_string(), v.to_string());
        }
    }

    let req_body = if !parsed.body.is_empty() {
        // Join multiple body parts if there are any
        let content = parsed.body.join("&");
        RequestBody {
            body_type: BodyType::RawJson, // Assume JSON for modern web requests
            content: Some(content),
        }
    } else {
        RequestBody {
            body_type: BodyType::None,
            content: None,
        }
    };

    Ok(ApiRequest {
        id: Uuid::new_v4().to_string(),
        name: "Imported from cURL".to_string(),
        url: parsed.url.to_string(),
        method: http_method,
        headers,
        query_params: HashMap::new(),
        body: req_body,
    })
}
