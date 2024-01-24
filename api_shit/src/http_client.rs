use crate::AppError;
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, USER_AGENT};
use reqwest::Client;
use std::time::Duration;
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(32))
            .build()
            .expect("Failed to build client");

        Self { client }
    }

    pub async fn get(&self, url: &str) -> Result<reqwest::Response, AppError> {
        // let item_id = env::var("ITEM_ID").expect("cannot find item_id");
        let response = self.client
            .get(url)
            .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0")
            .header(ACCEPT, "text/javascript, application/javascript, application/ecmascript, application/x-ecmascript, */*; q=0.01")
            // .header(REFERER, &format!("https://www.ebay.com/itm/{}?autorefresh=true", item_id))
            .header(ACCEPT_ENCODING, "gzip,deflate")
            .send()
            .await?;

        Ok(response)
    }
}
