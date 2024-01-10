use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiAuthResponse {
    pub status: String,
    pub message: String,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiAuthResponseBuilder {
    pub status: String,
    pub message: String,
    pub token: String,
}

impl ApiAuthResponse {
    pub fn set() -> ApiAuthResponseBuilder {
        ApiAuthResponseBuilder {
            status: String::new(),
            message: String::new(),
            token: String::new(),
        }
    }
}

impl ApiAuthResponseBuilder {
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

    /// Sets the token of the response.
    pub fn token(&mut self, token: String) -> &mut Self {
        self.token = token;
        self
    }

    /// Builds the ApiAuthResponse.
    pub fn build(&mut self) -> ApiAuthResponse {
        ApiAuthResponse {
            status: self.status.to_owned(),
            message: self.message.to_owned(),
            token: self.token.to_owned(),
        }
    }
}
