use reqwest::header::{HeaderMap, HeaderValue};

#[tauri::command]
pub async fn get_user_info(sec_uid: String) -> Result<String, String> {
  // let url = "https://www.iesdouyin.com/web/api/v2/user/info/?sec_uid=MS4wLjABAAAAkA4CpbT-TNLAK_0-I-I6BSrAGhtG6MeoG7SloOjwABo";

  let url = format!("https://www.iesdouyin.com/web/api/v2/user/info/?sec_uid={}", sec_uid);

  let client = reqwest::Client::new();

  let mut headers = HeaderMap::new();
  headers.insert("authority", HeaderValue::from_static("www.iesdouyin.com"));
  headers.insert("sec-fetch-user", HeaderValue::from_static("?1"));
  headers.insert("upgrade-insecure-requests", HeaderValue::from_static("1"));
  headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1"));

  let response = client.get(&url)
      .headers(headers)
      .send()
      .await
      .or(Err(format!("Failed to GET from '{}'", &url)))?;

  let body = response.text().await.or(Err(format!("Failed to GET from '{}'", &url)))?;

  Ok(body)
}
