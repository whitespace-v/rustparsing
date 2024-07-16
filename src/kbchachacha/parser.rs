#![warn(clippy::all, clippy::pedantic)]
use crate::kbchachacha::{crawler, maker, structs::CarMaker};
use std::error::Error;

pub fn parse() -> Result<(), Box<dyn Error>> {
    /* Add to Makers: maker_code & class_code */
    let makers = maker::maker::generate_makers_list()?;
    /* Add to Makers: total & pages */
    let makers = crawler::crawler::collect_param_list(makers)?;
    /* Add to Makers: car_seq */
    let id_list = crawler::crawler::collect_seq_list(makers);

    Ok(())
}
