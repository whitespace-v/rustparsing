#![warn(clippy::all, clippy::pedantic)]
use crate::kbchachacha::{crawler, maker, pagination, structs::CarMaker};
use std::error::Error;

use super::structs::Car;

pub fn parse() -> Result<(), Box<dyn Error>> {
    /* Add to Makers: maker_code & class_code */
    // let makers = maker::maker::generate_makers_list()?;
    // /* Add to Makers: total & pages */
    // let makers = crawler::crawler::collect_param_list(makers)?;
    /* Add to Makers: car_seq */
    // let makers = CarMaker {
    //     class_code: "1101".to_owned(),
    //     maker_code: "101".to_owned(),
    //     pages: Some(2),
    //     total: Some(16),
    //     total_url: None,
    // };
    // let makers = vec![makers];
    // let cars = crawler::crawler::collect_seq_list(makers);

    let cars = vec![
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "25495764".to_owned(),
        },
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "25502389".to_owned(),
        // },
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "25506208".to_owned(),
        // },
    ];

    // todo: go to -> https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25919156
    let data = pagination::parser::parse(cars);
    Ok(())
}
