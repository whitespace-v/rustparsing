use scraper::Html;
use std::error::Error;

use crate::extractor::extract::{extract_attr, extract_attrs, extract_value, extract_values};

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    let resp: String = ureq::post("https://www.kbchachacha.com/public/layer/car/check/info.kbc")
        .send_form(&[
            ("layerId", "layerCarCheckInfo"),
            ("carSeq", "24633080"),
            ("diagCarYn", "N"),
            ("diagCarSeq", ""),
            ("premiumCarYn", "N"),
        ])
        .unwrap()
        .into_string()
        .unwrap();
    let document = &scraper::Html::parse_document(&resp);
    // name
    let title = extract_value(document, "div.ch-car-name");
    // number
    let title = extract_value(document, "div.ch-car-data > span.cd-right > span.txt-ho");
    // date of seclist
    let title = extract_value(
        document,
        "div.ch-car-data > span.cd-left > div.data-line > :nth-child(1)",
    );
    // probeg
    let title = extract_value(
        document,
        "div.ch-car-data > span.cd-left > div.data-line > :nth-child(2)",
    );
    // toplivo
    let title = extract_value(
        document,
        "div.ch-car-data > span.cd-left > div.data-line > :nth-child(3)",
    );
    // city
    let title = extract_value(
        document,
        "div.ch-car-data > span.cd-left > div.data-line > :nth-child(4)",
    );
    // Название автомобиля
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(1) > td",
    );
    // grz
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(2) > td",
    );
    // god vypuska
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(3) > td",
    );
    // Срок действия проверки
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(4) > td",
    );
    // Дата первой регистрации
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(5) > td",
    );
    // Номер ходовой части
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(6) > td",
    );
    // Типы коробок передач
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(7) > td > div",
    );
    // Использовать топливо
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(8) > td > div",
    );
    // Тип первичного двигателя
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(7) > td",
    );
    // Тип гарантии
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(10) > td > div",
    );
    // Баланс цен Справочная цена
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(11) > td",
    );
    // Состояние датчика пробега
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(1) > td > div",
    );
    // Статус пробега
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(2) > :nth-child(2) > div",
    );
    // probeg
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(1) > :nth-child(3)",
    );
    // Обозначение номера транспортного средства
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(3) > :nth-child(2) > div",
    );
    // vibrosy nazvanie
    let title = extract_values(
        document,
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(4) > :nth-child(2) > div",
    );
    // // pokazateli vibrosov
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(4) > :nth-child(3)",
    );
    // tuning
    let title = extract_attrs(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(5) > :nth-child(2) > div",
    );
    // Особая история
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(6) > :nth-child(2) > div",
    );
    // navodnenie / ogon
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(6) > :nth-child(3) > div",
    );
    // Изменение способа использования
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(7) > :nth-child(2) > div",
    );
    // arenda / prodaja
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(7) > :nth-child(3) > div",
    );
    // Цвет hrom/ahrom
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(8) > :nth-child(2) > div",
    );
    // polnocvet / Изменение цвета
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(8) > :nth-child(3) > div",
    );
    // Основные опции
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(9) > :nth-child(3) > div",
    );
    println!("{title:?}");
    // navigacia / luk / drugoe
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(9) > :nth-child(3) > div",
    );
    // table 4
    // История несчастных случаев (Примечание 5)
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(12) > table > tbody > :nth-child(1) > :nth-child(2) > div",
    );
    // Простой ремонт
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(12) > table > tbody > :nth-child(1) > :nth-child(4) > div",
    );
    // Наружная пластина 1 ранга
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(12) > table > tbody > :nth-child(2) > :nth-child(3) > div",
    );
    // Наружная пластина 2 ранга
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(12) > table > tbody > :nth-child(3) > :nth-child(2) > div",
    );
    // Основной блок
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(12) > table > tbody > :nth-child(4) > :nth-child(2) > div",
    );
    // дополнение к основному блоку
    let title = extract_values(
        document,
        "div.cmm-table:nth-child(12) > table > tbody > :nth-child(4) > :nth-child(2) > div > span.on",
    );
    // ранги
    let title = extract_attrs(
        document,
        "class",
        "div.repair-check-area > div > span > span:not(.blind)",
    );
    let title = extract_values(
        document,
        "div.repair-check-area > div > span > span:not(.blind)",
    );
    let title = extract_attr(document, "value", "input[id=carCheck]");
    let mut s = title.split("").collect::<Vec<&str>>();
    s.remove(0);
    s.pop();
    // X - Замена, C - Коррозия, W - листовой металл/сварка, A - Царапины, U - неровности, T - повреждения
    //
    // 1 -  1. Капот 2. Передняя планка 3. Дверь 4. Решетка багажника 5. Опора радиатора (детали корпуса на болтах)
    // 2 - 6. Боковая панель (задняя часть) 7. Петлевая панель 8. Боковая панель салона
    // a - 9. Передняя панель 10. Поперечины 11. Внутренняя панель 17. Пол багажника 18. Задняя панель
    // b - 12. Боковые элементы 13. Рулевая рубка 14. Панель наполнителя (A, B, C) 19. Лоток для упаковки
    // c - 15. Приборная панель 16. Панель пола
    let names: Vec<&str> = vec![
        // 1
        "Капот",
        // 2
        "Передняя планка",
        "Передняя планка",
        // 3
        "Дверь",
        "Дверь",
        "Дверь",
        "Дверь",
        // 4
        "Решетка багажника",
        // 5
        "Опора радиатора",
        // 6
        "Боковая панель (задняя часть)",
        "Боковая панель (задняя часть)",
        // 7
        "Петлевая панель",
        // 8
        "Боковая панель салона",
        "Боковая панель салона",
        // 9
        "Передняя панель",
        // 10
        "Поперечины",
        // 11
        "Внутренняя панель",
        "Внутренняя панель",
        // 12
        "Боковые элементы",
        "Боковые элементы",
        "Боковые элементы",
        "Боковые элементы",
        // 13
        "Рулевая рубка",
        "Рулевая рубка",
        "Рулевая рубка",
        "Рулевая рубка",
        // 14
        "Панель наполнителя (A, B, C)",
        "Панель наполнителя (A, B, C)",
        "Панель наполнителя (A, B, C)",
        "Панель наполнителя (A, B, C)",
        "Панель наполнителя (A, B, C)",
        "Панель наполнителя (A, B, C)",
        // 15
        "Приборная панель",
        "", // 34
        // 16
        "Панель пола",
        "", // 36
        // 17
        "Пол багажника",
        // 18
        "Задняя панель",
        // 19
        "Лоток для упаковки",
    ];
    for (idx, mark) in s.iter().enumerate() {
        match mark.to_owned() {
            " " => {}
            _ => {
                println!("[{}]- {}", mark, names[idx]);
            }
        }
    }
    Ok(())
}
