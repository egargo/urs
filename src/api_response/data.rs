use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiDataResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiDataResponseBuilder<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

impl<T> ApiDataResponse<T> {
    pub fn set(data: T) -> ApiDataResponseBuilder<T> {
        ApiDataResponseBuilder {
            status: String::new(),
            message: String::new(),
            data,
        }
    }
}

impl<T: std::clone::Clone> ApiDataResponseBuilder<T> {
    /// Sets the status value ([`StatusCode`]) of the response.
    pub fn status(&mut self, status: StatusCode) -> &mut Self {
        self.status = status.to_string();
        self
    }

    /// Sets the message of the response.
    pub fn message(&mut self, message: &str) -> &mut Self {
        self.message = message.to_string();
        self
    }

    /// Builds the ApiDataResponse.
    pub fn build(&mut self) -> ApiDataResponse<T> {
        ApiDataResponse {
            status: self.status.to_owned(),
            message: self.message.to_owned(),
            data: self.data.clone(),
        }
    }
}

/// Check if the returned data is empty or not.
///
/// # Example
///
/// ```rs
/// response_check_data!(data, "Data Type");
/// ```
#[macro_export]
macro_rules! response_check_data {
    ($data: expr, $request_type: expr) => {
        if $data.unwrap().len() == 0 {
            return Ok(HttpResponse::NotFound().json(
                ApiErrorResponse::set()
                    .status(StatusCode::NOT_FOUND)
                    .message(&format!("{} not found.", $request_type))
                    .build(),
            ));
        }
    };
}
