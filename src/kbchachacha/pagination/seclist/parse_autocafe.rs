use crate::extractor::extract::{extract_attrs, extract_value, extract_values};
use scraper::Html;
use std::error::Error;

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    // name
    // http://www.autocafe.co.kr/asso/Check_Form_2020.asp?ChkSeq=1704389
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(1) > :nth-child(2)",
    );
    // extended name
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(1) > :nth-child(3)",
    );
    // grz
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(1) > :nth-child(5)",
    );
    // god
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(2) > :nth-child(2)",
    );
    // (4) Срок действия проверки
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(2) > :nth-child(4)",
    );
    // (5) Дата первоначальной регистрации
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(3) > :nth-child(2)",
    );
    // (6) Номер транспортного средства
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(4) > :nth-child(2)",
    );
    // engine
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(6) > :nth-child(2)",
    );
    //extract 1 table
    // need to extractor with sibling text
    println!("{title}");
    Ok(())
}
