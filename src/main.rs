mod config;
mod handlers;
mod services;
mod models;
mod utils;

use warp::Filter;
use crate::handlers::{data_handler::handle_data_request, permitted_handler::handle_permitted_request, bundle_handler::*, columns_handler::handle_columns_request};
use warp::http::header::CONTENT_TYPE;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Warp server...");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![CONTENT_TYPE])
        .allow_methods(vec!["GET", "POST", "OPTIONS"]);

    let api = warp::path("api")
        .and(
            warp::path("savedqueries").and_then(handle_data_request)
            .or(warp::path("permitted").and_then(handle_permitted_request))
            .or(warp::path("configuration").and(warp::path("bundle").and_then(handle_configuration_bundle_request)))
            .or(warp::path("users").and(warp::path("bundle").and_then(handle_users_bundle_request)))
            .or(warp::path("columns")
            .and(warp::path::param::<String>())
            .and(warp::path::param::<String>())
            .and_then(handle_columns_request))
        )
        .with(cors);

    println!("Server running on http://127.0.0.1:3030");
    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
