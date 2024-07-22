use scraper::Html;
use std::error::Error;

use crate::extractor::extract::{
    extract_ids_from_json_js, extract_value, extract_values, with_checked, with_checked_label,
};

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    //clear txt_small
    //заголовок
    let title = extract_value(document, "span.txt_small");
    // название авто
    let table1_1 = extract_value(document, "tbody > :nth-child(1) > :nth-child(2)");
    // грз
    let table1_2 = extract_value(document, "tbody > :nth-child(1) > :nth-child(4)");
    // год выпуска
    let table1_3 = extract_value(document, "tbody > :nth-child(2) > :nth-child(2)");
    // Срок действия проверки
    let table1_4 = extract_value(document, "tbody > :nth-child(2) > :nth-child(4)");
    // Дата первой регистрации
    let table1_5 = extract_value(document, "tbody > :nth-child(3) > :nth-child(2)");
    // Номер ходовой части
    let table1_6 = extract_value(document, "tbody > :nth-child(5) > :nth-child(2)");
    // тип кпп
    let table1_7 = with_checked_label(
        document,
        "div.page_line > :nth-child(2) > tbody > :nth-child(3) > :nth-child(4) > label",
    );
    // тип топлива
    let table1_8 = with_checked_label(
        document,
        "div.page_line > :nth-child(2) > tbody > :nth-child(5) > :nth-child(2) > label",
    );
    // тип двигателя
    let table1_9 = extract_value(document, "tbody > :nth-child(6) > :nth-child(2)");
    // Тип гарантии // later -> layout is messed up
    // let table1_10 = with_checked_label(
    //     document,
    //     "div.page_line > :nth-child(2) > tbody > :nth-child(6) > :nth-child(4) > td",
    // );
    println!(
        "
    \ntitle {title} 
    \nname {table1_1}
    \ngrz {table1_2}
    \nyear {table1_3}
    \nsrok {table1_4}
    \ndata {table1_5}
    \nnomer {table1_6:?}
    \nkpp {table1_7:?}
    \ntoplivo {table1_8:?}
    \nengine {table1_9}
    "
    );
    // Пробег и cостояние прибора
    let table2_1 = with_checked_label(
        document,
        "div.page_line > :nth-child(3) > tbody > :nth-child(2) > :nth-child(2) > label",
    );
    // Пробег и cостояние прибора
    let table2_1_1 = with_checked_label(
        document,
        "div.page_line > :nth-child(3) > tbody > :nth-child(3) > :nth-child(1) > label",
    );
    println!(
        "
    \nprobeg {table2_1:?} 
    \nprobeg {table2_1_1:?} 
    "
    );

    // bc check value > 0
    let bc = extract_ids_from_json_js(
        document,
        "body > :last-child",
        "setData('bc', '",
        "');",
        "input[id=bc_",
        "_",
        "]",
        true,
    );
    println!("{:#?}", bc["11"]);

    let dc = extract_ids_from_json_js(
        document,
        "body > :last-child",
        "setData('dc', '",
        "');",
        "input[id=dc_",
        "_",
        "]",
        true,
    );
    println!("{dc:#?}");

    let out = extract_ids_from_json_js(
        document,
        "body > :last-child",
        "var ucAccOutCheck = '",
        "';",
        "img[id=accout_",
        "",
        "]",
        false,
    );
    println!("{out:#?}");

    let bones = extract_ids_from_json_js(
        document,
        "body > :last-child",
        "var ucAccBoneCheck = '",
        "';",
        "img[id=accbone_",
        "",
        "]",
        false,
    );
    println!("{bones:#?}");
    Ok(())
}
