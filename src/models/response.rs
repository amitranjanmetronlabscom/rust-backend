use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub message: String,
    pub data: Option<Value>,
}
