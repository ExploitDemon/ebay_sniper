use api_shit::{ApiHandler, AppError, Config};
mod api;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    api::rocket().launch().await.expect("TODO: panic message");
    Ok(())
}
