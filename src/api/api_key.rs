use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request;
use rocket::request::{FromRequest, Request};

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

pub struct ApiKey(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("key") {
            Some(key) if key == "7fa8104e-6f6d-45a0-962e-bdc555c68767" => {
                Outcome::Success(ApiKey(key.to_string()))
            }
            Some(_) => Outcome::Error((Status::Unauthorized, ApiKeyError::Invalid)),
            None => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
        }
    }
}
