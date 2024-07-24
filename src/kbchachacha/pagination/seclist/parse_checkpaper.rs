use crate::extractor::extract::{
    extract_attr, extract_attrs, extract_value, extract_values, with_checked,
};
use scraper::Html;
use std::error::Error;

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    let title = extract_value(document, "div.docu_title");
    // 1. название; 2. ГРЗ; 3. год выпуска; 4. срок действия тех.отчета;
    // 5. дата первой регистрации 7.идентификационный номер авто; 10. мотор
    let table1 = extract_values(document, "table.ins_tbl1 > tbody > tr > td");
    // 7. КПП
    let table1_1 = with_checked(
        document,
        "table.ins_tbl1 > tbody > tr:nth-child(3) > td > ul.chkbox_list > li",
    );
    // топливо
    let table1_2 = with_checked(
        document,
        "table.ins_tbl1 > tbody > tr:nth-child(5) > td > ul.chkbox_list > li",
    );
    // тип гарантии
    let table1_3 = with_checked(
        document,
        "table.ins_tbl1 > tbody > tr:nth-child(6) > td > ul.chkbox_list > li",
    );
    println!("\n[Seclist]:\ntitle:{title}\n[table1]:\n{table1:#?}\nTransmission: {table1_1:?}\ngas: {table1_2:?},\nWarranty: {table1_3:?}");
    // состояние Пробега
    let table2_1 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(3) > td > ul.chkbox_list > li",
    );
    // общее состояние авто
    let table2_2 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(4) > td > ul.chkbox_list > li ",
    );
    // пробег в км
    let table2_3 = extract_value(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(2) > :nth-child(3)",
    );
    // состояние шильдика с вином
    let table2_4 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(4) > td > ul.chkbox_list > li",
    );
    // выбросы
    let table2_5 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(5) > td > ul.chkbox_list > li",
    );
    // показатели выбросов
    let table2_5_1 = extract_values(document, "td.exhaust_gas");

    // тюнинг/модификации
    let table2_6 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(6) > :nth-child(2) > ul.chkbox_list > li",
    );
    // законность тюнинга если есть
    let table2_6_1 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(6) > :nth-child(3) > ul.chkbox_list > li",
    );
    // хуй знает что в тюнинге если есть
    let table2_6_2 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(6) > :nth-child(4) > ul.chkbox_list > li",
    );
    // особая история
    let table2_7 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(7) > :nth-child(2) > ul.chkbox_list > li",
    );
    // наводнение/огонь
    let table2_7_1 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(7) > :nth-child(3) > ul.chkbox_list > li",
    );
    // Изменение способа использования ??
    let table2_8 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(8) > :nth-child(2) > ul.chkbox_list > li",
    );
    // аренда/ продажа
    let table2_8_1 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(8) > :nth-child(3) > ul.chkbox_list > li",
    );
    // Цвет (а)/хроматический
    let table2_9 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(9) > :nth-child(2) > ul.chkbox_list > li",
    );
    // полная покраска // изменение цвета
    let table2_9_1 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(9) > :nth-child(3) > ul.chkbox_list > li",
    );
    // Основные опции
    let table2_10 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(10) > :nth-child(2) > ul.chkbox_list > li",
    );
    // люк на крыше/навигация/ другое
    let table2_10_1 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(10) > :nth-child(3) > ul.chkbox_list > li",
    );
    // подлежит отзыву: неприменимо/ применимо
    let table2_11 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(11) > :nth-child(2) > ul.chkbox_list > li",
    );
    // реализация отзыва: было/ не было
    let table2_11_1 = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(12) > :nth-child(4) > ul.chkbox_list > li",
    );
    println!("\nprobeg:{table2_1:?}\nsostoyanie: {table2_2:?}\nprobeg: {table2_3:?}\nshield: {table2_4:?}\nvibrosy: {table2_5:?} {table2_5_1:?}
            \ntuning: {table2_6:?} {table2_6_1:?} {table2_6_2:?}\nadvanced exp: {table2_7:?} {table2_7_1:?}\ntable2_8: {table2_8:?}\ntable2_8_1: {table2_8_1:?}\ntable2_9: {table2_9:?}\ntable2_9_1: {table2_9_1:?}\ntable2_10: {table2_10:?}\ntable2_10_1: {table2_10_1:?}\ntable2_11: {table2_11:?}\ntable2_11_1: {table2_11_1:?}
            ");

    //image
    let table3_1 = extract_attr(
        document,
        "src",
        "table.ins_tbl3 > tbody > :nth-child(2) > td > div > p > img",
    );
    // история несчатный случаев
    let table3_2 = with_checked(
        document,
        "table.ins_tbl3 > tbody > tr:nth-child(3) > :nth-child(2) > ul.chkbox_list > li",
    );
    // простой ремонт
    let table3_2_1 = with_checked(
        document,
        "table.ins_tbl3 > tbody > tr:nth-child(3) > :nth-child(4) > ul.chkbox_list > li",
    );
    // внешняя часть
    //// 1 ранг
    let table3_3 = with_checked(
        document,
        "table.ins_tbl3 > tbody > tr:nth-child(5) > td > ul.chkbox_list > li",
    );
    //// 2 ранг
    let table3_3_1 = with_checked(
        document,
        "table.ins_tbl3 > tbody > tr:nth-child(6) > td > ul.chkbox_list > li",
    );
    // основной скелет
    //// A ранг
    let table3_3_2 = with_checked(
        document,
        "table.ins_tbl3 > tbody > tr:nth-child(7) > td > ul.chkbox_list > li",
    );
    //// B ранг
    let table3_3_3 = with_checked(
        document,
        "table.ins_tbl3 > tbody > tr:nth-child(8) > td > ul.chkbox_list > li",
    );
    //// C ранг
    let table3_3_4 = with_checked(
        document,
        "table.ins_tbl3 > tbody > tr:nth-child(9) > td > ul.chkbox_list > li",
    );
    println!(
                "\n[Table3]\nimg: {table3_1}, \ntable3_2:{table3_2:?}\ntable3_2_1:{table3_2_1:?}\ntable3_3: {table3_3:?}\ntable3_3_1: {table3_3_1:?}\ntable3_3_2: {table3_3_2:?}\ntable3_3_3: {table3_3_3:?}\ntable3_3_4: {table3_3_4:?}"
            );
    //table 4
    // самодиагностика
    //// мотор
    let table4_1 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(2) > td > ul.chkbox_list > li",
    );
    //// кпп
    let table4_2 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(3) > td > ul.chkbox_list > li",
    );
    // мотор
    //// рабочее состояние (холостой ход)
    let table4_3 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(4) > td > ul.chkbox_list > li",
    );
    // масло
    //// Крышка цилиндра (крышка коромысла)
    let table4_4 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(5) > td > ul.chkbox_list > li",
    );
    ///// Головка блока цилиндров / прокладка
    let table4_5 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(6) > td > ul.chkbox_list > li",
    );
    ///// Блок цилиндров / Масляный поддон Производитель Китай
    let table4_6 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(7) > td > ul.chkbox_list > li",
    );
    /////  Расход масла
    let table4_7 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(8) > td > ul.chkbox_list > li",
    );
    // Утечка охлаждающей жидкости
    ///// Головка блока цилиндров / прокладка
    let table4_8 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(9) > td > ul.chkbox_list > li",
    );
    ///// водяной насос
    let table4_9 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(10) > td > ul.chkbox_list > li",
    );
    ///// Радиатор
    let table4_10 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(11) > td > ul.chkbox_list > li",
    );
    ///// Количество охлаждающей жидкости
    let table4_11 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(12) > td > ul.chkbox_list > li",
    );
    ///// Общая магистраль
    let table4_12 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(13) > td > ul.chkbox_list > li",
    );
    // КПП
    ////// АКПП
    /// Утечка масла Масло
    let table4_13 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(14) > td > ul.chkbox_list > li",
    );
    /// Расход и состояние масла
    let table4_14 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(15) > td > ul.chkbox_list > li",
    );
    /// Рабочее состояние (холостой ход)
    let table4_15 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(16) > td > ul.chkbox_list > li",
    );
    ////// МКПП
    /// Утечка масла Масло
    let table4_16 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(17) > td > ul.chkbox_list > li",
    );
    ///  Переключение передач
    let table4_17 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(18) > td > ul.chkbox_list > li",
    );
    ///Расход и состояние масла
    let table4_18 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(19) > td > ul.chkbox_list > li",
    );
    /// Рабочее состояние (холостой ход)
    let table4_19 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(20) > td > ul.chkbox_list > li",
    );
    /////// Передача электроэнергии
    //Сцепление в сборе
    let table4_20 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(21) > td > ul.chkbox_list > li",
    );
    //Шарнир с постоянной скоростью вращения
    let table4_21 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(22) > td > ul.chkbox_list > li",
    );
    //Приводной вал и подшипник
    let table4_22 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(23) > td > ul.chkbox_list > li",
    );
    //Дифференциальная передача
    let table4_23 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(24) > td > ul.chkbox_list > li",
    );
    /////// Рулевое управление
    // Утечка масла при работе гидроусилителя рулевого управления
    let table4_24 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(25) > td > ul.chkbox_list > li",
    );
    //// Рабочее состояние
    //Насос рулевого управления
    let table4_25 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(26) > td > ul.chkbox_list > li",
    );
    //Рулевой механизм с MDPS
    let table4_26 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(27) > td > ul.chkbox_list > li",
    );
    //Шарнир рулевого управления
    let table4_27 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(28) > td > ul.chkbox_list > li",
    );
    // Силовой шланг высокого давления
    let table4_28 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(29) > td > ul.chkbox_list > li",
    );
    // Наконечник рулевой тяги и шаровой шарнир
    let table4_29 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(30) > td > ul.chkbox_list > li",
    );
    /////// тормозная система
    /// Утечка масла из Главного тормозного цилиндра
    let table4_30 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(31) > td > ul.chkbox_list > li",
    );
    /// Утечка тормозного масла
    let table4_31 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(32) > td > ul.chkbox_list > li",
    );
    ///  Состояние источника питания
    let table4_32 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(33) > td > ul.chkbox_list > li",
    );
    ////////// Электричество
    /// Выход генератора
    let table4_33 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(34) > td > ul.chkbox_list > li",
    );
    /// Пусковой двигатель
    let table4_34 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(35) > td > ul.chkbox_list > li",
    );
    /// Функция двигателя стеклоочистителя
    let table4_35 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(36) > td > ul.chkbox_list > li",
    );
    /// Двигатель для вентиляции помещений
    let table4_36 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(37) > td > ul.chkbox_list > li",
    );
    /// Двигатель вентилятора радиатора
    let table4_37 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(38) > td > ul.chkbox_list > li",
    );
    /// Привод стеклоподъемника
    let table4_38 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(39) > td > ul.chkbox_list > li",
    );
    ////////////////Классические источники Электрическое устройство
    // Состояние изоляции зарядного порта
    let table4_39 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(40) > td > ul.chkbox_list > li",
    );
    // Состояние изоляции аккумуляторной батареи привода
    let table4_40 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(41) > td > ul.chkbox_list > li",
    );
    //Состояние электропроводки высокой мощности (Соединительная клемма, ткань, защитный механизм)
    let table4_41 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(42) > td > ul.chkbox_list > li",
    );
    ///////////// Топливная система
    // Утечка топлива (включая сжиженный газ)
    let table4_42 = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(43) > td > ul.chkbox_list > li",
    );
    println!(
        "\n[Table4]:
table4_1: {table4_1:?}
table4_2: {table4_2:?}
table4_3: {table4_3:?}
table4_4: {table4_4:?}
table4_5: {table4_5:?}
table4_6: {table4_6:?}
table4_7: {table4_7:?}
table4_8: {table4_8:?}
table4_9: {table4_9:?}
table4_10: {table4_10:?}
table4_11: {table4_11:?}
table4_12: {table4_12:?}
table4_13: {table4_13:?}
table4_14: {table4_14:?}
table4_15: {table4_15:?}
table4_16: {table4_16:?}
table4_17: {table4_17:?}
table4_18: {table4_18:?}
table4_19: {table4_19:?}
table4_20: {table4_20:?}
table4_21: {table4_21:?}
table4_22: {table4_22:?}
table4_23: {table4_23:?}
table4_24: {table4_24:?}
table4_25: {table4_25:?}
table4_26: {table4_26:?}
table4_27: {table4_27:?}
table4_28: {table4_28:?}
table4_29: {table4_29:?}
table4_30: {table4_30:?}
table4_31: {table4_31:?}
table4_32: {table4_32:?}
table4_33: {table4_33:?}
table4_34: {table4_34:?}
table4_35: {table4_35:?}
table4_36: {table4_36:?}
table4_37: {table4_37:?}
table4_38: {table4_38:?}
table4_39: {table4_39:?}
table4_40: {table4_40:?}
table4_41: {table4_41:?}
table4_42: {table4_42:?}
            "
    );

    //// требуется ремонт
    // снаружи
    let table5_1 = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(2) > :nth-child(3) > ul.chkbox_list > li",
    );
    // внутри
    let table5_2 = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(2) > :nth-child(5) > ul.chkbox_list > li",
    );
    // блеск
    let table5_3 = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(3) > :nth-child(2) > ul.chkbox_list > li",
    );
    // требуется Уборка
    let table5_4 = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(3) > :nth-child(4) > ul.chkbox_list > li",
    );
    /////// Колеса
    let table5_5 = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(4) > :nth-child(2) > ul.chkbox_list > li",
    );
    // какое именно колесо либо критическая ситуация
    let table5_6 = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(4) > :nth-child(3) > ul.chkbox_list > li",
    );
    ///////// шины
    let table5_7 = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(5) > :nth-child(2) > ul.chkbox_list > li",
    );
    // какое именно колесо либо критическая ситуация
    let table5_8 = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(5) > :nth-child(3) > ul.chkbox_list > li",
    );
    // стекло
    let table5_9 = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(6) > td > ul.chkbox_list > li",
    );
    // основное
    // статус? какая-то хуйня нужно разобраться
    let table5_10 = with_checked(
                document,
                "table.ins_tbl5 > tbody > tr:nth-child(7) > :nth-child(3) > ul.chkbox_list:nth-child(1) > li",
            );
    let table5_11 = with_checked(
                document,
                "table.ins_tbl5 > tbody > tr:nth-child(7) > :nth-child(3) > ul.chkbox_list:nth-child(2) > li",
            );
    println!(
        "
\ntable5_1: {table5_1:?}
\ntable5_2: {table5_2:?}
\ntable5_3: {table5_3:?}
\ntable5_4: {table5_4:?}
\ntable5_5: {table5_5:?}
\ntable5_6: {table5_6:?}
\ntable5_7: {table5_7:?}
\ntable5_8: {table5_8:?}
\ntable5_9: {table5_9:?}
\ntable5_10: {table5_10:?}
\ntable5_11: {table5_11:?}
            "
    );
    // ////// Особенности и мнения инспекторов
    // Производительность/ Комментарий Контролера
    let table5_12 = extract_value(document, "table.ins_tbl5 > tbody > tr:nth-child(9) > td");
    //Цена/ Опрос Калькулятор
    let table5_13 = extract_value(document, "table.ins_tbl5 > tbody > tr:nth-child(10) > td");

    println!("{table5_12} {table5_13}");
    // фотографии
    let table7_1 = extract_attrs(
        document,
        "src",
        "table.ins_tbl7 > tbody > tr > td.car_photo > ul > li > img",
    )?;
    // дата проведения техосмотра
    let table8_1 = extract_values(document, "table.ins_tbl8 > tbody > tr > td p.date");

    println!("images: {}\n{table8_1:?}", table7_1.len());
    Ok(())
}
