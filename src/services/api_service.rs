use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use warp::Rejection;
use serde_json::Value;
use crate::utils::error::CustomError;

pub async fn send_api_request(client: &Client, api_url: &str) -> Result<Value, Rejection> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let response = client
        .get(api_url)
        .headers(headers)
        .send()
        .await.map_err(|e| {
            warp::reject::custom(CustomError::new(&format!("API request failed: {}", e)))
        })?;

    let data: Value = response.json().await.map_err(|e| {
        warp::reject::custom(CustomError::new(&format!("Failed to parse JSON response: {}", e)))
    })?;

    Ok(data)
}
