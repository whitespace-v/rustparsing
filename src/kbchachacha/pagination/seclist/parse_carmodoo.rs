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
    let table2_4_2 = extract_with_sibling(document, &bc["31"]);
    //// устройство / структура
    let table2_4_3 = extract_with_sibling(document, &bc["32"]);
    // особая история
    //// есть нет
    let table2_5_1 = extract_with_sibling(document, &bc["4"]);
    //// огонь пожар
    let table2_5_2 = extract_with_sibling(document, &bc["41"]);
    // Изменение способа использования
    //// есть нет
    let table2_6_1 = extract_with_sibling(document, &bc["5"]);
    //// продажа аренда
    let table2_6_2 = extract_with_sibling(document, &bc["51"]);
    // цвет
    ///// хром ахром
    let table2_7_1 = extract_with_sibling(document, &bc["61"]);
    ///// полноцветный / изменение цвета
    let table2_7_2 = extract_with_sibling(document, &bc["62"]);
    // основные опции
    //// есть нет
    let table2_8_1 = extract_with_sibling(document, &bc["7"]);
    //// люк на крыше навигация другое
    let table2_8_2 = extract_with_sibling(document, &bc["71"]);
    // подлежит отзыву
    //// применимо неприминимо
    let table2_9_1 = extract_with_sibling(document, &bc["81"]);
    //// выполнение/ неработающий
    let table2_9_2 = extract_with_sibling(document, &bc["82"]);
    println!(
        "
    \nprobeg {table2_1:?} 
    \nprobeg {table2_1_1:?} 
    \nprobeg.km: {table2_1_2:?}
    \nobozna {table2_2}
    \nvibrosy {table2_3:?} -> {table2_3_1:?}
    \ntuning {table2_4_1} 
    \nspec.exp {table2_5_1}
    table2_4_2
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
    let table31 = extract_with_sibling(document, &dc["11"]);
    // Коробка передач
    let table32 = extract_with_sibling(document, &dc["12"]);
    // Рабочее состояние (холостой ход)
    let table33 = extract_with_sibling(document, &dc["21"]);
    // Утечка масла
    //// Крышка цилиндра (крышка коромысла)
    let table34 = extract_with_sibling(document, &dc["221"]);
    //// Головка блока цилиндров / прокладка
    let table35 = extract_with_sibling(document, &dc["222"]);
    //// Блок цилиндров / Масляный поддон Производитель Китай
    let table36 = extract_with_sibling(document, &dc["223"]);
    //// Расход масла
    let table37 = extract_with_sibling(document, &dc["23"]);
    //// Охлаждающая жидкость Утечки
    //// Головка блока цилиндров / прокладка
    let table38 = extract_with_sibling(document, &dc["231"]);
    //// водяной насос
    let table39 = extract_with_sibling(document, &dc["232"]);
    //// Радиатор
    let table40 = extract_with_sibling(document, &dc["233"]);
    //// Количество охлаждающей жидкости
    let table41 = extract_with_sibling(document, &dc["234"]);
    //Общая магистраль
    let table42 = extract_with_sibling(document, &dc["24"]);
    //////// Коробка передач
    //// АКПП
    /// Утечка масла Масло
    let table43 = extract_with_sibling(document, &dc["311"]);
    /// Расход и состояние масла
    let table44 = extract_with_sibling(document, &dc["312"]);
    /// Рабочее состояние (холостой ход)
    let table45 = extract_with_sibling(document, &dc["313"]);
    //// МКПП
    /// Утечка масла Масло
    let table46 = extract_with_sibling(document, &dc["321"]);
    /// Переключение передач
    let table47 = extract_with_sibling(document, &dc["322"]);
    /// Расход и состояние масла
    let table48 = extract_with_sibling(document, &dc["323"]);
    /// Рабочее состояние (холостой ход)
    let table49 = extract_with_sibling(document, &dc["324"]);
    /////Передача электроэнергии
    //Сцепление в сборе
    let table50 = extract_with_sibling(document, &dc["41"]);
    // Соединение с постоянной скоростью
    let table51 = extract_with_sibling(document, &dc["42"]);
    // Приводной вал и подшипник
    let table52 = extract_with_sibling(document, &dc["43"]);
    // Дифференциальная передача
    let table53 = extract_with_sibling(document, &dc["44"]);
    ///////////////Рулевое управление
    /// Утечка масла при работе гидроусилителя рулевого управления
    let table54 = extract_with_sibling(document, &dc["51"]);
    /// Рабочее состояние
    /// Насос рулевого управления
    let table55 = extract_with_sibling(document, &dc["522"]);
    /// Рулевой механизм с MDPS
    let table56 = extract_with_sibling(document, &dc["521"]);
    /// Шарнир рулевого управления
    let table57 = extract_with_sibling(document, &dc["524"]);
    /// Силовой шланг высокого давления
    let table58 = extract_with_sibling(document, &dc["525"]);
    /// Наконечник рулевой тяги и шаровой шарнир
    let table59 = extract_with_sibling(document, &dc["523"]);
    /////////////////// Тормозная система
    /// Утечка масла из Главного тормозного цилиндра
    let table60 = extract_with_sibling(document, &dc["61"]);
    /// Утечка тормозного масла
    let table61 = extract_with_sibling(document, &dc["62"]);
    /// Состояние источника питания
    let table62 = extract_with_sibling(document, &dc["63"]);
    ////////////// Электричество
    /// Выход генератора
    let table63 = extract_with_sibling(document, &dc["71"]);
    /// Пусковой двигатель
    let table64 = extract_with_sibling(document, &dc["72"]);
    /// Функция двигателя стеклоочистителя
    let table65 = extract_with_sibling(document, &dc["73"]);
    /// Двигатель для вентиляции помещений
    let table66 = extract_with_sibling(document, &dc["74"]);
    /// Двигатель вентилятора радиатора
    let table67 = extract_with_sibling(document, &dc["75"]);
    /// Привод стеклоподъемника
    let table68 = extract_with_sibling(document, &dc["76"]);
    ////////////////////////// Классические источники Электрическое устройство
    /// Состояние изоляции зарядного порта
    let table69 = extract_with_sibling(document, &dc["91"]);
    /// Состояние изоляции аккумуляторной батареи привода
    let table70 = extract_with_sibling(document, &dc["92"]);
    /// Состояние электропроводки высокой мощности (соединительная клемма, ткань, защитный механизм)
    let table71 = extract_with_sibling(document, &dc["93"]);
    /////////// Топливо
    /// Утечка топлива (включая сжиженный газ)
    let table72 = extract_with_sibling(document, &dc["81"]);

    ////////////table 4
    ///////   Требуется ремонт
    /// Внешний вид
    let table73 = extract_with_sibling(document, &eac["1"]);
    /// Встроенный
    let table74 = extract_with_sibling(document, &eac["2"]);
    /// Блеск
    let table75 = extract_with_sibling(document, &eac["3"]);
    /// Уборка помещений
    let table76 = extract_with_sibling(document, &eac["4"]);
    /// Колесо
    let table77 = extract_with_sibling(document, &eac["5"]);
    /// Водительское сиденье
    /// до
    let table78 = extract_with_sibling(document, &eac["51"]);
    // после
    let table79 = extract_with_sibling(document, &eac["52"]);
    // пассажирское
    // до
    let table81 = extract_with_sibling(document, &eac["53"]);
    // после
    let table82 = extract_with_sibling(document, &eac["54"]);
    // чрезвычайный случай
    let table83 = extract_with_sibling(document, &eac["55"]);
    /// Шины
    let table84 = extract_with_sibling(document, &eac["6"]);
    /// Водительское сиденье
    /// до
    let table85 = extract_with_sibling(document, &eac["61"]);
    // после
    let table86 = extract_with_sibling(document, &eac["62"]);
    // пассажирское
    // до
    let table87 = extract_with_sibling(document, &eac["63"]);
    // после
    let table88 = extract_with_sibling(document, &eac["64"]);
    // чрезвычайная ситуация
    let table89 = extract_with_sibling(document, &eac["65"]);
    /// Стекло
    let table90 = extract_with_sibling(document, &eac["7"]);
    /// Основные предметы
    ///  Статус удержания
    /// нет есть
    let table91 = extract_with_sibling(document, &eac["8"]);
    // Руководство по эксплуатации, джек
    let table92 = extract_with_sibling(document, &eac["83"]);
    // Защитный штатив
    let table93 = extract_with_sibling(document, &eac["84"]);

    // Особенности и мнения инспекторов
    let table94 = extract_value(
        document,
        "div.page_col2 > div.page_line > table.fuc_normal > tbody > : nth-child(1) > td",
    );
    let table95 = extract_value(
        document,
        "div.page_col2 > div.page_line > table.fuc_normal > tbody > : nth-child(2) > td",
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
    Ok(())
}
