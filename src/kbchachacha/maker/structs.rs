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
struct MakerResult {
    hits: [i8; 0],
    total: u8,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Maker {
    status: u8,
    message: String,
    message_detail: String,
    params: MakerParams,
    result: MakerResult,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}
