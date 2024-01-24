use crate::data::{ApiResponse, ApiResponseWrapper};
use crate::errors::{ApiError, AppError};
use reqwest::Response;
use serde_json::Value;
// use std::time::{SystemTime, UNIX_EPOCH};

pub struct ResponseProcessor;

impl ResponseProcessor {
    pub async fn process_response(response: Response) -> Result<String, AppError> {
        match response.text().await {
            Ok(response_text) => {
                // Extract JSON from JSONP
                let json_start = response_text.find('{').unwrap_or(0);
                let json_end = response_text.rfind('}').unwrap_or(response_text.len()) + 1;
                let json = &response_text[json_start..json_end];

                // Deserialize the JSON into Rust data structures
                let value: Value = serde_json::from_str(json)?;

                // Check if the JSON contains an error message
                if let Some(view_item_lite_response) = value.get("ViewItemLiteResponse") {
                    if let Some(errors) = view_item_lite_response.get("Error") {
                        // Parse the error message and create the appropriate ApiError
                        let error_id = errors[0].get("Id").unwrap().as_str().unwrap();
                        let error_value = errors[0].get("Value").unwrap()[0].as_u64().unwrap();
                        let api_error = match error_id {
                            "ItemNotFound" => ApiError::ItemNotFound(error_value),
                            // Add other error types here as needed
                            _ => return Err(AppError::Unknown),
                        };
                        return Err(AppError::ApiError(api_error));
                    }
                }

                // Deserialize the JSON value into ApiResponseWrapper
                let deserialized: ApiResponseWrapper =
                    serde_json::from_value(value).map_err(|e| {
                        if e.to_string().contains("missing field `TimeLeft`") {
                            println!("{}", AppError::MissingTimeLeftField);
                            AppError::MissingTimeLeftField
                        } else {
                            AppError::Deserialization(e)
                        }
                    })?;

                // let view_item_lite_response: &ApiResponse = &deserialized.view_item_lite_response;
                // If no error, return the data as a string
                // let timestamp = SystemTime::now()
                //     .duration_since(UNIX_EPOCH)
                //     .expect("Time went backwards")
                //     .as_secs();

                // println!("{:?}", view_item_lite_response.item[0].is_auto_refresh_enabled);
                let view_item_lite_response: &ApiResponse = &deserialized.view_item_lite_response;
                let end_date = &view_item_lite_response.item[0].end_date;
                let _formatted_end_date = format!("{} {}", end_date.date, end_date.time);

                // println!("Current timestamp: {:?}", timestamp);

                // println!("{:?}", deserialized.view_item_lite_response.item[0].is_ended);

                serde_json::to_string(&deserialized).map_err(AppError::Deserialization)
            }
            Err(e) => {
                // Handle the case where the response could not be retrieved as text
                Err(AppError::ApiRequest(e))
            }
        }
    }
    pub async fn get_end_date_string(response: Response) -> Result<String, AppError> {
        let end_date = ResponseProcessor::process_response(response).await?;
        Ok(end_date)
    }
}
