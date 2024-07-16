use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CarMaker {
    pub total_url: Option<String>,
    pub maker_code: String,
    pub class_code: String,
    pub total: Option<u16>,
    pub pages: Option<u16>,
    pub car_seq: Option<String>,
}
