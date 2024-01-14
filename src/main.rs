use crate::api::get_end_date;
use api_shit::AppError;
use rocket::routes;

mod api;

#[rocket::main] // Use the rocket::main attribute here
async fn main() -> Result<(), AppError> {
    let _ = rocket().launch().await.expect("TODO: panic message");
    Ok(())
}
#[allow(dead_code)]
pub fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![get_end_date])
}
