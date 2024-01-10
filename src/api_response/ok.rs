use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiResponseBuilder {
    pub status: String,
    pub message: String,
}

impl ApiResponse {
    pub fn set() -> ApiResponseBuilder {
        ApiResponseBuilder {
            status: String::new(),
            message: String::new(),
        }
    }
}

impl ApiResponseBuilder {
    pub fn status(&mut self, status: StatusCode) -> &mut Self {
        self.status = status.to_string();
        self
    }

    pub fn message(&mut self, message: &str) -> &mut Self {
        self.message = message.to_string();
        self
    }

    pub fn build(&mut self) -> ApiResponse {
        ApiResponse {
            status: self.status.to_owned(),
            message: self.message.to_owned(),
        }
    }
}
