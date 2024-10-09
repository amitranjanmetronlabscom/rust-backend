use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use reqwest::cookie::Jar;
use serde::{Deserialize, Serialize};
use std::error::Error;
use warp::{Filter, Rejection, Reply};

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String,
    data: Option<serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Warp server...");

    // Define the CORS filter
    let cors = warp::cors()
        .allow_any_origin() // Allow requests from any origin
        .allow_headers(vec![CONTENT_TYPE]) // Allow specific headers
        .allow_methods(vec!["GET", "POST", "OPTIONS"]); // Allow specific methods

    let api = warp::path("api")
        .and(warp::path("data").and_then(handle_data_request))
        .or(warp::path("permitted").and_then(handle_permitted_request))
        .with(cors); // Apply the CORS filter

    println!("Server running on http://127.0.0.1:3030");
    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

async fn handle_data_request() -> Result<impl Reply, Rejection> {
    println!("Handling request for saved queries...");

    let (client, base_url) = create_client_and_base_url().await?;

    let endpoint_url = "/rest/savedQueries/all";
    let api_url = format!("{}{}", base_url, endpoint_url);

    let response_data = send_api_request(&client, &api_url).await?;

    Ok(warp::reply::json(&ApiResponse {
        message: "Data retrieved successfully".to_string(),
        data: Some(response_data),
    }))
}

async fn handle_permitted_request() -> Result<impl Reply, Rejection> {
    println!("Handling request for permitted groups...");

    let (client, base_url) = create_client_and_base_url().await?;

    let endpoint_url = "/rest/groups/permitted";
    let api_url = format!("{}{}", base_url, endpoint_url);

    let response_data = send_api_request(&client, &api_url).await?;

    Ok(warp::reply::json(&ApiResponse {
        message: "Permitted groups retrieved successfully".to_string(),
        data: Some(response_data),
    }))
}

async fn create_client_and_base_url() -> Result<(Client, String), Rejection> {
    let username = "amit.ranjan@metronlabs.com";
    let password = "metron@123";
    let server = "integration.cybereason.net";
    let port = "8443";

    let base_url = format!("https://{}:{}", server, port);

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .cookie_provider(Jar::default().into())
        .build()
        .map_err(|e| {
            println!("Error creating client: {:?}", e);
            warp::reject()
        })?;

    let login_url = format!("{}/login.html", base_url);
    let data = [("username", username), ("password", password)];

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

    if !login_response.status().is_success() {
        println!("Login failed with status: {}", login_response.status());
        return Err(warp::reject());
    }
    println!("Login successful!");

    Ok((client, base_url))
}

async fn send_api_request(client: &Client, api_url: &str) -> Result<serde_json::Value, Rejection> {
    let mut api_headers = HeaderMap::new();
    api_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    println!("Sending API request to {}", api_url);
    let api_response = client
        .get(api_url)
        .headers(api_headers)
        .send()
        .await
        .map_err(|e| {
            println!("Error during API request: {:?}", e);
            warp::reject()
        })?;

    let response_data: serde_json::Value = api_response.json().await.map_err(|e| {
        println!("Error parsing JSON: {:?}", e);
        warp::reject()
    })?;

    println!("API request successful. Response data: {:?}", response_data);
    Ok(response_data)
}

