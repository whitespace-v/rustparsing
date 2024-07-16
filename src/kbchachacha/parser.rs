#![warn(clippy::all, clippy::pedantic)]
use crate::kbchachacha::{crawler, maker};
use std::error::Error;

pub fn parse() -> Result<(), Box<dyn Error>> {
    /* Generate Vector of car-list urls within maker */
    // let url_list = maker::maker::generate_makers_list()?;
    /* Generate Page url vector */
    let url_list = vec![
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=101&classCode=1114"
            .to_owned(),
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=103&classCode=1187"
            .to_owned(),
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=117&classCode=2230"
            .to_owned(),
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=162&classCode=2214"
            .to_owned(),
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=108&classCode=1929"
            .to_owned(),
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=128&classCode=2004"
            .to_owned(),
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=102&classCode=1135"
            .to_owned(),
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=132&classCode=2111"
            .to_owned(),
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=117&classCode=2234"
            .to_owned(),
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=124&classCode=2079"
            .to_owned(),
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=121&classCode=2417"
            .to_owned(),
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=129&classCode=2586"
            .to_owned(),
        "https://www.kbchachacha.com/public/search/optionSale.json?makerCode=146&classCode=2379"
            .to_owned(),
    ];

    let car_page_url_list = crawler::crawler::collect_param_list(url_list);
    println!("{car_page_url_list:?}");
    Ok(())
}
