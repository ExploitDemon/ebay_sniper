// src/errors.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to make API request: {0}")]
    ApiRequest(#[from] reqwest::Error),
    #[error("Failed to deserialize API response: {0}")]
    Deserialization(#[from] serde_json::Error),
    #[error("API returned an error: {0}")]
    ApiError(ApiError),
    #[error("unknown error")]
    Unknown,
    #[error("Field not found: {0}")]
    MissingField(String),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Item not found: {0}")]
    ItemNotFound(u64),
    #[error("Failed to get item ID from request parameters")]
    ItemIdError,
    #[error("Invalid API key")]
    InvalidApiKey,
    // Add other API error types here as needed
}
