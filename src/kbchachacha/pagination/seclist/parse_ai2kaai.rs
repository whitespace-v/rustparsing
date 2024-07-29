use crate::extractor::extract::{
    extract_attrs, extract_ids_from_json_js, extract_value, extract_values,
};
use scraper::Html;
use std::error::Error;

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    // title

    let title = extract_value(
        document,
        "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(12) > td:nth-child(7) > span:nth-child(1)",
    );
    // Название автомобиля
    let title = extract_value(
        document,
        "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(17) > td:nth-child(4) > span:nth-child(1)",
    );
    // grz
    let title = extract_value(
        document,
        "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(17) > td:nth-child(8) > span:nth-child(1)",
    );
    // god
    let title = extract_value(
        document,
        "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(18) > td:nth-child(3) > span:nth-child(1)",
    );
    // srok godnosti
    let title = extract_values(
        document,
        "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(18) > td",
    );
    // Дата первой регистрации
    let title = extract_values(
        document,
        "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(19) > td:nth-child(3) > span:nth-child(1)",
    );
    // vin
    let title = extract_values(
        document,
        "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(20) > td:nth-child(3) > span:nth-child(1)",
    );
    // акпп
    // nedostatochno (potom sverstat)
    // engine
    let title = extract_values(
        document,
        "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(22) > td:nth-child(3) > span:nth-child(1)",
    );
    // 1
    let title = extract_values(
            document,
            "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(45) > td:nth-child(4) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)",
    );
    // 2
    let title = extract_values(
        document,
        "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(46) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)",
    );
    // a
    let title = extract_values(
    document,
    "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(47) > td:nth-child(4) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)",
    );
    // b
    let title = extract_values(document, "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(48) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // c
    let title = extract_values(document, "table.jrPage:nth-child(2) > tbody:nth-child(1) > tr:nth-child(49) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");

    ////////////////////// Самодиагностика
    // Первичный двигатель
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(10) > td:nth-child(4) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Коробка передач
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(11) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    ////////////////////// Первичный двигатель
    // Рабочее состояние (холостой ход)
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(12) > td:nth-child(4) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    ////// Утечка масла Масло
    // Крышка цилиндра (крышка коромысла)
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(13) > td:nth-child(4) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Головка/прокладка уплотнителя
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(14) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Блок цилиндров / Масляный поддон Производитель Китай
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(15) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    //
    // Расход масла
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(16) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    ////// Утечка охлаждающей жидкости
    // Головка блока цилиндров / прокладка
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(17) > td:nth-child(4) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // водяной насос
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(18) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Радиатор
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(19) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Количество охлаждающей жидкости
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(20) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    //
    // Общая магистраль
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(21) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    ////////////////////// Коробка передач
    ////// avtomat
    // Утечка масла Масло
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(22) > td:nth-child(5) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Расход и состояние масла
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(23) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Рабочее состояние (холостой ход)
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(24) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    ////// palka
    // Утечка масла Масло
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(25) > td:nth-child(4) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Переключение передач
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(26) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Расход и состояние масла
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(27) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Рабочее состояние (холостой ход)
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(28) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    ////////////////////// Передача электроэнергии
    // Сцепление в сборе
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(29) > td:nth-child(4) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Шарнир с постоянной скоростью вращения
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(30) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Приводной вал и подшипник
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(31) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Дифференциальная передача

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(32) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    ////////////////////// Рулевое управление
    // Утечка масла при работе гидроусилителя рулевого управления

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(33) > td:nth-child(4) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    /////// Рабочее состояние
    // Насос рулевого управления

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(34) > td:nth-child(4) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Рулевой механизм с MDPS
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(35) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Шарнир рулевого управления
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(36) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Силовой шланг высокого давления
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(37) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Наконечник рулевой тяги и шаровой шарнир
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(38) > td:nth-child(3) > div:nth-child(1) > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(1)");
    ////////////////////// Тормозной
    // Утечка масла из Главного тормозного цилиндра

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(39) > td:nth-child(4) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Утечка тормозного масла

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(40) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Состояние источника питания

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(41) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    ////////////////////// Электричество
    // Выход генератора

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(42) > td:nth-child(4) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Пусковой двигатель

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(43) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Функция двигателя стеклоочистителя

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(44) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Двигатель для вентиляции помещений

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(45) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Двигатель вентилятора радиатора

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(46) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    // Привод стеклоподъемника

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(47) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    ////////////////////// Классические источники Электрическое устройство
    // Состояние изоляции зарядного порта

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(48) > td:nth-child(4) > div:nth-child(1) > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(1)");
    // Состояние изоляции аккумуляторной батареи привода

    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(49) > td:nth-child(3) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1)");
    // Состояние электропроводки высокой мощности (соединительная клемма, ткань, защитный механизм)
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(50) > td:nth-child(2) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(2) > div:nth-child(1) > div:nth-child(2) > table:nth-child(1) > tbody:nth-child(1)");
    ////////////////////// Топливо
    // Утечка топлива (включая сжиженный газ)
    let title = extract_values(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(51) > td:nth-child(4) > div:nth-child(1) > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(1)");
    //
    // комментарий
    //
    let title = extract_value(document, "table.jrPage:nth-child(6) > tbody:nth-child(1) > tr:nth-child(61) > td:nth-child(5) > div:nth-child(1) > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(1) > span:nth-child(1)");
    // дата проверки
    let title = extract_value(document, "table.jrPage:nth-child(10) > tbody:nth-child(1) > tr:nth-child(8) > td:nth-child(4) > span:nth-child(1)");
    // imgs
    let title = extract_attrs(document,
         "style",
         "table.jrPage:nth-child(14) > tbody:nth-child(1) > tr:nth-child(8) > td:nth-child(2) > div:nth-child(1) > div:nth-child(2) > table > tbody > tr > td > div > div > table > tbody > tr > td > div > div",
        );
    println!("{title:?}");
    Ok(())
}
