#[macro_use]
extern crate rocket;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::error::Error;

mod api;

use crate::api::end_date::get_end_date;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Cors configuration failed");

    rocket::build()
        .attach(cors)
        .mount("/", routes![get_end_date])
        .launch()
        .await?;

    Ok(())
}
