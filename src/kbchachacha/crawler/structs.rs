use crate::kbchachacha::maker::structs::MakerParams;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct OptionResponse {
    pub option_sale: OptionSale,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionSale {
    // pub status: u8,
    // pub message: String,
    // pub message_detail: String,
    // pub params: MakerParams,
    pub result: OptionSaleResult,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionSaleResult {
    // pub sell_amt_gbn: HashMap<String, u16>,
    // pub car_master_special_yn: u16,
    // pub color: HashMap<String, u16>,
    // pub changeme_yn: u16,
    // pub kb_certified_delivery_yn: u16,
    // pub premium_yn: u16,
    // pub city_code: HashMap<String, HashMap<String, u8>>,
    // pub lease_yn: u16,
    // pub kb_certified_shop_yn: u16,
    // pub owner_yn: u16,
    // pub kb_certified_yn: u16,
    // pub option_code_array: HashMap<String, u16>,
    // pub regi_day_histogram: Vec<RegiDayHistogram>,
    pub total: u16,
    // pub falsity_yn: u16,
    // pub one_yn: u16,
    // pub delivery_yn: u16,
    // pub certified_shop_yn: u16,
    // pub gas: HashMap<String, u16>,
    // pub warranty_yn: u16,
    // pub disability_yn: u16,
    // pub homeservice_yn2: u16,
    // pub maker_code: HashMap<String, u16>,
    // pub take_men: HashMap<String, u16>,
    // pub friend_dealer_yn: u16,
    // pub deferred_yn: u16,
    // pub hits: Vec<u16>,
    // pub accident_yn: u16,
    // pub use_code: HashMap<String, u16>,
    // pub direct_yn: u16,
    // pub kb_lease_yn: u16,
    // pub diag_yn: u16,
    // pub labs_danji_no2: HashMap<String, u16>,
    // pub auto_gbn: HashMap<String, u16>,
    // pub interest_free_yn: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegiDayHistogram {
    pub count: u16,
    pub key: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
// pub struct OptionSaleResultCityCode(String, HashMap<String, u16>);
pub struct Odsfsd {}
