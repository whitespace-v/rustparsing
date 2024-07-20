use scraper::Html;
use std::error::Error;

use crate::extractor::extract::extract_value;

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    //clear txt_small
    let title = extract_value(document, "span.txt_small");

    println!("{title}");
    Ok(())
}
