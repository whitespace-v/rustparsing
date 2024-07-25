use scraper::Html;
use std::error::Error;

use crate::extractor::extract::{extract_ids_from_json_js, extract_value, extract_values};

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
    // Объем расчета стоимости обследования и особенности его проведения
    let table = extract_value(
        document,
        "div.section_repair > table > tbody > :nth-child(3) > :nth-child(2)",
    );
    let table = extract_value(
        document,
        "div.section_repair > table > tbody > :nth-child(3) > :nth-child(3)",
    );

    // table 4 (image)
    // 1
    let big = extract_ids_from_json_js(
        document,
        "body > :last-child",
        "performanceCheck.init({
	data : ",
        "
});",
        " ",
        " ",
        " ",
        true,
    );

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
\n big: {big:?}
"
    );
    //////////// 1/ Самодиагностика
    // Первичный двигатель
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(1) > td > span.on",
    );
    // Коробка передач
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(2) > td > span.on",
    );
    /////////// 2/ Первичный двигатель
    // Рабочее состояние (холостой ход)
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(3) > td > span.on",
    );
    ////// Утечка масла Масло
    // Крышка цилиндра (крышка коромысла)
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(4) > td > span.on",
    );
    // Головка блока цилиндров/прокладка
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(5) > td > span.on",
    );
    // Блок цилиндров / Масляный поддон Производитель Китай
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(6) > td > span.on",
    );
    //
    // Поток масла
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(7) > td > span.on",
    );
    ///// Утечка охлаждающей жидкости
    // Головка блока цилиндров/прокладка
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(8) > td > span.on",
    );
    // водяной насос
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(9) > td > span.on",
    );
    // Радиатор
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(10) > td > span.on",
    );
    // Количество охлаждающей жидкости
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(11) > td > span.on",
    );
    //
    // Общая магистраль
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(12) > td > span.on",
    );
    //////////// 3 Коробка передач
    ///// Автоматическая коробка передач (A/T)
    // Утечка масла Масло
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(13) > td > span.on",
    );
    // Расход и состояние масла
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(14) > td > span.on",
    );
    // Рабочее состояние (холостой ход)
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(15) > td > span.on",
    );
    /////////// 4 Передача электроэнергии
    // Сцепление в сборе
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(16) > td > span.on",
    );
    // Шарнир с постоянной скоростью вращения
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(17) > td > span.on",
    );
    // Приводной вал и подшипник
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(18) > td > span.on",
    );
    // Дифференциальная передача
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(19) > td > span.on",
    );
    /////////// 5 Рулевое управление
    // Утечка масла при работе гидроусилителя рулевого управления
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(20) > td > span.on",
    );
    ///// Рабочее состояние
    // Насос рулевого управления
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(21) > td > span.on",
    );
    // Рулевой механизм с MDPS
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(22) > td > span.on",
    );
    // Шарнир рулевого управления
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(23) > td > span.on",
    );
    // Силовой шланг высокого давления
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(24) > td > span.on",
    );
    // Наконечник рулевой тяги и Шаровой шарнир
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(25) > td > span.on",
    );
    /////////// 6 Тормозной
    // Утечка масла из Главного тормозного цилиндра
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(26) > td > span.on",
    );
    // Утечка тормозного масла
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(27) > td > span.on",
    );
    // Состояние источника питания
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(28) > td > span.on",
    );
    ////////// 7 Электричество
    // Выход генератора
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(29) > td > span.on",
    );
    // Пусковой двигатель
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(30) > td > span.on",
    );
    // Функция двигателя стеклоочистителя
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(31) > td > span.on",
    );
    // Двигатель для вентиляции помещений
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(32) > td > span.on",
    );
    // Двигатель вентилятора радиатора
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(33) > td > span.on",
    );
    // Привод стеклоподъемника
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(34) > td > span.on",
    );
    /////////// 8 Топливо
    // Утечка топлива (включая сжиженный газ)
    let table = extract_value(
        document,
        "div.section_detail > table.tbl_detail > tbody > :nth-child(35) > td > span.on",
    );
    // Особенности и мнения инспекторов
    let table = extract_value(document, "div.section_opinion > table > tbody > tr > td");
    // дата проверки
    let date = extract_value(document, "p.date");
    println!("{date}");
    Ok(())
}
