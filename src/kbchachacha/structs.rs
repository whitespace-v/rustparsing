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
    pub car_seq: String,
    pub seclist: CarDataSeclist,
    pub params: CarDataParams,
    pub dealer: CarDataDealer,
    pub images: Vec<String>,
    pub details: CarDataDetails,
    pub mark_sold: bool,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CarDataDetails {
    pub license_plate: String,
    pub release_year: String,
    pub mileage: String,
    pub fuel_type: String,
    pub transmission_type: String,
    pub fuel_effeciency: String,
    pub body_type: String,
    // рабочий объем
    pub engine_displacement: String,
    pub color: String,
    pub taxes: String,
    // Лишение права выкупа имущества
    pub foreclosures: String,
    // ипотека ?? кредит ?저당
    pub credit: String,
    // 제시번호 ???
    pub presentation_number: String,
    // 2 detail
    // 전손이력 ??
    pub total_loss: String,
    // наводнения
    pub immersions: String,
    // история использования
    pub usage: String,
    // количество владельцев
    pub owners: String,
    // дата запроса о страховых случаях
    pub insurance_inquiry_date: String,
}
