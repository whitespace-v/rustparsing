use crate::extractor::extract::extract_ids_from_json_js;
use scraper::Html;
use std::error::Error;

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    let big = extract_ids_from_json_js(
        document,
        "body > :last-child",
        "var data = ",
        ";",
        "",
        "",
        "",
        false,
    );

    println!("{big:?}");
    Ok(())
}
