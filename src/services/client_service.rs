use reqwest::{Client, cookie::Jar};
use crate::config::load_config;
use warp::Rejection;

pub async fn create_client_and_base_url() -> Result<(Client, String), Rejection> {
    let settings = load_config().map_err(|_| warp::reject())?;
    let credentials = crate::config::get_credentials(&settings).map_err(|_| warp::reject())?;

    let base_url = format!("https://{}:{}", credentials.server, credentials.port);

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .cookie_provider(Jar::default().into())
        .build()
        .map_err(|_| warp::reject())?;

    let login_url = format!("{}/login.html", base_url);
    let data = [("username", &credentials.username), ("password", &credentials.password)];

    let login_response = client
        .post(&login_url)
        .form(&data)
        .send()
        .await
        .map_err(|_| warp::reject())?;

    if !login_response.status().is_success() {
        return Err(warp::reject());
    }

    Ok((client, base_url))
}
