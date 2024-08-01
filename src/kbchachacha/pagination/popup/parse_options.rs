use crate::extractor::extract::{extract_attr, extract_value, extract_values};
use scraper::Html;
use std::error::Error;

pub fn parse() -> Result<(), Box<dyn Error>> {
    let resp: String = ureq::post("https://www.kbchachacha.com/public/layer/car/option/list.kbc")
        .send_form(&[("layerId", "layerCarOptionView"), ("carSeq", "26074103")])
        .unwrap()
        .into_string()
        .unwrap();
    let document = &scraper::Html::parse_document(&resp);
    // name
    let title = extract_value(document, "strong.car-name");
    // options
    let title = extract_attr(document, "value", "input[id='carOption']");
    println!("{title}");
    let resp: String = ureq::post("https://www.kbchachacha.com/public/car/option/code/list.json")
        .send_form(&[("layerId", "layerCarOptionView"), ("carSeq", "26074103")])
        .unwrap()
        .into_string()
        .unwrap();
    // gbns codes:
    // 039110
    // 039120
    // 039130
    // 039140
    // 039150
    println!("{resp}");
    Ok(())
}
