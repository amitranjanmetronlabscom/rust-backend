use warp::{Reply, Rejection};
use crate::services::api_service::send_api_request;
use crate::services::client_service::create_client_and_base_url;
use crate::models::response::ApiResponse;

pub async fn handle_data_request() -> Result<impl Reply, Rejection> {
    let (client, base_url) = create_client_and_base_url().await?;

    let endpoint_url = "/rest/savedQueries/all";
    let api_url = format!("{}{}", base_url, endpoint_url);

    let response_data = send_api_request(&client, &api_url).await?;

    Ok(warp::reply::json(&ApiResponse {
        message: "Saved queries data retrieved successfully".to_string(),
        data: Some(response_data),
    }))
}
