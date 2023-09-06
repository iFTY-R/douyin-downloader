use std::collections::HashMap;
use std::str::FromStr;
use reqwest::{self, Method};
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue};

fn header_map_to_hash_map(header_map: &HeaderMap<HeaderValue>) -> HashMap<String, String> {
  let mut headers = HashMap::new();
  for (key, value) in header_map.iter() {
    if let Ok(key_str) = key.to_string().parse::<String>() {
      if let Ok(value_str) = value.to_str() {
        headers.insert(key_str, value_str.to_string());
      }
    }
  }
  headers
}

#[derive(Deserialize, Serialize)]
pub struct Request {
  method: String,
  url: String,
  headers: HashMap<String, String>,
  body: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
  status: u16,
  headers: HashMap<String, String>,
  body: String,
}

#[tauri::command]
pub async fn proxy_request(
  request: Request,
) -> Result<Response, String> {
  let method = Method::from_str(&request.method).map_err(|_| "Invalid method".to_string())?;
  let url = reqwest::Url::parse(&request.url).map_err(|_| "Invalid URL".to_string())?;

  let client = reqwest::Client::new();
  let mut request_builder = client.request(method.clone(), url.clone());

  for (key, value) in request.headers {
    request_builder = request_builder.header(&key, value);
  }

  if let Some(body) = request.body {
    request_builder = request_builder.body(body);
  }

  let response = request_builder.send().await.map_err(|e| e.to_string())?;

  let status = response.status().as_u16();
  let headers = header_map_to_hash_map(response.headers());
  let body = response.text().await.unwrap_or_default();

  Ok(Response {
    status,
    headers,
    body,
  })
}
