use scraper::Html;
use std::error::Error;

use crate::extractor::extract::extract_value;

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    let title = extract_value(document, "span.ckdate");
    // Название автомобиля
    let table1_1 = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(1) > :nth-child(2)",
    );
    // год выпуска
    let table1_2 = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(1) > :nth-child(4)",
    );
    // Номер транспортного средства
    let table2_1 = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(2) > :nth-child(2)",
    );
    // Срок действия проверки
    let table2_2 = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(2) > :nth-child(4)",
    );
    // Дата первой регистрации
    let table3_1 = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(3) > :nth-child(2)",
    );
    // Тип трансмиссии
    let table3_2 = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(3) > :nth-child(4)",
    );
    // Использованное топливо
    let table4_1 = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(4) > :nth-child(2)",
    );
    // Номер ходовой части
    let table4_2 = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(4) > :nth-child(4)",
    );
    // Тип гарантии
    let table5_1 = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(5) > :nth-child(2)",
    );
    // Тип первичного двигателя
    let table5_2 = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(5) > :nth-child(4)",
    );
    // Расчет базовой цены
    let table6_1 = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(6) > :nth-child(2)",
    );
    // table 2
    ////////// Статус
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(1) > :nth-child(2) > span.on",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(2) > :nth-child(2) > span.on",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(3) > :nth-child(2) > span.on",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(4) > :nth-child(2) > span.on",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(5) > :nth-child(2) > span.on",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(6) > :nth-child(2) > span.on",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(7) > :nth-child(2) > span.on",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(8) > :nth-child(2) > span.on",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(9) > :nth-child(2) > span.on",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(10) > :nth-child(2) > span.on",
    );
    //// Изделие/Применимые детали
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(1) > :nth-child(3)",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(2) > :nth-child(3)",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(3) > :nth-child(3)",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(4) > :nth-child(3)",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(5) > :nth-child(3)",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(6) > :nth-child(3)",
    );

    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(7) > :nth-child(3)",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(8) > :nth-child(3)",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(9) > :nth-child(3)",
    );
    let table = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(10) > :nth-child(3)",
    );
    // table 3
    // История несчастных случаев Читать далее
    let table = extract_value(
        document,
        "div.section_repair > table > tbody > :nth-child(1) > td > span.on",
    );
    // Простой ремонт
    let table = extract_value(
        document,
        "div.section_repair > table > tbody > :nth-child(2) > td > span.on",
    );
    // Объем расчета стоимости обследования
и особенности его проведения
    println!("{table}");
    println!(
        "
\n title: {title:?} 
\n table1_1: {table1_1:?}
\n table1_2: {table1_2:?}
\n table2_1: {table2_1:?}
\n table2_2: {table2_2:?}
\n table3_1: {table3_1:?}
\n table3_2: {table3_2:?}
\n table4_1: {table4_1:?}
\n table4_2: {table4_2:?}
\n table5_1: {table5_1:?}
\n table5_2: {table5_2:?}
\n table6_1: {table6_1:?}
"
    );
    Ok(())
}
