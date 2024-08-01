use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionsData {
    #[serde(rename = "optionList")]
    pub option_list: Vec<OptionsListItem>,
    #[serde(rename = "IMAGE_HOST")]
    pub image_host: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionsListItem {
    pub option_code: String,
    pub option_name: String,
    pub option_short_name: String,
    pub option_gbn: String,
    pub option_gbn_name: String,
    pub order_seq: u16,
    pub use_yn: String,
    pub file_name: String,
    pub option_remark: String,
    pub modify_user: String,
    pub modify_date: String,
    pub regi_user: String,
    pub regi_date: String,
}
#[derive(Debug)]
pub struct OptionResultItem<'a> {
    pub option_name: &'a String,
    pub option_remark: &'a String,
}
