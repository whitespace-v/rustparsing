use super::structs::CarSecList;
use crate::extractor::extract::{
    extract_attrs, extract_value, extract_values, extract_with_sibling, with_checked_label,
};
use scraper::Html;
use std::error::Error;

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    let title = extract_value(document, "span.num");
    // nazvanie
    let title = extract_value(document, "table > tbody > :nth-child(1) > :nth-child(2)");
    // grz
    let title = extract_value(document, "table > tbody > :nth-child(1) > :nth-child(4)");
    // year
    let title = extract_value(document, "table > tbody > :nth-child(2) > :nth-child(2)");
    // srok godnosti
    let title = extract_value(document, "table > tbody > :nth-child(2) > :nth-child(4)");
    // data pervoy registracii
    let title = extract_value(document, "table > tbody > :nth-child(3) > :nth-child(2)");
    // Типы коробок передач
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(4) > tbody > :nth-child(3) > :nth-child(4) > label",
    );
    // Номер ходовой части
    let title = extract_value(document, "table > tbody > :nth-child(4) > :nth-child(2)");
    // Использованное топливо
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(4) > tbody > :nth-child(5) > :nth-child(2) > label",
    );
    //Тип первичного двигателя
    let title = extract_value(document, "table > tbody > :nth-child(6) > :nth-child(2)");
    // tip garantii (если есть название -> гарантия компании вернет только название) если нет - самогарантия (не найдет)
    let title = extract_value(document, "div.ins_name");
    // table 2
    // Пробег и состояние прибора
    // Хороший Плохой
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(2) > :nth-child(2) > label",
    );
    // много обычно мало
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(3) > :nth-child(1) > label",
    );
    // probeg
    let title = extract_value(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(2) > :nth-child(3) > strong",
    );
    // Обозначение номера транспортного средства
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(4) > :nth-child(2) > label",
    );
    // vybros
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(5) > :nth-child(2) > label",
    );
    // vybros pokazateli
    let title = extract_value(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(5) > :nth-child(3)",
    );
    //tuning
    // est net
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(6) > :nth-child(2) > label",
    );
    // zakonno / net
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(6) > :nth-child(3) > label",
    );
    // struktura / oborudovanie
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(6) > :nth-child(4) > label",
    );
    // Особая история
    // est net
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(7) > :nth-child(2) > label",
    );
    // ogon navodnenie
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(7) > :nth-child(3) > label",
    );
    // Изменение способа использования
    //est net
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(8) > :nth-child(2) > label",
    );
    // prodaja arenda
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(8) > :nth-child(3) > label",
    );
    // Цвет
    // hrom ahrom
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(9) > :nth-child(2) > label",
    );
    // Полноцветный / Изменение цвета
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(9) > :nth-child(3) > label",
    );
    // Основные опции
    // da net
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(10) > :nth-child(2) > label",
    );
    // navigacia / luk / drugoe
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(10) > :nth-child(3) > label",
    );
    // Подлежит отзыву
    // Непригодный / Применимый
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(11) > :nth-child(2) > label",
    );
    // Отозвать неисполнение / Отозвать реализацию
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(5) > tbody > :nth-child(11) > :nth-child(3) > label",
    );

    // table 3
    // prostoy remont da / net
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(6) > tbody > :nth-child(3) > :nth-child(4) > label",
    );
    //////////// out
    ////////////// hashmap {name, gif}, delete where gif = n.gif, save where x.gif ->
    //// в будущем преобразовать все подобные выводы из секлиста в единую систему
    // 1
    // names
    let title1 = extract_values(
        document,
        "body > div[id=wrap] > :nth-child(6) > tbody > :nth-child(5) > :nth-child(3) > label",
    );
    // srcs
    let title1 = extract_attrs(
        document,
        "src",
        "body > div[id=wrap] > :nth-child(6) > tbody > :nth-child(5) > :nth-child(3) > label > img",
    )?;
    // 2
    // names
    let title2 = extract_values(
        document,
        "body > div[id=wrap] > :nth-child(6) > tbody > :nth-child(6) > :nth-child(2) > label",
    );
    // srcs
    let title2 = extract_attrs(
        document,
        "src",
        "body > div[id=wrap] > :nth-child(6) > tbody > :nth-child(6) > :nth-child(2) > label > img",
    )?;
    //////////// bones
    // a
    // names
    let titlea = extract_values(
        document,
        "body > div[id=wrap] > :nth-child(6) > tbody > :nth-child(7) > :nth-child(3) > label",
    );
    // srcs
    let titlea = extract_attrs(
        document,
        "src",
        "body > div[id=wrap] > :nth-child(6) > tbody > :nth-child(7) > :nth-child(3) > label > img",
    )?;
    // b
    // names
    let titleb = extract_values(
        document,
        "body > div[id=wrap] > :nth-child(6) > tbody > :nth-child(8) > :nth-child(2) > label",
    );
    // srcs
    let titleb = extract_attrs(
        document,
        "src",
        "body > div[id=wrap] > :nth-child(6) > tbody > :nth-child(8) > :nth-child(2) > label > img",
    )?;
    // c
    // names
    let titlec = extract_values(
        document,
        "body > div[id=wrap] > :nth-child(6) > tbody > :nth-child(9) > :nth-child(2) > label",
    );
    // srcs
    let titlec = extract_attrs(
        document,
        "src",
        "body > div[id=wrap] > :nth-child(6) > tbody > :nth-child(9) > :nth-child(2) > label > img",
    )?;
    // table 4
    /////////////////// Самодиагностика
    // Первичный двигатель
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(2) > td > label",
    );
    // Коробка передач
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(3) > td > label",
    );
    /////////////////// Первичный двигатель
    // Рабочее состояние (холостой ход)
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(4) > td > label",
    );
    ////// Утечка масла
    // Крышка цилиндра (крышка коромысла)
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(5) > td > label",
    );
    // Головка блока цилиндров / прокладка
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(6) > td > label",
    );
    // Блок цилиндров / Масляный поддон Производитель Китай
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(7) > td > label",
    );
    //
    // Расход масла
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(8) > td > label",
    );
    //////// Охлаждающая жидкость Утечки
    // Головка блока цилиндров/прокладка
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(9) > td > label",
    );
    // водяной насос
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(10) > td > label",
    );
    // Радиатор
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(11) > td > label",
    );
    // Количество охлаждающей жидкости
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(12) > td > label",
    );
    //
    // Общая магистраль
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(13) > td > label",
    );
    /////////////////// Коробка передач
    /////// Автоматическая коробка передач (A/T)
    // Утечка масла Масло
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(14) > td > label",
    );
    // Расход и состояние масла
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(15) > td > label",
    );
    // Рабочее состояние (холостой ход)
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(16) > td > label",
    );
    /////// Механическая коробка передач (M/T)
    // Утечка масла Масло
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(17) > td > label",
    );
    // Переключение передач
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(18) > td > label",
    );
    // Расход и состояние масла
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(19) > td > label",
    );
    // Рабочее состояние (холостой ход)
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(20) > td > label",
    );
    /////////////////// Передача электроэнергии
    // Сцепление в сборе
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(21) > td > label",
    );
    // Шарнир с постоянной скоростью вращения
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(22) > td > label",
    );
    // Приводной вал и подшипник
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(23) > td > label",
    );
    // Дифференциальная передача
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(24) > td > label",
    );
    /////////////////// Рулевое управление
    // Утечка масла при работе гидроусилителя рулевого управления
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(25) > td > label",
    );
    ////////// Рабочее состояние
    // Рулевой механизм с MDPS
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(26) > td > label",
    );
    // Насос рулевого управления
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(27) > td > label",
    );
    // Шарнир рулевого управления
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(28) > td > label",
    );
    // Силовой шланг высокого давления
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(29) > td > label",
    );
    // Наконечник рулевой тяги и шаровой шарнир
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(30) > td > label",
    );
    /////////////////// Тормозной
    // Утечка масла из Главного тормозного цилиндра
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(31) > td > label",
    );
    // Утечка тормозного масла
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(32) > td > label",
    );
    // Состояние источника питания
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(33) > td > label",
    );
    /////////////////// Электричество
    // Выход генератора
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(34) > td > label",
    );
    // Пусковой двигатель
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(35) > td > label",
    );
    // Функция двигателя стеклоочистителя
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(36) > td > label",
    );
    // Двигатель для вентиляции помещений
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(37) > td > label",
    );
    // Двигатель вентилятора радиатора
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(38) > td > label",
    );
    // Привод стеклоподъемника
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(39) > td > label",
    );
    /////////////////// Мощное электрическое устройство
    // Состояние изоляции зарядного порта
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(40) > td > label",
    );
    // Состояние изоляции аккумуляторной батареи привода
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(41) > td > label",
    );
    // Состояние электропроводки высокой мощности (соединительная клемма, ткань, защитный механизм)
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(42) > td > label",
    );
    /////////////////// Топливо
    // Утечка топлива (включая сжиженный газ)
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(8) > tbody > :nth-child(43) > td > label",
    );

    // table 5
    ///////////// Требуется ремонт
    // Внешний вид
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(9) > tbody > :nth-child(2) > :nth-child(3) > label",
    );
    // Встроенный
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(9) > tbody > :nth-child(2) > :nth-child(5) > label",
    );
    // Блеск
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(9) > tbody > :nth-child(3) > :nth-child(2) > label",
    );
    // Уборка помещений
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(9) > tbody > :nth-child(3) > :nth-child(4) > label",
    );
    // Колесо
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(9) > tbody > :nth-child(4) > :nth-child(2) > label",
    );
    // Шины
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(9) > tbody > :nth-child(5) > :nth-child(2) > label",
    );
    // Стекло
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(9) > tbody > :nth-child(6) > :nth-child(2) > label",
    );
    /////////// Основные предметы
    // Статус удержания
    let title = with_checked_label(
        document,
        "body > div[id=wrap] > :nth-child(9) > tbody > :nth-child(7) > :nth-child(3) > label",
    );
    // table 6
    // Особенности и мнения инспекторов
    // Инспектор по производительности труда и охране здоровья
    let title = extract_value(
        document,
        "body > div[id=wrap] > :nth-child(10) > tbody > :nth-child(1) > :nth-child(3)",
    );
    //  Цена / Обзор
    let title = extract_value(
        document,
        "body > div[id=wrap] > :nth-child(10) > tbody > :nth-child(2) > :nth-child(2)",
    );
    Ok(())
}
