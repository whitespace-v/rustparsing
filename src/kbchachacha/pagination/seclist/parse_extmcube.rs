use crate::extractor::extract::{extract_attrs, extract_value, extract_values};
use scraper::Html;
use std::error::Error;

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    // N of seclist
    let title = extract_value(document, "span.ckdate");
    // table 1
    // Название автомобиля
    let title = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(1) > :nth-child(2)",
    );
    //god vipuska
    let title = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(1) > :nth-child(4)",
    );
    //Номер транспортного средства
    let title = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(2) > :nth-child(2)",
    );
    //Срок действия проверки
    let title = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(2) > :nth-child(4)",
    );
    //Дата первой регистрации
    let title = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(3) > :nth-child(2)",
    );
    //Тип трансмиссии
    let title = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(3) > :nth-child(4)",
    );
    //Использованное топливо
    let title = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(4) > :nth-child(2)",
    );
    //Номер ходовой части
    let title = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(4) > :nth-child(4)",
    );
    //Тип гарантии
    let title = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(5) > :nth-child(2)",
    );
    //Тип первичного двигателя
    let title = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(5) > :nth-child(4)",
    );
    //Расчет базовой цены
    let title = extract_value(
        document,
        "div.inspec_carinfo > table > tbody > :nth-child(6) > :nth-child(2)",
    );
    // table 2 state / comment(if exists)
    // Состояние датчика пробега
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(1) > :nth-child(2) > span.on",
    );
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(1) > :nth-child(3)",
    );
    // Пробег
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(2) > :nth-child(2) > span.active",
    );
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(2) > :nth-child(3)",
    );
    // Обозначение номера транспортного средства
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(3) > :nth-child(2) > span.on",
    );
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(3) > :nth-child(3)",
    );
    // Выбросы
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(4) > :nth-child(2)",
    );
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(4) > :nth-child(3)",
    );
    // Настройка
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(5) > :nth-child(2) > span.on",
    );
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(5) > :nth-child(3)",
    );
    // Особая история
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(6) > :nth-child(2) > span.on",
    );
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(6) > :nth-child(3)",
    );
    // Изменение способа использования
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(7) > :nth-child(2) > span.on",
    );
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(7) > :nth-child(3) > span.on",
    );
    // Цвет
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(8) > :nth-child(2) > span.on",
    );
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(8) > :nth-child(3)",
    );
    // Основные опции
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(9) > :nth-child(2) > span.on",
    );
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(9) > :nth-child(3)",
    );
    // Подлежит отзыву
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(10) > :nth-child(2) > span.on",
    );
    let title = extract_value(
        document,
        "div.section_total > table > tbody > :nth-child(10) > :nth-child(3) > span.on",
    );
    // table 3
    // История несчастных случаев Читать далее
    let title = extract_value(
        document,
        "div.section_repair > table > tbody > :nth-child(1) > :nth-child(2) > span.active",
    );
    // Простой ремонт Читать далее
    let title = extract_value(
        document,
        "div.section_repair > table > tbody > :nth-child(2) > :nth-child(2) > span.active",
    );
    // Объем расчета стоимости обследования и особенности его проведения
    let title = extract_value(
        document,
        "div.section_repair > table > tbody > :nth-child(3) > :nth-child(2)",
    );
    // table 4 сопоставить имена со значениями и отправить только те которые есть
    let title = extract_attrs(document, "value", "div[id=dataForm] > input")?;
    let title1 = extract_attrs(document, "name", "div[id=dataForm] > input")?;
    // table 5
    ////////////////////// Самодиагностика
    // Первичный двигатель
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(1) > :nth-child(3) > span.on",
    );
    // Коробка передач
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(2) > :nth-child(2) > span.on",
    );
    ///////////////////// Первичный двигатель
    // Рабочее состояние (холостой ход)
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(3) > :nth-child(3) > span.on",
    );
    ///// Утечка масла Масло
    // Крышка цилиндра (крышка коромысла)
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(4) > :nth-child(3) > span.on",
    );
    // Головка блока цилиндров/прокладка
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(5) > :nth-child(2) > span.on",
    );
    // Блок цилиндров / Масляный поддон Производитель Китай
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(6) > :nth-child(2) > span.on",
    );
    //
    // Поток масла
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(7) > :nth-child(2) > span.on",
    );
    /////// Утечка охлаждающей жидкости
    // Головка блока цилиндров/прокладка
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(8) > :nth-child(3) > span.on",
    );
    // водяной насос
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(9) > :nth-child(2) > span.on",
    );
    // Радиатор
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(10) > :nth-child(2) > span.on",
    );
    // Количество охлаждающей жидкости
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(11) > :nth-child(2) > span.on",
    );
    //
    // Общая магистраль
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(12) > :nth-child(2) > span.on",
    );
    /////////////////////// Коробка передач
    ////////// Автоматическая коробка передач (A/T)
    // Утечка масла Масло
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(13) > :nth-child(4) > span.on",
    );
    // Расход и состояние масла
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(14) > :nth-child(2) > span.on",
    );
    // Рабочее состояние (холостой ход)
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(15) > :nth-child(2) > span.on",
    );
    ////////// Механическая коробка передач (M/ T)
    // Утечка масла Масло
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(16) > :nth-child(3) > span.on",
    );
    // Переключение передач
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(17) > :nth-child(2) > span.on",
    );
    // Расход и состояние масла
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(18) > :nth-child(2) > span.on",
    );
    // Рабочее состояние (холостой ход)
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(19) > :nth-child(2) > span.on",
    );
    ////////////////////// Передача электроэнергии
    // Сцепление в сборе
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(20) > :nth-child(3) > span.on",
    );
    // Шарнир с постоянной скоростью вращения
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(21) > :nth-child(2) > span.on",
    );
    // Приводной вал и подшипник
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(22) > :nth-child(2) > span.on",
    );
    // Дифференциальная передача
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(23) > :nth-child(2) > span.on",
    );
    ////////////////////// Рулевое управление
    // Утечка масла при работе гидроусилителя рулевого управления
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(24) > :nth-child(3) > span.on",
    );
    /////// Рабочее состояние
    // Насос рулевого управления
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(25) > :nth-child(2) > span.on",
    );
    // Рулевой механизм с MDPS
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(26) > :nth-child(2) > span.on",
    );
    // Шарнир рулевого управления
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(27) > :nth-child(2) > span.on",
    );
    // Силовой шланг высокого давления
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(28) > :nth-child(2) > span.on",
    );
    // Наконечник рулевой тяги и Шаровой шарнир
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(29) > :nth-child(3) > span.on",
    );
    ////////////////////// Тормозной
    // Утечка масла из Главного тормозного цилиндра
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(30) > :nth-child(2) > span.on",
    );
    // Утечка тормозного масла
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(31) > :nth-child(2) > span.on",
    );
    // Состояние источника питания
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(32) > :nth-child(3) > span.on",
    );
    ////////////////////// Электричество
    // Выход генератора
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(33) > :nth-child(3) > span.on",
    );
    // Пусковой двигатель
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(34) > :nth-child(2) > span.on",
    );
    // Функция двигателя стеклоочистителя
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(35) > :nth-child(2) > span.on",
    );
    // Двигатель для вентиляции помещений
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(36) > :nth-child(2) > span.on",
    );
    // Двигатель вентилятора радиатора
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(37) > :nth-child(2) > span.on",
    );
    // Привод стеклоподъемника
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(38) > :nth-child(2) > span.on",
    );
    ////////////////////// Мощное электрическое устройство
    // Состояние изоляции зарядного порта
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(39) > :nth-child(3) > span.on",
    );
    // Состояние изоляции аккумуляторной батареи привода
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(40) > :nth-child(2) > span.on",
    );
    // Состояние электропроводки высокой мощности (соединительная клемма, ткань, защитный механизм)
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(41) > :nth-child(2) > span.on",
    );
    ////////////////////// Топливо
    // Утечка топлива (включая сжиженный газ)
    let title = extract_value(
        document,
        "div.section_detail > table > tbody > :nth-child(42) > :nth-child(3) > span.on",
    );

    // table 6
    ///////////// Требуется ремонт
    // Внешний вид
    let title = extract_value(
        document,
        "div.section_etc > table > tbody > :nth-child(1) > :nth-child(3) > span.on",
    );
    // Встроенный
    let title = extract_value(
        document,
        "div.section_etc > table > tbody > :nth-child(2) > :nth-child(2) > span.on",
    );
    // Блеск
    let title = extract_value(
        document,
        "div.section_etc > table > tbody > :nth-child(3) > :nth-child(2) > span.on",
    );
    // Уборка помещений
    let title = extract_value(
        document,
        "div.section_etc > table > tbody > :nth-child(4) > :nth-child(2) > span.on",
    );
    // Колесо
    let title = extract_value(
        document,
        "div.section_etc > table > tbody > :nth-child(5) > :nth-child(2) > span.on",
    );
    let title = extract_values(
        document,
        "div.section_etc > table > tbody > :nth-child(5) > :nth-child(2) > :nth-child(3) > span.on",
    );
    // Шины
    let title = extract_value(
        document,
        "div.section_etc > table > tbody > :nth-child(6) > :nth-child(2) > span.on",
    );
    let title = extract_values(
        document,
        "div.section_etc > table > tbody > :nth-child(6) > :nth-child(2) > :nth-child(3) > span.on",
    );
    // Стекло
    let title = extract_value(
        document,
        "div.section_etc > table > tbody > :nth-child(7) > :nth-child(2) > span.on",
    );
    ///////////// Основные предметы
    // Статус удержания
    let title = extract_value(
        document,
        "div.section_etc > table > tbody > :nth-child(8) > :nth-child(2) > span.on",
    );
    //// table 7
    //// Особенности и мнения инспекторов
    //// Инспектор по производительности труда и охране здоровья
    let title = extract_value(
        document,
        "div.section_opinion > table > tbody > :nth-child(1) > td",
    );
    //// Цена · Расчет стоимости обследования
    let title = extract_value(
        document,
        "div.section_opinion > table > tbody > :nth-child(2) > td",
    );
    // images
    let title = extract_attrs(document, "src", "img.img_inspect");
    //date
    let title = extract_value(document, "div.inspc_sign > div.boxst > p.date");
    println!("{title}");
    Ok(())
}
