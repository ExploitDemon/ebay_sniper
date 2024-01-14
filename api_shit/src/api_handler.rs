// src/api_handler.rs
use crate::errors::AppError;
use crate::http_client::HttpClient;
use crate::response_processor::ResponseProcessor;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ApiHandler {
    http_client: HttpClient,
    item_id: String,
}

impl ApiHandler {
    pub fn new(item_id: String) -> Self {
        Self {
            http_client: HttpClient::new(),
            item_id,
        }
    }
    pub async fn get_data(&self) -> Result<String, AppError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let url = format!("https://www.ebay.com/lit/v1/item?item={}&pbv=1&si=%2FayvuiFKUHdzB%2Fkju4dbjbl2934%3D&lvr=2&dl=4&cb=jQuery17007696575562079055_1687152942880&_={}", self.item_id, timestamp);

        let response = self.http_client.get(&url).await?;
        ResponseProcessor::process_response(response).await
    }
}
