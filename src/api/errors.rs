use rocket::http::Status;
use rocket::response::{self, Responder, Response};
use rocket::Request;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to get item ID from request parameters")]
    ItemIdError,
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("Failed to deserialize response")]
    DeserializationError,
    // Add other error types here as needed
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        match self {
            ApiError::ItemIdError => Response::build().status(Status::BadRequest).ok(),
            ApiError::InvalidApiKey => Response::build().status(Status::Unauthorized).ok(),
            ApiError::DeserializationError => {
                Response::build().status(Status::InternalServerError).ok()
            }
        }
    }
}
