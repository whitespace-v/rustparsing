use crate::{
    http,
    kbchachacha::structs::{Car, CarData},
};
use std::str;
use std::sync::Mutex;
use std::{error::Error, thread};
pub fn parse(cars: Vec<Car>) -> Result<Vec<CarData>, Box<dyn Error>> {
    let client = http::builder::build_ureq_client()?;

    let mutex_data_list: Mutex<Vec<CarData>> = Mutex::new(vec![]);
    // https://www.kbchachacha.com/public/layer/common/finance/monthly/calc.json
    thread::scope(|scope| {
        for chunk in cars.chunks(20) {
            for car in chunk {
                scope.spawn(|| {
                    let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25495764"
                        .to_owned();
                    match client.get(&url).call() {
                        Ok(response) => {
                            let mut u_mutex_data_list = mutex_data_list.lock().unwrap();
                            let html = response.into_string().expect("couldn't parse string");
                            let document = &scraper::Html::parse_document(&html);

                            // стоимость
                            let data_price_selector =
                                &scraper::Selector::parse("dd > strong").unwrap();
                            // название
                            let data_name_selector =
                                &scraper::Selector::parse("strong.car-buy-name").unwrap();

                            let data_info_selector =
                                &scraper::Selector::parse("div.txt-info > span").unwrap();

                            let data_info = document
                                .select(&data_info_selector)
                                .map(|price| price.inner_html())
                                .collect::<Vec<String>>();
                            print!("data_info: {data_info:?}");

                            // dealer
                            let data_dealer_name_selector =
                                &scraper::Selector::parse("div.dealer-cnt > span.name").unwrap();
                            let data_dealer_place_selector =
                                &scraper::Selector::parse("span.place-add").unwrap();
                            let data_dealer_tel_selector =
                                &scraper::Selector::parse("div.dealer-tel-num").unwrap();

                            // bytes
                            let data_dealer_location_selector =
                                &scraper::Selector::parse("div.map-txt").unwrap();
                            // bytes
                            let data_dealer_info_selector =
                                &scraper::Selector::parse("div.dealer-scroll").unwrap();
                            let data_dealer_sell_selector =
                                &scraper::Selector::parse("span[id=btnDealerHome3]").unwrap();

                            let data_dealer_sold_selector =
                                &scraper::Selector::parse("span[id=btnDealerHome4]").unwrap();

                            // table of information
                            let data_table_selector =
                                &scraper::Selector::parse("div.detail-info01").unwrap(); //here

                            // //works
                            let selectors = vec![
                                data_name_selector,
                                data_price_selector,
                                data_dealer_name_selector,
                                data_dealer_place_selector,
                                data_dealer_tel_selector,
                                data_dealer_info_selector,
                                data_dealer_location_selector,
                                data_dealer_sell_selector,
                                data_dealer_sold_selector,
                            ];
                            for selector in selectors {
                                let title = document
                                    .select(&selector)
                                    .next()
                                    .map(|price| price.text().collect::<String>());

                                println!("\n{title:?}");
                            }

                            // // extract data
                            let car_data = CarData {
                                title: String::from("sds"),
                                maker_code: car.maker_code.to_string(),
                                class_code: car.class_code.to_string(),
                            };
                            u_mutex_data_list.push(car_data)
                        }
                        Err(e) => {
                            eprintln!("{e:#?}")
                        }
                    }
                });
            }
        }
    });
    Ok(mutex_data_list.into_inner().unwrap())
}
