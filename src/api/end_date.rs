use crate::api::api_key::ApiKey;
use crate::api::errors::ApiError;
use api_shit::data::{ApiResponse, ApiResponseWrapper};
use api_shit::ApiHandler;
use dotenv::dotenv;
use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use std::env;

#[derive(Serialize)]
pub struct EndDate {
    formatted_end_date: String,
}

#[get("/get_end_date/<item_id>")]
pub async fn get_end_date(item_id: &str, key: ApiKey) -> Result<Json<EndDate>, ApiError> {
    dotenv().ok();z
    let match_key = env::var("API_KEY").unwrap_or_default();
    if key.0 != match_key {
        return Err(ApiError::InvalidApiKey);
    }

    let api_handler = ApiHandler::new(item_id.to_string());

    match api_handler.get_data().await {
        Ok(end_date) => {
            let deserialized: ApiResponseWrapper =
                serde_json::from_value((end_date).parse().unwrap())
                    .map_err(|_| ApiError::DeserializationError)?;
            let view_item_lite_response: &ApiResponse = &deserialized.view_item_lite_response;
            let end_date = &view_item_lite_response.item[0].end_date;
            let formatted_end_date = format!("{} {}", end_date.date, end_date.time);
            Ok(Json(EndDate { formatted_end_date }))
        }
        Err(_) => Err(ApiError::ItemIdError),
    }
}
