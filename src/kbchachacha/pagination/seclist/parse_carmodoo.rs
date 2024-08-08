use super::structs::CarSecList;
use crate::extractor::extract::{
    extract_attrs, extract_ids_from_json_js, extract_value, extract_values, extract_with_sibling,
    with_checked_label,
};
use scraper::Html;
use std::{collections::HashMap, error::Error};

pub fn parse(document: &Html) -> Result<CarSecList, Box<dyn Error>> {
    // номер
    let seclist_num = extract_value(document, "span.num");
    // название авто
    let name = extract_value(document, "tbody > :nth-child(1) > :nth-child(2)");
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
    let mut table2_4_1: String = "".to_owned();
    if bc.contains_key("3") {
        table2_4_1 = extract_with_sibling(document, &bc["3"]);
    }
    //// законность
    let mut table2_4_2: String = "".to_owned();
    if bc.contains_key("31") {
        table2_4_2 = extract_with_sibling(document, &bc["31"]);
    }

    //// устройство / структура
    let mut table2_4_3: String = "".to_owned();
    if bc.contains_key("32") {
        table2_4_3 = extract_with_sibling(document, &bc["32"]);
    }
    // особая история
    //// есть нет
    let mut table2_5_1: String = "".to_owned();
    if bc.contains_key("4") {
        table2_5_1 = extract_with_sibling(document, &bc["4"]);
    }
    //// огонь пожар
    let mut table2_5_2: String = "".to_owned();
    if bc.contains_key("41") {
        table2_5_2 = extract_with_sibling(document, &bc["41"]);
    }
    // Изменение способа использования
    //// есть нет
    let mut table2_6_1: String = "".to_owned();
    if bc.contains_key("5") {
        table2_6_1 = extract_with_sibling(document, &bc["5"]);
    }
    //// продажа аренда
    let mut table2_6_2: String = "".to_owned();
    if bc.contains_key("51") {
        table2_6_2 = extract_with_sibling(document, &bc["51"]);
    }
    // цвет
    ///// хром ахром
    let mut table2_7_1: String = "".to_owned();
    if bc.contains_key("61") {
        table2_7_1 = extract_with_sibling(document, &bc["61"]);
    }

    ///// полноцветный / изменение цвета
    let mut table2_7_2: String = "".to_owned();
    if bc.contains_key("62") {
        table2_7_2 = extract_with_sibling(document, &bc["62"]);
    }

    // основные опции
    //// есть нет
    let mut table2_8_1: String = "".to_owned();
    if bc.contains_key("7") {
        table2_8_1 = extract_with_sibling(document, &bc["7"]);
    }
    //// люк на крыше навигация другое
    let mut table2_8_2: String = "".to_owned();
    if bc.contains_key("71") {
        table2_8_2 = extract_with_sibling(document, &bc["71"]);
    }
    // подлежит отзыву
    //// применимо неприминимо
    let mut table2_9_1: String = "".to_owned();
    if bc.contains_key("81") {
        table2_9_1 = extract_with_sibling(document, &bc["81"]);
    }

    //// выполнение/ неработающий
    let mut table2_9_2: String = "".to_owned();
    if bc.contains_key("82") {
        table2_9_2 = extract_with_sibling(document, &bc["82"]);
    }
    println!(
        "
\nprobeg {table2_1:?} 
\nprobeg {table2_1_1:?} 
\nprobeg.km: {table2_1_2:?}
\nobozna {table2_2}
\nvibrosy {table2_3:?} -> {table2_3_1:?}
\ntuning {table2_4_1}
\nspec.exp {table2_5_1}
\n table2_4_2: {table2_4_2:?}
\n table2_4_3: {table2_4_3:?}
\n table2_5_2: {table2_5_2:?}
\n table2_6_1: {table2_6_1:?}
\n table2_6_2: {table2_6_2:?}
\n table2_7_1: {table2_7_1:?}
\n table2_7_2: {table2_7_2:?}
\n table2_8_1: {table2_8_1:?}
\n table2_8_2: {table2_8_2:?}
\n table2_9_1: {table2_9_1:?}
\n table2_9_2: {table2_9_2:?}
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
    //// X - Замена детали
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
    let mut table31: String = "".to_owned();
    if dc.contains_key("11") {
        table31 = extract_with_sibling(document, &dc["11"]);
    }
    // Коробка передач
    let mut table32: String = "".to_owned();
    if dc.contains_key("12") {
        table32 = extract_with_sibling(document, &dc["12"]);
    }
    // Рабочее состояние (холостой ход)
    let mut table33: String = "".to_owned();
    if dc.contains_key("21") {
        table33 = extract_with_sibling(document, &dc["21"]);
    }
    // Утечка масла
    //// Крышка цилиндра (крышка коромысла)
    let mut table34: String = "".to_owned();
    if dc.contains_key("221") {
        table34 = extract_with_sibling(document, &dc["221"]);
    }
    //// Головка блока цилиндров / прокладка
    let mut table35: String = "".to_owned();
    if dc.contains_key("222") {
        table35 = extract_with_sibling(document, &dc["222"]);
    }
    //// Блок цилиндров / Масляный поддон Производитель Китай
    let mut table36: String = "".to_owned();
    if dc.contains_key("223") {
        table36 = extract_with_sibling(document, &dc["223"]);
    }
    //// Расход масла
    let mut table37: String = "".to_owned();
    if dc.contains_key("23") {
        table37 = extract_with_sibling(document, &dc["23"]);
    }
    //// Охлаждающая жидкость Утечки
    //// Головка блока цилиндров / прокладка
    let mut table38: String = "".to_owned();
    if dc.contains_key("231") {
        table38 = extract_with_sibling(document, &dc["231"]);
    }
    //// водяной насос
    let mut table39: String = "".to_owned();
    if dc.contains_key("232") {
        table39 = extract_with_sibling(document, &dc["232"]);
    }
    //// Радиатор
    let mut table40: String = "".to_owned();
    if dc.contains_key("233") {
        table40 = extract_with_sibling(document, &dc["233"]);
    }
    //// Количество охлаждающей жидкости
    let mut table41: String = "".to_owned();
    if dc.contains_key("234") {
        table41 = extract_with_sibling(document, &dc["234"]);
    }
    //Общая магистраль
    let mut table42: String = "".to_owned();
    if dc.contains_key("24") {
        table42 = extract_with_sibling(document, &dc["24"]);
    }
    //////// Коробка передач
    //// АКПП
    /// Утечка масла Масло
    let mut table43: String = "".to_owned();
    if dc.contains_key("311") {
        table43 = extract_with_sibling(document, &dc["311"]);
    }
    /// Расход и состояние масла
    let mut table44: String = "".to_owned();
    if dc.contains_key("312") {
        table44 = extract_with_sibling(document, &dc["312"]);
    }
    /// Рабочее состояние (холостой ход)
    let mut table45: String = "".to_owned();
    if dc.contains_key("313") {
        table45 = extract_with_sibling(document, &dc["313"]);
    }
    //// МКПП
    /// Утечка масла Масло
    let mut table46: String = "".to_owned();
    if dc.contains_key("321") {
        table46 = extract_with_sibling(document, &dc["321"]);
    }
    /// Переключение передач
    let mut table47: String = "".to_owned();
    if dc.contains_key("322") {
        table47 = extract_with_sibling(document, &dc["322"]);
    }
    /// Расход и состояние масла
    let mut table48: String = "".to_owned();
    if dc.contains_key("323") {
        table48 = extract_with_sibling(document, &dc["323"]);
    }
    /// Рабочее состояние (холостой ход)
    let mut table49: String = "".to_owned();
    if dc.contains_key("324") {
        table49 = extract_with_sibling(document, &dc["324"]);
    }
    /////Передача электроэнергии
    //Сцепление в сборе
    let mut table50: String = "".to_owned();
    if dc.contains_key("41") {
        table50 = extract_with_sibling(document, &dc["41"]);
    }
    // Соединение с постоянной скоростью
    let mut table51: String = "".to_owned();
    if dc.contains_key("42") {
        table51 = extract_with_sibling(document, &dc["42"]);
    }
    // Приводной вал и подшипник
    let mut table52: String = "".to_owned();
    if dc.contains_key("43") {
        table52 = extract_with_sibling(document, &dc["43"]);
    }
    // Дифференциальная передача
    let mut table53: String = "".to_owned();
    if dc.contains_key("44") {
        table53 = extract_with_sibling(document, &dc["44"]);
    }
    ///////////////Рулевое управление
    /// Утечка масла при работе гидроусилителя рулевого управления
    let mut table54: String = "".to_owned();
    if dc.contains_key("51") {
        table54 = extract_with_sibling(document, &dc["51"]);
    }
    /// Рабочее состояние
    /// Насос рулевого управления
    let mut table55: String = "".to_owned();
    if dc.contains_key("522") {
        table55 = extract_with_sibling(document, &dc["522"]);
    }
    /// Рулевой механизм с MDPS
    let mut table56: String = "".to_owned();
    if dc.contains_key("521") {
        table56 = extract_with_sibling(document, &dc["521"]);
    }
    /// Шарнир рулевого управления
    let mut table57: String = "".to_owned();
    if dc.contains_key("524") {
        table57 = extract_with_sibling(document, &dc["524"]);
    }
    /// Силовой шланг высокого давления
    let mut table58: String = "".to_owned();
    if dc.contains_key("525") {
        table58 = extract_with_sibling(document, &dc["525"]);
    }
    /// Наконечник рулевой тяги и шаровой шарнир
    let mut table59: String = "".to_owned();
    if dc.contains_key("523") {
        table59 = extract_with_sibling(document, &dc["523"]);
    }
    /////////////////// Тормозная система
    /// Утечка масла из Главного тормозного цилиндра
    let mut table60: String = "".to_owned();
    if dc.contains_key("61") {
        table60 = extract_with_sibling(document, &dc["61"]);
    }
    /// Утечка тормозного масла
    let mut table61: String = "".to_owned();
    if dc.contains_key("62") {
        table61 = extract_with_sibling(document, &dc["62"]);
    }
    /// Состояние источника питания
    let mut table62: String = "".to_owned();
    if dc.contains_key("63") {
        table62 = extract_with_sibling(document, &dc["63"]);
    }
    ////////////// Электричество
    /// Выход генератора
    let mut table63: String = "".to_owned();
    if dc.contains_key("71") {
        table63 = extract_with_sibling(document, &dc["71"]);
    }
    /// Пусковой двигатель
    let mut table64: String = "".to_owned();
    if dc.contains_key("72") {
        table64 = extract_with_sibling(document, &dc["72"]);
    }
    /// Функция двигателя стеклоочистителя
    let mut table65: String = "".to_owned();
    if dc.contains_key("73") {
        table65 = extract_with_sibling(document, &dc["73"]);
    }
    /// Двигатель для вентиляции помещений
    let mut table66: String = "".to_owned();
    if dc.contains_key("74") {
        table66 = extract_with_sibling(document, &dc["74"]);
    }
    /// Двигатель вентилятора радиатора
    let mut table67: String = "".to_owned();
    if dc.contains_key("75") {
        table67 = extract_with_sibling(document, &dc["75"]);
    }
    /// Привод стеклоподъемника
    let mut table68: String = "".to_owned();
    if dc.contains_key("76") {
        table68 = extract_with_sibling(document, &dc["76"]);
    }
    ////////////////////////// Классические источники Электрическое устройство
    /// Состояние изоляции зарядного порта
    let mut table69: String = "".to_owned();
    if dc.contains_key("91") {
        table69 = extract_with_sibling(document, &dc["91"]);
    }
    /// Состояние изоляции аккумуляторной батареи привода
    let mut table70: String = "".to_owned();
    if dc.contains_key("92") {
        table70 = extract_with_sibling(document, &dc["92"]);
    }
    /// Состояние электропроводки высокой мощности (соединительная клемма, ткань, защитный механизм)
    let mut table71: String = "".to_owned();
    if dc.contains_key("93") {
        table71 = extract_with_sibling(document, &dc["93"]);
    }
    /////////// Топливо
    /// Утечка топлива (включая сжиженный газ)
    let mut table72: String = "".to_owned();
    if dc.contains_key("81") {
        table72 = extract_with_sibling(document, &dc["81"]);
    }

    ////////////table 4
    ///////   Требуется ремонт
    /// Внешний вид
    let mut table73: String = "".to_owned();
    if eac.contains_key("1") {
        table73 = extract_with_sibling(document, &eac["1"]);
    }
    /// Встроенный
    let mut table74: String = "".to_owned();
    if eac.contains_key("2") {
        table74 = extract_with_sibling(document, &eac["2"]);
    }
    /// Блеск
    let mut table75: String = "".to_owned();
    if eac.contains_key("3") {
        table75 = extract_with_sibling(document, &eac["3"]);
    }
    /// Уборка помещений
    let mut table76: String = "".to_owned();
    if eac.contains_key("4") {
        table76 = extract_with_sibling(document, &eac["4"]);
    }
    /// Колесо
    let mut table77: String = "".to_owned();
    if eac.contains_key("5") {
        table77 = extract_with_sibling(document, &eac["5"]);
    }
    /// Водительское сиденье
    /// до
    let mut table78: String = "".to_owned();
    if eac.contains_key("51") {
        table78 = extract_with_sibling(document, &eac["51"]);
    }
    // после
    let mut table79: String = "".to_owned();
    if eac.contains_key("52") {
        table79 = extract_with_sibling(document, &eac["52"]);
    }
    // пассажирское
    // до
    let mut table81: String = "".to_owned();
    if eac.contains_key("53") {
        table81 = extract_with_sibling(document, &eac["53"]);
    }
    // после
    let mut table82: String = "".to_owned();
    if eac.contains_key("54") {
        table82 = extract_with_sibling(document, &eac["54"]);
    }
    // чрезвычайный случай
    let mut table83: String = "".to_owned();
    if eac.contains_key("55") {
        table83 = extract_with_sibling(document, &eac["55"]);
    }
    /// Шины
    let mut table84: String = "".to_owned();
    if eac.contains_key("6") {
        table84 = extract_with_sibling(document, &eac["6"]);
    }
    /// Водительское сиденье
    /// до
    let mut table85: String = "".to_owned();
    if eac.contains_key("61") {
        table85 = extract_with_sibling(document, &eac["61"]);
    }
    // после
    let mut table86: String = "".to_owned();
    if eac.contains_key("62") {
        table86 = extract_with_sibling(document, &eac["62"]);
    }
    // пассажирское
    // до
    let mut table87: String = "".to_owned();
    if eac.contains_key("63") {
        table87 = extract_with_sibling(document, &eac["63"]);
    }
    // после
    let mut table88: String = "".to_owned();
    if eac.contains_key("64") {
        table88 = extract_with_sibling(document, &eac["64"]);
    }
    // чрезвычайная ситуация
    let mut table89: String = "".to_owned();
    if eac.contains_key("65") {
        table89 = extract_with_sibling(document, &eac["65"]);
    }
    // Стекло
    let mut table90: String = "".to_owned();
    if eac.contains_key("7") {
        table90 = extract_with_sibling(document, &eac["7"]);
    }
    // Основные предметы
    // Статус удержания
    // нет есть
    let mut table91: String = "".to_owned();
    if eac.contains_key("8") {
        table91 = extract_with_sibling(document, &eac["8"]);
    }
    // Руководство по эксплуатации, джек
    let mut table92: String = "".to_owned();
    if eac.contains_key("83") {
        table92 = extract_with_sibling(document, &eac["83"]);
    }
    // Защитный штатив
    let mut table93: String = "".to_owned();
    if eac.contains_key("84") {
        table93 = extract_with_sibling(document, &eac["84"]);
    }

    // Особенности и мнения инспекторов
    let table94 = extract_value(
        document,
        "div.page_col2 > div.page_line > table.fuc_normal > tbody > :nth-child(1) > td",
    );
    let table95 = extract_value(
        document,
        "div.page_col2 > div.page_line > table.fuc_normal > tbody > :nth-child(2) > td",
    );
    // фотографии
    let table96 = extract_attrs(document, "src", "table.height_set3 > tbody > tr > td > img")?;
    println!(
        "
\n table31: {table31:?}
\n table32: {table32:?}
\n table33: {table33:?}
\n table34: {table34:?}
\n table35: {table35:?}
\n table36: {table36:?}
\n table37: {table37:?}
\n table38: {table38:?}
\n table39: {table39:?}
\n table40: {table40:?}
\n table41: {table41:?}
\n table42: {table42:?}
\n table43: {table43:?}
\n table44: {table44:?}
\n table45: {table45:?}
\n table46: {table46:?}
\n table47: {table47:?}
\n table48: {table48:?}
\n table49: {table49:?}
\n table50: {table50:?}
\n table51: {table51:?}
\n table52: {table52:?}
\n table53: {table53:?}
\n table54: {table54:?}
\n table55: {table55:?}
\n table56: {table56:?}
\n table57: {table57:?}
\n table58: {table58:?}
\n table59: {table59:?}
\n table60: {table60:?}
\n table61: {table61:?}
\n table62: {table62:?}
\n table63: {table63:?}
\n table64: {table64:?}
\n table65: {table65:?}
\n table66: {table66:?}
\n table67: {table67:?}
\n table68: {table68:?}
\n table69: {table69:?}
\n table70: {table70:?}
\n table71: {table71:?}
\n table72: {table72:?}
\n table73: {table73:?}
\n table74: {table74:?}
\n table75: {table75:?}
\n table76: {table76:?}
\n table77: {table77:?}
\n table78: {table78:?}
\n table79: {table79:?}
\n table81: {table81:?}
\n table82: {table82:?}
\n table83: {table83:?}
\n table84: {table84:?}
\n table85: {table85:?}
\n table86: {table86:?}
\n table87: {table87:?}
\n table88: {table88:?}
\n table89: {table89:?}
\n table90: {table90:?}
\n table91: {table91:?}
\n table92: {table92:?}
\n table93: {table93:?}
\n table94: {table94:?}
\n table95: {table95:?}
\n table96: {table96:?}
    "
    );
    Ok((CarSecList {}))
}
