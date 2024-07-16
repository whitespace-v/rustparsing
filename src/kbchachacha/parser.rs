#![warn(clippy::all, clippy::pedantic)]
use crate::kbchachacha::{crawler, maker, structs::CarMaker};
use std::error::Error;

pub fn parse() -> Result<(), Box<dyn Error>> {
    /* Add to Makers: maker_code & class_code */
    // let makers = maker::maker::generate_makers_list()?;
    // /* Add to Makers: total & pages */
    // let makers = crawler::crawler::collect_param_list(makers)?;
    /* Add to Makers: car_seq */
    let makers = CarMaker {
        class_code: "1101".to_owned(),
        maker_code: "101".to_owned(),
        pages: Some(2),
        total: Some(16),
        total_url: None,
    };
    let makers = vec![makers];
    let cars = crawler::crawler::collect_seq_list(makers);
    println!("{cars:#?}");
    // todo: go to -> https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25919156
    Ok(())
}
