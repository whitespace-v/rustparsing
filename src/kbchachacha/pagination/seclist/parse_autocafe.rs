use crate::extractor::extract::{
    extract_attrs, extract_near_text_with, extract_value, extract_values, extract_with_sibling,
};
use scraper::Html;
use std::error::Error;

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    // name
    // http://www.autocafe.co.kr/asso/Check_Form_2020.asp?ChkSeq=1704389

    ////// table 1
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(1) > :nth-child(2)",
    );
    // extended name
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(1) > :nth-child(3)",
    );
    // grz
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(1) > :nth-child(5)",
    );
    // god
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(2) > :nth-child(2)",
    );
    // (4) Срок действия проверки
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(2) > :nth-child(4)",
    );
    // (5) Дата первоначальной регистрации
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(3) > :nth-child(2)",
    );
    // (6) Номер транспортного средства
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(4) > :nth-child(2)",
    );
    // engine
    let title = extract_value(
        document,
        "div.main-listbox > table > tbody > :nth-child(6) > :nth-child(2)",
    );
    // (7) Коробка передач
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(3) > tbody > :nth-child(3) > :nth-child(4)",
        "src",
        "img/check_on.gif",
    );
    // (8) Использованное топливо
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(3) > tbody > :nth-child(5) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // (10) Тип гарантии
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(3) > tbody > :nth-child(6) > :nth-child(4)",
        "src",
        "img/check_on.gif",
    );

    //// table 2
    // probeg
    let title = extract_value(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(2) > :nth-child(3)",
    );
    //Пробег и
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(2) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Состояние прибора
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(3) > :nth-child(1)",
        "src",
        "img/check_on.gif",
    );
    // Обозначение номера транспортного средства
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(4) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // vibrosi
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(5) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    let title = extract_value(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(5) > :nth-child(3)",
    );
    // tuning
    // da net
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(6) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // zakonno / net
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(6) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Структура / ustroystvo
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(6) > :nth-child(4)",
        "src",
        "img/check_on.gif",
    );
    //Особая история
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(7) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // navodnenie / fire
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(7) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Изменение способа использования
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(8) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // arenda / prodaja
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(8) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Цвет
    // chrome / achrome
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(9) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // polnocvet / Изменение цвета
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(9) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Основные опции
    // da net
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(10) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Люк в крыше / Навигация / drugoe
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(10) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Подлежит отзыву
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(11) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Реализация отзыва
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(5) > tbody > :nth-child(11) > :nth-child(4)",
        "src",
        "img/check_on.gif",
    );

    // table 3
    // (12) История несчастных случаев
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(7) > tbody > :nth-child(3) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Простой ремонт
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(7) > tbody > :nth-child(3) > :nth-child(4)",
        "src",
        "img/check_on.gif",
    );
    ////////// out
    // 1
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(7) > tbody > :nth-child(5) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // 2
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(7) > tbody > :nth-child(6) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    ///////// bones
    // a
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(7) > tbody > :nth-child(7) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // b
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(7) > tbody > :nth-child(8) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // c
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(7) > tbody > :nth-child(9) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );

    // table 4
    ////////////////////// Самодиагностика
    // Первичный двигатель
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(2) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Коробка передач
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(3) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    ////////////////////// Первичный двигатель
    // Рабочее состояние (холостой ход)
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(4) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    /////// Утечка масла Масло
    // Крышка цилиндра (крышка коромысла)
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(5) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Головка блока цилиндров / прокладка
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(6) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Блок цилиндров / Масляный поддон Производитель Китай
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(7) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    //
    // Расход масла
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(8) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    /////// Утечка охлаждающей жидкости
    // Головка блока цилиндров / прокладка
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(9) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // водяной насос
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(10) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Радиатор
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(11) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Количество охлаждающей жидкости
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(12) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    //
    // Общая магистраль
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(13) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    ////////////////////// Коробка передач
    /////// avtomat
    // Утечка масла Масло
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(14) > :nth-child(4)",
        "src",
        "img/check_on.gif",
    );
    // Расход и состояние масла
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(15) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Рабочее состояние (холостой ход)
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(16) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    /////// mechanica
    // Утечка масла Масло
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(17) > :nth-child(4)",
        "src",
        "img/check_on.gif",
    );
    // Переключение передач
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(18) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Расход и состояние масла
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(19) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Рабочее состояние (холостой ход)
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(20) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    ////////////////////// Передача электроэнергии
    // Сцепление в сборе
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(21) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Шарнир с постоянной скоростью вращения
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(22) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Приводной вал и подшипник
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(23) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Дифференциальная передача
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(24) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    ////////////////////// Рулевое управление
    // Утечка масла при работе гидроусилителя рулевого управления
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(25) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    /////////  Рабочее состояние
    // Насос рулевого управления
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(26) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Рулевой механизм с MDPS
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(27) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Шарнир рулевого управления
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(28) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Силовой шланг высокого давления
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(29) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Наконечник рулевой тяги и шаровой шарнир
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(30) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    ////////////////////// Тормозной
    // Утечка масла из Главного тормозного цилиндра
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(31) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Утечка тормозного масла
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(32) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Состояние источника питания
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(33) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    ////////////////////// Электричество
    // Выход генератора
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(34) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Пусковой двигатель
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(35) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Функция двигателя стеклоочистителя
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(36) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Двигатель для вентиляции помещений
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(37) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Двигатель вентилятора радиатора
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(38) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Привод стеклоподъемника
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(39) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    ////////////////////// Классические источники Электрическое устройство
    // Состояние изоляции зарядного порта
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(40) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Состояние изоляции аккумуляторной батареи привода
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(41) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Состояние электропроводки высокой мощности (Соединительная клемма, ткань, защитный механизм)
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(42) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    ////////////////////// Топливо
    // Утечка топлива (включая сжиженный газ)
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(9) > tbody > :nth-child(43) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // table 5
    /////////// Требуется ремонт
    /// Внешний вид
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(11) > tbody > :nth-child(2) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Встроенный
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(11) > tbody > :nth-child(2) > :nth-child(4)",
        "src",
        "img/check_on.gif",
    );
    // Блеск
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(11) > tbody > :nth-child(3) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // Уборка помещений
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(11) > tbody > :nth-child(3) > :nth-child(4)",
        "src",
        "img/check_on.gif",
    );
    // Колесо
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(11) > tbody > :nth-child(4) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // kakoe
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(11) > tbody > :nth-child(4) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Шины
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(11) > tbody > :nth-child(5) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    // kakoe
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(11) > tbody > :nth-child(5) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );
    // Стекло
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(11) > tbody > :nth-child(6) > :nth-child(2)",
        "src",
        "img/check_on.gif",
    );
    //////////// Основные предметы
    // Статус удержания
    let title = extract_near_text_with(
        document,
        "div.main-listbox > :nth-child(11) > tbody > :nth-child(7) > :nth-child(3)",
        "src",
        "img/check_on.gif",
    );

    // table 6
    // (15) Особые замечания и заключения инспекторов
    // Инспектор по производительности труда и охране здоровья
    let title = extract_value(
        document,
        "div.main-listbox > :nth-child(13) > tbody > :nth-child(1) > td",
    );
    // Цена · Расчет стоимости обследования
    let title = extract_value(
        document,
        "div.main-listbox > :nth-child(13) > tbody > :nth-child(2) > td",
    );
    // img
    let title = extract_attrs(
        document,
        "src",
        "div.main-listbox > :nth-child(17) > tbody > tr > td > p > img",
    );
    println!("{title:?}");
    Ok(())
}
