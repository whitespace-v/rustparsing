use crate::{
    http,
    kbchachacha::structs::{Car, CarData},
};
use std::sync::Mutex;
use std::{error::Error, thread};

pub fn parse(cars: Vec<Car>) -> Result<Vec<CarData>, Box<dyn Error>> {
    let client = http::builder::build()?;

    let mutex_data_list: Mutex<Vec<CarData>> = Mutex::new(vec![]);

    thread::scope(|scope| {
        for chunk in cars.chunks(20) {
            for car in chunk {
                scope.spawn(|| {
                    let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq="
                        .to_owned()
                        + &car.car_seq;

                    match client.get(url).send() {
                        Ok(response) => {
                            let mut u_mutex_data_list = mutex_data_list.lock().unwrap();
                            let document = scraper::Html::parse_document(&response.text().unwrap());
                            // extract data
                            let car_data = CarData {
                                title: String::from("sds"),
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
