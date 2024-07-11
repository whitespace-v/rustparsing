use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Maker {
    pub status: u8,
    pub message: String,
    pub message_detail: String,
    pub params: MakerParams,
    pub result: MakerResult,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MakerParams {
    pub debug_mode: bool,
    pub page_size: u8,
    pub page: u8,
    pub default_fields: [i8; 0],
    pub include_fields: [i8; 0],
    pub exclude_fields: [i8; 0],
    pub sort: [i8; 0],
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MakerResult {
    pub hits: [i8; 0],
    pub total: u8,
    #[serde(rename = "수입")]
    pub income: Vec<MakerResultItem>,
    #[serde(rename = "국산")]
    pub domestical: Vec<MakerResultItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MakerResultItem {
    pub country_code: String,
    pub maker_order: u8,
    pub maker_name: String,
    pub maker_code: String,
    pub count: u16,
}
