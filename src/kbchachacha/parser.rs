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
        // SecListUrl
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "24631894".to_owned(),
        // },
        // // SecListUrl
        Car {
            maker_code: "101".to_owned(),
            class_code: "1101".to_owned(),
            car_seq: "25956913".to_owned(),
        },
        // // SecListUrl
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "24955004".to_owned(),
        // },
        // // Sold out
        // // SecListUrl
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "25941145".to_owned(),
        // },
        // // SecListUrl
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "25496599".to_owned(),
        // },
        // // SecListUrl
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "25837384".to_owned(),
        // },
        // // SecListUrl
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "26071714".to_owned(),
        // },
        // // sold out
        // // SecListUrl
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "25879309".to_owned(),
        // },
        // // SecListUrl
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "25925267".to_owned(),
        // },
        // // SecListUrl
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "25539294".to_owned(),
        // },
        // // SecListUrl
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "21422734".to_owned(),
        // },
        // // SecListUrl
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "23220785".to_owned(),
        // },
        // // checkinfo
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "24633080".to_owned(),
        // },
        // // nothing
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "23469260".to_owned(),
        // },
        // // checkinfo
        // Car {
        //     maker_code: "101".to_owned(),
        //     class_code: "1101".to_owned(),
        //     car_seq: "24663799".to_owned(),
        // },
    ];
    let data = pagination::parser::parse(cars);
    Ok(())
}
