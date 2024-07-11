use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub default_fields: [u8; 0],
    pub include_fields: [u8; 0],
    pub exclude_fields: [u8; 0],
    pub sort: [u8; 0],
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MakerResult {
    pub hits: [u8; 0],
    pub total: u8,
    pub sale: HashMap<String, u16>,
    pub code: Vec<MakerResultCodeItem>,
    pub top: [u8; 0],
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MakerResultCodeItem {
    pub use_code: String,
    pub use_code_name: String,
    pub country_order: u8,
    pub maker_order: u16,
    pub maker_name: String,
    pub maker_code: String,
    pub class_order: u16,
    pub class_name: String,
    pub class_code: String,
    pub car_code: String,
}

pub enum ProcessorMessage {
    Success(String),
    Error(String),
}
