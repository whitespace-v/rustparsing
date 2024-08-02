use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CarMaker {
    pub total_url: Option<String>,
    pub maker_code: String, // бренд
    pub class_code: String, // модель
    pub total: Option<u16>,
    pub pages: Option<u16>,
    //  вообще сюда бы добавить переводы брендов и моделей
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    pub maker_code: String,
    pub class_code: String,
    pub car_seq: String,
    // add data here
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarData {
    pub name: String,
    pub price: String,
    pub maker_code: String, // бренд
    pub class_code: String, // модель
    pub seclist: CarDataSeclist,
    pub params: CarDataParams,
    pub dealer: CarDataDealer,
    pub images: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarDataSeclist {
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarDataParams {
    pub param_diag_car_yn: String,
    pub param_diag_car_seq: String,
    pub param_premium_car_yn: String,
    pub param_sec_list_type: ParamSecListType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarDataDealer {
    pub dealer_name: String,
    pub dealer_place: String,
    pub dealer_tel: String,
    pub dealer_location: String,
    pub dealer_info: String,
    pub dealer_selling: String,
    pub dealer_sold: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum ParamSecListType {
    Nothing,
    SecListUrl,
    CheckInfo,
}
