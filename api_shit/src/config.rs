use crate::errors::ApiError;
use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub item_id: String,
}

impl Config {
    pub fn new() -> Result<Self, ApiError> {
        dotenv().ok();
        // let item_id = env::var("ITEM_ID").expect("cannot find item_id");
        let item_id = env::var("ITEM_ID").expect("cannot find item_id");
        Ok(Self { item_id })
    }
}
