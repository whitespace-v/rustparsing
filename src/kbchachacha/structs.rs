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
    pub title: String,
    pub maker_code: String, // бренд
    pub class_code: String, // модель
}
