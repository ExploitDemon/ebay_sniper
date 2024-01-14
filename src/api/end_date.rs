use api_shit::data::{ApiResponse, ApiResponseWrapper};
use rocket::get;
use rocket::http::Status;
use rocket::request::Request;
use rocket::serde::Serialize;
use thiserror::Error;

use api_shit::{ApiHandler, AppError};
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;

#[derive(Serialize)]
pub struct EndDate {
    formatted_end_date: String,
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
        Ok(end_date) => {
            println!("{}", end_date);
            let deserialized: ApiResponseWrapper =
                serde_json::from_value((end_date).parse().unwrap())
                    .map_err(AppError::Deserialization)
                    .expect("paul");
            let view_item_lite_response: &ApiResponse = &deserialized.view_item_lite_response;
            let end_date = &view_item_lite_response.item[0].end_date;
            let formatted_end_date = format!("{} {}", end_date.date, end_date.time);
            // println!("{:?}", deserialized);
            // let end_date: ApiResponse =
            Ok(Json(EndDate { formatted_end_date }))
        }
        Err(_) => Err(ApiError::ItemIdError),
    }
}
