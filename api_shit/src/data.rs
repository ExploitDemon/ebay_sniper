use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiResponseWrapper {
    pub view_item_lite_response: ApiResponse,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiResponse {
    pub item: Vec<Item>,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub is_refresh_page: bool,
    pub viewer_item_relation: String,
    pub end_date: EndDate,
    pub last_modified_date: u64,
    pub current_price: Price,
    pub is_ended: bool,
    pub accessed_date: u64,
    pub bid_count: u8,
    pub minimum_to_bid: Price,
    pub time_left: TimeLeft,
    pub id: u64,
    pub is_finalized: bool,
    pub viewer_item_relation_id: u8,
    pub is_auto_refresh_enabled: bool,
    pub power_bid_val1: Price,
    pub power_bid_val2: Price,
    pub power_bid_val3: Price,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndDate {
    pub time: String,
    pub date: String,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Price {
    pub clean_amount: String,
    pub amount: f64,
    pub money_standard: String,
    pub currency_code: String,
    pub currency_symbol: String,
    #[serde(default)]
    pub money_standard_no_decimals: Option<String>,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimeLeft {
    pub seconds_left: u64,
    pub minutes_left: u64,
    pub hours_left: u64,
    pub days_left: u64,
}
