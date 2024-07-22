use crate::extractor::extract::{
    extract_attrs, extract_ids_from_json_js, extract_value, extract_values, extract_with_sibling,
    with_checked_label,
};
use scraper::Html;
use std::{collections::HashMap, error::Error, vec};
// сохранить изображение схемы вручную на каждый аукционный лист что бы не тянуть дохуя картинок
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
    let table1_6 = extract_value(document, "tbody > :nth-child(4) > :nth-child(2)");
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
    // тип гарантии // если есть -> название компании, если пусто -> самогарантия
    let table1_10 = extract_value(
        document,
        "div.page_line > :nth-child(2) > tbody > :nth-child(6) > :nth-child(4) > span",
    );
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
    \nwarranty {table1_10:?}
    "
    );

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
    let eac = extract_ids_from_json_js(
        document,
        "body > :last-child",
        "var ucEac = '",
        "';",
        "input[id=eac_",
        "_",
        "]",
        true,
    );
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
    // Пробег и cостояние прибора
    let table2_1 = extract_with_sibling(document, &bc["11"]);
    let table2_1_1 = extract_with_sibling(document, &bc["12"]);
    // пробег в км
    let table2_1_2 = extract_value(document, "strong.km");
    // Обозначение номера транспортного средства
    let table2_2 = extract_with_sibling(document, &bc["2"]);
    // Выбросы
    let table2_3 = with_checked_label(
        document,
        "div.page_line > :nth-child(3) > tbody > :nth-child(5) > :nth-child(2) > label",
    );
    // Выбросы показатели
    let table2_3_1 = extract_values(
        document,
        "div.page_line > :nth-child(3) > tbody > :nth-child(5) > :nth-child(3) > span",
    );
    // тюнинг модификации
    //// есть нет
    let table2_4_1 = extract_with_sibling(document, &bc["3"]);
    //// законность
    // let table2_4_2 = extract_with_sibling(document, &bc["31"]);
    //// устройство / структура
    // let table2_4_3 = extract_with_sibling(document, &bc["32"]);
    // особая история
    //// есть нет
    let table2_5_1 = extract_with_sibling(document, &bc["4"]);
    //// огонь пожар
    // let table2_5_2 = extract_with_sibling(document, &bc["41"]);
    // Изменение способа использования
    //// есть нет
    let table2_6_1 = extract_with_sibling(document, &bc["5"]);
    //// продажа аренда
    // let table2_6_2 = extract_with_sibling(document, &bc["51"]);
    // цвет
    ///// хром ахром
    //let table2_7_1 = extract_with_sibling(document, &bc["61"]);
    ///// полноцветный / изменение цвета
    //let table2_7_2 = extract_with_sibling(document, &bc["62"]);
    // основные опции
    //// есть нет
    //let table2_8_1 = extract_with_sibling(document, &bc["7"]);
    //// люк на крыше навигация другое
    //let table2_8_1 = extract_with_sibling(document, &bc["71"]);
    // подлежит отзыву
    //// применимо неприминимо
    //let table2_9_1 = extract_with_sibling(document, &bc["81"]);
    //// выполнение/ неработающий
    //let table2_9_1 = extract_with_sibling(document, &bc["82"]);
    println!(
        "
    \nprobeg {table2_1:?} 
    \nprobeg {table2_1_1:?} 
    \nprobeg.km: {table2_1_2:?}
    \nobozna {table2_2}
    \nvibrosy {table2_3:?} -> {table2_3_1:?}
    \ntuning {table2_4_1} 
    \nspec.exp {table2_5_1}
    "
    );
    // история несчастный случаев -Есть если есть простой ремонт -> (нужен / не нужен)
    let table3_1 = with_checked_label(
        document,
        "div.page_line > :nth-child(4) > tbody > :nth-child(3) > :nth-child(4) > label",
    );
    println!(
        "
    \nhistory {table3_1:?} 
    "
    );
    //// X - Обмен
    //// W - Листовой металл или сварка
    //// A - Царапины
    //// U - Неровности
    //// C - Коррозия
    //// T - Ущерб
    //// внешка
    //// 1 класс - 1,2,3,4,5
    //// 2 класс - 6,7,8
    //// скелет
    //// A класс - 9,10,11,17,18
    //// B класс - 12,13,14,19
    //// C класс - 15,16
    let mut out_s: HashMap<String, String> = HashMap::new();
    for i in out {
        let t = extract_with_sibling(document, &i.0);
        out_s.insert(t, i.1);
    }

    let mut bones_s: HashMap<String, String> = HashMap::new();
    for i in bones {
        let t = extract_with_sibling(document, &i.0);
        bones_s.insert(t, i.1);
    }
    println!("{bones_s:?}");
    println!("{out_s:?}");
    // Самодиагностика
    /////////// Первичный двигатель
    let table3_1 = extract_with_sibling(document, &dc["11"]);
    // Коробка передач
    let table3_1_1 = extract_with_sibling(document, &dc["12"]);
    // Рабочее состояние (холостой ход)
    let table3_2_1 = extract_with_sibling(document, &dc["21"]);
    // Утечка масла
    //// Крышка цилиндра (крышка коромысла)
    let table3_3_1 = extract_with_sibling(document, &dc["221"]);
    //// Головка блока цилиндров / прокладка
    let table3_3_2 = extract_with_sibling(document, &dc["222"]);
    //// Блок цилиндров / Масляный поддон Производитель Китай
    let table3_3_3 = extract_with_sibling(document, &dc["223"]);
    //// Расход масла
    let table3_4_1 = extract_with_sibling(document, &dc["23"]);
    //// Охлаждающая жидкость Утечки
    //// Головка блока цилиндров / прокладка
    let table3_5_1 = extract_with_sibling(document, &dc["231"]);
    //// водяной насос
    let table3_5_1 = extract_with_sibling(document, &dc["232"]);
    //// Радиатор
    let table3_5_1 = extract_with_sibling(document, &dc["233"]);
    //// Количество охлаждающей жидкости
    let table3_5_1 = extract_with_sibling(document, &dc["234"]);
    //Общая магистраль
    let table3_5_1 = extract_with_sibling(document, &dc["24"]);
    //////// Коробка передач
    //// АКПП
    /// Утечка масла Масло
    let table3_5_1 = extract_with_sibling(document, &dc["311"]);
    /// Расход и состояние масла
    let table3_5_1 = extract_with_sibling(document, &dc["312"]);
    /// Рабочее состояние (холостой ход)
    let table3_5_1 = extract_with_sibling(document, &dc["313"]);
    //// МКПП
    /// Утечка масла Масло
    let table3_5_1 = extract_with_sibling(document, &dc["321"]);
    /// Переключение передач
    let table3_5_1 = extract_with_sibling(document, &dc["322"]);
    /// Расход и состояние масла
    let table3_5_1 = extract_with_sibling(document, &dc["323"]);
    /// Рабочее состояние (холостой ход)
    let table3_5_1 = extract_with_sibling(document, &dc["324"]);
    /////Передача электроэнергии
    //Сцепление в сборе
    let table3_5_1 = extract_with_sibling(document, &dc["41"]);
    // Соединение с постоянной скоростью
    let table3_5_1 = extract_with_sibling(document, &dc["42"]);
    // Приводной вал и подшипник
    let table3_5_1 = extract_with_sibling(document, &dc["43"]);
    // Дифференциальная передача
    let table3_5_1 = extract_with_sibling(document, &dc["44"]);
    ///////////////Рулевое управление
    /// Утечка масла при работе гидроусилителя рулевого управления
    let table3_5_1 = extract_with_sibling(document, &dc["51"]);
    /// Рабочее состояние
    /// Насос рулевого управления
    let table3_5_1 = extract_with_sibling(document, &dc["522"]);
    /// Рулевой механизм с MDPS
    let table3_5_1 = extract_with_sibling(document, &dc["521"]);
    /// Шарнир рулевого управления
    let table3_5_1 = extract_with_sibling(document, &dc["524"]);
    /// Силовой шланг высокого давления
    let table3_5_1 = extract_with_sibling(document, &dc["525"]);
    /// Наконечник рулевой тяги и шаровой шарнир
    let table3_5_1 = extract_with_sibling(document, &dc["523"]);
    /////////////////// Тормозная система
    /// Утечка масла из Главного тормозного цилиндра
    let table3_5_1 = extract_with_sibling(document, &dc["61"]);
    /// Утечка тормозного масла
    let table3_5_1 = extract_with_sibling(document, &dc["62"]);
    /// Состояние источника питания
    let table3_5_1 = extract_with_sibling(document, &dc["63"]);
    ////////////// Электричество
    /// Выход генератора
    let table3_5_1 = extract_with_sibling(document, &dc["71"]);
    /// Пусковой двигатель
    let table3_5_1 = extract_with_sibling(document, &dc["72"]);
    /// Функция двигателя стеклоочистителя
    let table3_5_1 = extract_with_sibling(document, &dc["73"]);
    /// Двигатель для вентиляции помещений
    let table3_5_1 = extract_with_sibling(document, &dc["74"]);
    /// Двигатель вентилятора радиатора
    let table3_5_1 = extract_with_sibling(document, &dc["75"]);
    /// Привод стеклоподъемника
    let table3_5_1 = extract_with_sibling(document, &dc["76"]);
    ////////////////////////// Классические источники Электрическое устройство
    /// Состояние изоляции зарядного порта
    let table3_5_1 = extract_with_sibling(document, &dc["91"]);
    /// Состояние изоляции аккумуляторной батареи привода
    let table3_5_1 = extract_with_sibling(document, &dc["92"]);
    /// Состояние электропроводки высокой мощности (соединительная клемма, ткань, защитный механизм)
    let table3_5_1 = extract_with_sibling(document, &dc["93"]);
    /////////// Топливо
    /// Утечка топлива (включая сжиженный газ)
    let table3_5_1 = extract_with_sibling(document, &dc["81"]);

    ////////////table 4
    ///////   Требуется ремонт
    /// Внешний вид
    let table3_5_1 = extract_with_sibling(document, &eac["1"]);
    /// Встроенный
    let table3_5_1 = extract_with_sibling(document, &eac["2"]);
    /// Блеск
    let table3_5_1 = extract_with_sibling(document, &eac["3"]);
    /// Уборка помещений
    let table3_5_1 = extract_with_sibling(document, &eac["4"]);
    /// Колесо
    let table3_5_1 = extract_with_sibling(document, &eac["5"]);
    /// Водительское сиденье
    /// до
    let table3_5_1 = extract_with_sibling(document, &eac["51"]);
    // после
    let table3_5_1 = extract_with_sibling(document, &eac["52"]);
    // пассажирское
    // до
    let table3_5_1 = extract_with_sibling(document, &eac["53"]);
    // после
    let table3_5_1 = extract_with_sibling(document, &eac["54"]);
    // чрезвычайный случай
    let table3_5_1 = extract_with_sibling(document, &eac["55"]);
    /// Шины
    let table3_5_1 = extract_with_sibling(document, &eac["6"]);
    /// Водительское сиденье
    /// до
    let table3_5_1 = extract_with_sibling(document, &eac["61"]);
    // после
    let table3_5_1 = extract_with_sibling(document, &eac["62"]);
    // пассажирское
    // до
    let table3_5_1 = extract_with_sibling(document, &eac["63"]);
    // после
    let table3_5_1 = extract_with_sibling(document, &eac["64"]);
    // чрезвычайная ситуация
    let table3_5_1 = extract_with_sibling(document, &eac["65"]);
    /// Стекло
    let table3_5_1 = extract_with_sibling(document, &eac["7"]);
    /// Основные предметы
    ///  Статус удержания
    /// нет есть
    let table3_5_1 = extract_with_sibling(document, &eac["8"]);
    // Руководство по эксплуатации, джек
    let table3_5_1 = extract_with_sibling(document, &eac["83"]);
    // Защитный штатив
    let table3_5_1 = extract_with_sibling(document, &eac["84"]);

    // Особенности и мнения инспекторов
    let table3_5_1 = extract_value(
        document,
        "div.page_col2 > div.page_line > table.fuc_normal > tbody > : nth-child(1) > td",
    );
    let table3_5_1 = extract_value(
        document,
        "div.page_col2 > div.page_line > table.fuc_normal > tbody > : nth-child(2) > td",
    );
    // фотографии
    let table7_1 = extract_attrs(document, "src", "table.height_set3 > tbody > tr > td > img");
    println!("{table7_1:?}");
    Ok(())
}
