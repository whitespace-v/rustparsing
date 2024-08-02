use crate::{
    extractor::extract::{extract_attr, extract_value},
    kbchachacha::pagination::popup::structs::{OptionResultItem, OptionsData},
};
use std::{collections::HashMap, error::Error, vec};

pub fn parse(car_seq: &str) -> Result<HashMap<String, Vec<OptionResultItem>>, Box<dyn Error>> {
    let resp: String = ureq::post("https://www.kbchachacha.com/public/layer/car/option/list.kbc")
        .send_form(&[("layerId", "layerCarOptionView"), ("carSeq", car_seq)])
        .unwrap()
        .into_string()
        .unwrap();
    let document = &scraper::Html::parse_document(&resp);
    // name
    let title = extract_value(document, "strong.car-name");

    // extract from here data
    let options_data: OptionsData =
        ureq::post("https://www.kbchachacha.com/public/car/option/code/list.json")
            .send_form(&[("layerId", "layerCarOptionView"), ("carSeq", car_seq)])
            .unwrap()
            .into_json()
            .unwrap();
    let avaliable_options = extract_attr(document, "value", "input[id='carOption']");

    let mut result_options1: Vec<OptionResultItem> = vec![];
    let mut result_options2: Vec<OptionResultItem> = vec![];
    let mut result_options3: Vec<OptionResultItem> = vec![];
    let mut result_options4: Vec<OptionResultItem> = vec![];
    let mut result_options5: Vec<OptionResultItem> = vec![];

    for i in avaliable_options.split(",").collect::<Vec<&str>>() {
        let item = options_data
            .option_list
            .iter()
            .find(|e| e.option_code == i)
            .unwrap();
        match item.option_gbn.as_str() {
            "039110" => result_options1.push(OptionResultItem {
                option_name: item.option_name.to_owned(),
                option_remark: item.option_remark.to_owned(),
            }),
            "039120" => result_options2.push(OptionResultItem {
                option_name: item.option_name.to_owned(),
                option_remark: item.option_remark.to_owned(),
            }),
            "039130" => result_options3.push(OptionResultItem {
                option_name: item.option_name.to_owned(),
                option_remark: item.option_remark.to_owned(),
            }),
            "039140" => result_options4.push(OptionResultItem {
                option_name: item.option_name.to_owned(),
                option_remark: item.option_remark.to_owned(),
            }),
            "039150" => result_options5.push(OptionResultItem {
                option_name: item.option_name.to_owned(),
                option_remark: item.option_remark.to_owned(),
            }),
            _ => {}
        }
    }
    Ok(HashMap::from([
        (String::from("Варианты экстерьера"), result_options1),
        (String::from("Встроенные опции"), result_options2),
        (
            String::from("Предохранительные устройства"),
            result_options3,
        ),
        (String::from("Устройства для Удобства"), result_options4),
        (String::from("Мультимедиа"), result_options5),
    ]))
}
