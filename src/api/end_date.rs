use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::Status;
use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use thiserror::Error;
use api_shit::{ApiHandler, ApiResponseWrapper, ResponseProcessor};
use rocket::response::{self, Responder, Response};



pub struct ItemId {
    pub value: String,
}

#[derive(Serialize)]
pub struct EndDate {
    end_date: String,
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build().status(Status::InternalServerError).ok()
    }
}
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to get item ID from request parameters")]
    ItemIdError,
    // Add other error types here as needed
}

#[get("/get_end_date/<item_id>")]
pub async fn get_end_date(item_id: &str) -> Result<Json<EndDate>, ApiError> {
    let api_handler = ApiHandler::new(item_id.to_string());
    match api_handler.get_data().await {
        Ok(response) => {
            // Process the response to extract the end date
            let processed_response = ResponseProcessor::process_response(response).await?;
            let deserialized: ApiResponseWrapper = serde_json::from_str(&processed_response)?;
            let api_end_date= &deserialized.view_item_lite_response.item[0].end_date;
            let end_date = format!("{} {}", api_end_date.date, api_end_date.time);
            Ok(Json(EndDate { end_date }))
        },
        Err(_) => Err(ApiError::ItemIdError),
    }
}