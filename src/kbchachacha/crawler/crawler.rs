use scraper::{Html, Selector};

use super::structs::OptionResponse;
use crate::http;
use crate::kbchachacha::structs::{Car, CarMaker};
use std::error::Error;
use std::sync::Mutex;
use std::{thread, vec};

// total: optionSale -> result -> total
pub fn collect_param_list(maker_list: Vec<CarMaker>) -> Result<Vec<CarMaker>, Box<dyn Error>> {
    let client = http::builder::build()?;
    let mutex_maker_list = Mutex::new(vec![]);
    thread::scope(|scope| {
        for chunk in maker_list.chunks(20) {
            for car in chunk {
                scope.spawn(|| {
                    match client
                        .get(car.total_url.as_ref().unwrap().to_string())
                        .send()
                    {
                        Ok(response) => match response.json::<OptionResponse>() {
                            Ok(serded_json) => {
                                let mut u_mutex_maker_list = mutex_maker_list.lock().unwrap();

                                let car_maker = CarMaker {
                                    total_url: None,
                                    class_code: car.class_code.to_string(),
                                    maker_code: car.maker_code.to_string(),
                                    total: Some(serded_json.option_sale.result.total),
                                    pages: Some(
                                        (serded_json.option_sale.result.total as f32 / 25 as f32)
                                            .ceil() as u16,
                                    ),
                                };
                                u_mutex_maker_list.push(car_maker)
                            }
                            Err(e) => {
                                eprintln!("{e:#?}")
                            }
                        },
                        Err(e) => {
                            eprintln!("{e:#?}")
                        }
                    };
                });
            }
        }
    });

    Ok(mutex_maker_list.into_inner().unwrap())
}

pub fn collect_seq_list(maker_list: Vec<CarMaker>) -> Result<Vec<Car>, Box<dyn Error>> {
    let client = http::builder::build()?; // todo: 20 inside
    let mutex_cars: Mutex<Vec<Car>> = Mutex::new(vec![]);
    thread::scope(|scope| {
        for chunk in maker_list.chunks(20) {
            for car in chunk {
                // divide bc max is 300...
                for page in 0..car.pages.unwrap() {
                    let link = create_car_list_page_link(&car) + &page.to_string();
                    scope.spawn(|| {
                        match client.get(link).send() {
                            Ok(response) => {
                                let mut u_mutex_cars = mutex_cars.lock().unwrap();
                                let document =
                                    scraper::Html::parse_document(&response.text().unwrap());

                                let list_selector = scraper::Selector::parse(
                                    "div.generalRegist>div.list-in>div.area",
                                )
                                .unwrap();

                                for links in document.select(&list_selector) {
                                    if let Some(seq) = links.value().attr("data-car-seq") {
                                        let item = Car {
                                            maker_code: String::from(&*car.maker_code),
                                            class_code: String::from(&*car.class_code),
                                            car_seq: seq.to_string(),
                                        };
                                        u_mutex_cars.push(item)
                                    }
                                }
                            }
                            Err(e) => eprintln!("{e:#?}"),
                        };
                    });
                }
            }
        }
    });

    Ok(mutex_cars.into_inner().unwrap())
}

fn create_car_list_page_link(car: &CarMaker) -> String {
    "https://www.kbchachacha.com/public/search/list.empty?".to_owned()
        + "makerCode="
        + &car.maker_code
        + "&classCode="
        + &car.class_code
        + "&carCode=&page="
}
