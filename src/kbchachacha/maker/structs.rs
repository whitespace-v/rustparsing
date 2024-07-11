use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Maker {
    status: u8,
    message: String,
    message_detail: String,
    params: MakerParams,
    result: MakerResult,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MakerParams {
    debug_mode: bool,
    page_size: u8,
    page: u8,
    default_fields: [i8; 0],
    include_fields: [i8; 0],
    exclude_fields: [i8; 0],
    sort: [i8; 0],
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MakerResult {
    hits: [i8; 0],
    total: u8,
    #[serde(rename = "수입")]
    income: Vec<MakerResultItem>,
    #[serde(rename = "국산")]
    domestical: Vec<MakerResultItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MakerResultItem {
    country_code: String,
    maker_order: u8,
    maker_name: String,
    maker_code: String,
    count: u16,
}
