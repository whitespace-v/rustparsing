use crate::kbchachacha::maker::structs::Maker;
use headless_chrome::Tab;
use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
// open:
// https://www.kbchachacha.com/public/search/main.kbc#!?makerCode=101&classCode=1101&carCode= //filter by brand+model
// -> be careful expecting "한줄광고 매물" - однострочное объявление
// get max cars -> get max pages (max cars / 25)
// iterate car list pages and grab "carSeq":
// https://www.kbchachacha.com/public/search/main.kbc#!?makerCode=101&classCode=1101&carCode=&page=2&sort=-orderDate
// then iterate pages and grab full info:
// https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25919156
pub async fn parse() {
    let model_list_result = fetch_models().await;
    if model_list_result.is_ok() {
        let maker: Maker = model_list_result.unwrap();
        for code in maker.result.code {
            let link = car_list_link_generator(&code.maker_code, &code.class_code);
            fetch_car_list_page(&link).await;
        }
    }
}

async fn fetch_models() -> Result<Maker, reqwest::Error> {
    let maker_url =
        String::from("https://www.kbchachacha.com/public/search/carClass.json?makerCode=");
    match reqwest::Client::new().get(&maker_url).send().await {
        /* -- successfull respose  --*/
        Ok(response) => match response.json().await {
            /* -- Successfull decode  --*/
            Ok(maker) => Ok(maker),
            /* -- Error decode  --*/
            Err(decode_error) => {
                eprintln!("Error decoding maker: {decode_error}");
                Err(decode_error)
            }
        },
        /* -- Error respose  --*/
        Err(request_error) => {
            eprintln!("Error requesting maker: {maker_url},\n {request_error}");
            Err(request_error)
        }
    }
}

fn car_list_link_generator(maker_code: &str, class_code: &str) -> String {
    let class_code_prefix = String::from("&classCode=");
    let car_code_postfix = String::from("&carCode=");
    let link = String::from("https://www.kbchachacha.com/public/search/main.kbc#!?makerCode=")
        + &maker_code
        + &class_code_prefix
        + &class_code
        + &car_code_postfix;
    link
}

async fn fetch_car_list_page(link: &String) {
    let browser = headless_chrome::Browser::default().unwrap();

    let tab = browser.new_tab().unwrap();

    let active: Result<&Tab, Box<dyn Error>> = match tab.navigate_to(&link) {
        Ok(unloaded_page) => match unloaded_page.wait_until_navigated() {
            Ok(loaded_page) => Ok(loaded_page),
            Err(load_error) => {
                println!("couldn't waited {}, {load_error}", &link);
                Err(load_error.into())
            }
        },
        Err(open_error) => {
            println!("couldn't open {}, {open_error}", &link);
            Err(open_error.into())
        }
    };

    if active.is_ok() {
        let b = &active.unwrap().get_content().expect("sdfsdf");
        let html = Html::parse_document(b);
        let total_selector = Selector::parse("span.__total").unwrap();
        let collected_vec = html
            .select(&total_selector)
            .map(|el| el.inner_html())
            .collect::<Vec<String>>();
        for i in collected_vec {
            println!("{}", i)
        }
    }
}
