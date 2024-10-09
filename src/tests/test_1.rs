use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use reqwest::cookie::Jar; 
use serde::{Deserialize, Serialize};
use std::error::Error;
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String,
    data: Option<serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Warp server...");

    // Create a warp filter for the API
    let api = warp::path("api")
        .and(warp::path("data"))
        .and_then(handle_request);

    // Start the warp server
    println!("Server running on http://127.0.0.1:3030");
    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

async fn handle_request() -> Result<impl warp::Reply, warp::Rejection> {
    println!("Handling request...");

    // Login information
    let username = "amit.ranjan@metronlabs.com";
    let password = "metron@123";
    let server = "integration.cybereason.net";
    let port = "8443";

    let data = [
        ("username", username),
        ("password", password),
    ];

    let base_url = format!("https://{}:{}", server, port);
    let login_url = format!("{}/login.html", base_url);

    // Create a new HTTP client
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .cookie_provider(Jar::default().into())
        .build()
        .map_err(|e| {
            println!("Error creating client: {:?}", e);
            warp::reject()
        })?;

    // Perform the login request
    println!("Sending login request to {}", login_url);
    let login_response = client
        .post(&login_url)
        .form(&data)
        .send()
        .await
        .map_err(|e| {
            println!("Error during login request: {:?}", e);
            warp::reject()
        })?;

    // Check the response status
    if !login_response.status().is_success() {
        println!("Login failed with status: {}", login_response.status());
        return Ok(warp::reply::json(&ApiResponse {
            message: "Login failed".to_string(),
            data: None,
        }));
    }
    println!("Login successful!");

    // Request URL
    let endpoint_url = "/rest/savedQueries/all";
    let api_url = format!("{}{}", base_url, endpoint_url);

    // Prepare headers for the API request
    let mut api_headers = HeaderMap::new();
    api_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Perform the API request
    println!("Sending API request to {}", api_url);
    let api_response = client
        .get(&api_url)
        .headers(api_headers)
        .send()
        .await
        .map_err(|e| {
            println!("Error during API request: {:?}", e);
            warp::reject()
        })?;

    // Parse and return the JSON response
    let response_data: serde_json::Value = api_response.json().await.map_err(|e| {
        println!("Error parsing JSON: {:?}", e);
        warp::reject()
    })?;

    println!("API request successful. Response data: {:?}", response_data);

    Ok(warp::reply::json(&ApiResponse {
        message: "Data retrieved successfully".to_string(),
        data: Some(response_data),
    }))
}
