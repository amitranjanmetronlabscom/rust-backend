use warp::reject::Reject;
use std::fmt;

#[derive(Debug)]
pub struct CustomError {
    message: String,
}

impl CustomError {
    pub fn new(message: &str) -> Self {
        CustomError {
            message: message.to_string(),
        }
    }
}

impl Reject for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CustomError {}
