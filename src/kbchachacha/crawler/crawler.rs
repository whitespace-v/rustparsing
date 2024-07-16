use crate::http;
use crate::kbchachacha::structs::CarMaker;
use std::error::Error;
use std::sync::Mutex;
use std::{thread, vec};

use super::structs::OptionResponse;
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

pub fn collect_seq_list(maker_list: Vec<CarMaker>) -> Result<Vec<CarMaker>, Box<dyn Error>> {
    Ok()
}
