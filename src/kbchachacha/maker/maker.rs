use super::structs::Maker;
use crate::http;

pub fn generate_makers_list() -> Result<Vec<String>, reqwest::Error> {
    let client = http::builder::build()?;
    let maker: Maker = client
        .get("https://www.kbchachacha.com/public/search/carClass.json?makerCode=")
        .send()?
        .json::<Maker>()?;

    let mut url_list: Vec<String> = vec![];
    for code in maker.result.code {
        url_list.push(car_list_link_generator(&code.maker_code, &code.class_code));
    }
    Ok(url_list)
}

fn car_list_link_generator(maker_code: &str, class_code: &str) -> String {
    let class_code_prefix = String::from("&classCode=");
    let link = String::from("https://www.kbchachacha.com/public/search/optionSale.json?makerCode=")
        + &maker_code
        + &class_code_prefix
        + &class_code;
    link
}
