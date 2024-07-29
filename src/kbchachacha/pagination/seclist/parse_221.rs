use scraper::Html;
use std::error::Error;

use crate::extractor::extract::{extract_value, with_checked_label};

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    // number
    let title = extract_value(document, "b.issue-number");
    // name
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(2) > :nth-child(2)",
    );
    // grz
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(2) > :nth-child(4)",
    );

    // god
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(3) > :nth-child(2)",
    );

    // srok deistvia tehosmotra
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(3) > :nth-child(4)",
    );

    // vin
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(5) > :nth-child(2)",
    );

    // engine
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(7) > :nth-child(2)",
    );

    // type kpp
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(4) > :nth-child(4) > label",
    );

    // toplivo
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(6) > :nth-child(2) > label",
    );

    // toplivo
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(7) > :nth-child(4) > label",
    );

    Ok(())
}
