use warp::{Reply, Rejection};
use crate::services::api_service::send_api_request;
use crate::services::client_service::create_client_and_base_url;
use crate::models::response::ApiResponse;

pub async fn handle_configuration_bundle_request() -> Result<impl Reply, Rejection> {
    let (client, base_url) = create_client_and_base_url().await?;

    let endpoint_url = "/rest/investigation/configuration/bundle";
    let api_url = format!("{}{}", base_url, endpoint_url);

    let response_data = send_api_request(&client, &api_url).await?;

    Ok(warp::reply::json(&ApiResponse {
        message: "Configuration bundle retrieved successfully".to_string(),
        data: Some(response_data),
    }))
}

pub async fn handle_users_bundle_request() -> Result<impl Reply, Rejection> {
    let (client, base_url) = create_client_and_base_url().await?;

    let endpoint_url = "/rest/users/bundle";
    let api_url = format!("{}{}", base_url, endpoint_url);

    let response_data = send_api_request(&client, &api_url).await?;

    Ok(warp::reply::json(&ApiResponse {
        message: "Users bundle retrieved successfully".to_string(),
        data: Some(response_data),
    }))
}
