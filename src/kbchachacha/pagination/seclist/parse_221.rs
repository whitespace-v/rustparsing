use scraper::Html;
use std::error::Error;

use crate::extractor::extract::{extract_attrs, extract_value, extract_values, with_checked_label};

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    // number
    let title = extract_value(document, "b.issue-number");
    // name
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(2) > :nth-child(2)",
    );
    // grz
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(2) > :nth-child(4)",
    );

    // god
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(3) > :nth-child(2)",
    );

    // srok deistvia tehosmotra
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(3) > :nth-child(4)",
    );

    // vin
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(5) > :nth-child(2)",
    );

    // engine
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(7) > :nth-child(2)",
    );

    // type kpp
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(4) > :nth-child(4) > label",
    );

    // toplivo
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(6) > :nth-child(2) > label",
    );

    // toplivo
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(7) > :nth-child(4) > label",
    );

    // Пробег
    // znachenie
    let title = extract_value(
        document,
        ".type-data > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child(10) > td:nth-child(3) > dl:nth-child(1) > dd:nth-child(2)",
    );
    // Хороший Плохой
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(10) > :nth-child(2) > label",
    );
    // mnogo obichno menshe
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(11) > :nth-child(1) > label",
    );
    // Обозначение номера автомобиля
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(12) > :nth-child(2) > label",
    );
    // Выбросы
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(13) > :nth-child(2) > label",
    );
    let title = extract_value(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(13) > :nth-child(3)",
    );
    // Настройка
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(14) > :nth-child(2) > label",
    );
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(14) > :nth-child(3) > label",
    );
    // Особая история
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(15) > :nth-child(2) > label",
    );
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(15) > :nth-child(3) > label",
    );
    // Изменение способа использования
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(16) > :nth-child(2) > label",
    );
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(16) > :nth-child(3) > label",
    );
    // Цвет
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(17) > :nth-child(2) > label",
    );
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(17) > :nth-child(3) > label",
    );
    // Основные опции 18
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(18) > :nth-child(2) > label",
    );
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(1) > tbody > :nth-child(18) > :nth-child(3) > label",
    );
    // X (замена), W (листовой металл или сварка), A (Царапина), U (неровность), C (коррозия), T (повреждение)
    let data = extract_values(document, "div.historyWrap > ul > li > span > font");
    // left image [2, 8, 3 (front), 3(rear), 6, (front stoyka), 14, (rear stoyka)]
    // 2 image [1, 7, 4]
    // 3 image [5, 9, 11 (left), 11(right), 12(left), 12(right), 13 (front-left), 13 (front-right), 10, 15, 16, 19, 13 (left-back), 13 (right-back), 12 (left-back), 12 (right-back), 17, 18]
    // 4 image [2, 8, 3 (front), 3 (rear), 6, front stoyka, 14, rear stoyka]

    // История несчастных случаев
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(2) > tbody > :nth-child(4) > :nth-child(2) > label",
    );
    // Простой ремонт
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(2) > tbody > :nth-child(4) > :nth-child(4) > label",
    );
    // Имеются ли отклонения в зависимости от места расположения
    // Наружная пластина 1 ранга
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(2) > tbody > :nth-child(5) > :nth-child(3) > label",
    );
    // Наружная пластина 2 ранга
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(2) > tbody > :nth-child(6) > :nth-child(2) > label",
    );
    // Основной блок
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(2) > tbody > :nth-child(7) > :nth-child(2) > label",
    );
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(2) > tbody > :nth-child(7) > :nth-child(3) > label",
    );
    // out
    //1
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(2) > tbody > :nth-child(9) > :nth-child(3) > label",
    );
    //2
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(2) > tbody > :nth-child(10) > :nth-child(2) > label",
    );
    // bones
    //a
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(2) > tbody > :nth-child(11) > :nth-child(3) > label",
    );
    //b
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(2) > tbody > :nth-child(12) > :nth-child(2) > label",
    );
    //c
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(2) > tbody > :nth-child(13) > :nth-child(2) > label",
    );
    // table 4
    //////////// Самодиагностика
    // Первичный двигатель
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(3) > :nth-child(3) > label",
    );
    // Коробка передач
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(4) > :nth-child(2) > label",
    );
    //////////// Первичный двигатель
    // Рабочее состояние (холостой ход)
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(5) > :nth-child(3) > label",
    );
    /////// Утечка масла Масло
    // Крышка коромысла
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(6) > :nth-child(3) > label",
    );
    // Коллектор цилиндра/прокладка
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(7) > :nth-child(2) > label",
    );
    // Масляный поддон
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(8) > :nth-child(2) > label",
    );
    //
    // Утечка нефти и загрязнение окружающей среды
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(9) > :nth-child(2) > label",
    );
    ///// Утечки охлаждающей жидкости
    // Коллектор цилиндра/прокладка
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(10) > :nth-child(3) > label",
    );
    // водяной насос
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(11) > :nth-child(2) > label",
    );
    // Радиатор
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(12) > :nth-child(2) > label",
    );
    // Количество охлаждающей жидкости
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(13) > :nth-child(2) > label",
    );
    //
    // Насос высокого давления (Common Rail) - Дизельный двигатель
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(14) > :nth-child(2) > label",
    );
    //////////// Коробка передач
    /////// avtomat
    // Утечка масла Масло
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(15) > :nth-child(4) > label",
    );
    // Расход и состояние масла
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(16) > :nth-child(2) > label",
    );
    // Рабочее состояние (холостой ход)
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(17) > :nth-child(2) > label",
    );
    /////// palka
    // Утечка масла Масло
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(18) > :nth-child(3) > label",
    );
    // Переключение передач
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(19) > :nth-child(2) > label",
    );
    // Расход и состояние масла
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(20) > :nth-child(2) > label",
    );
    // Рабочее состояние (холостой ход)
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(21) > :nth-child(2) > label",
    );
    //////////// Передача электроэнергии
    // Сцепление в сборе
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(22) > :nth-child(3) > label",
    );
    // Шарнир с постоянной скоростью вращения
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(23) > :nth-child(2) > label",
    );
    // Приводной вал и подшипник
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(24) > :nth-child(2) > label",
    );
    //////////// Рулевое управление
    // Утечка масла при работе гидроусилителя рулевого управления
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(25) > :nth-child(3) > label",
    );
    ///// Рабочее состояние
    // Рулевой механизм
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(26) > :nth-child(3) > label",
    );
    // Насос рулевого управления
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(27) > :nth-child(2) > label",
    );
    // Наконечник рулевой тяги и шарнирное соединение
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(28) > :nth-child(2) > label",
    );
    //////////// Тормозной
    // Утечка масла из Главного тормозного цилиндра
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(29) > :nth-child(3) > label",
    );
    // Утечка тормозного масла
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(30) > :nth-child(2) > label",
    );
    // Состояние источника питания
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(31) > :nth-child(2) > label",
    );
    //////////// Электричество
    // Выход генератора
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(32) > :nth-child(3) > label",
    );
    // Пусковой двигатель
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(33) > :nth-child(2) > label",
    );
    // Функция двигателя стеклоочистителя
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(34) > :nth-child(2) > label",
    );
    // Двигатель для вентиляции помещений
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(35) > :nth-child(2) > label",
    );
    // Двигатель вентилятора радиатора
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(36) > :nth-child(2) > label",
    );
    // Привод стеклоподъемника
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(37) > :nth-child(2) > label",
    );
    //////////// Другой
    // Утечка топлива (включая сжиженный газ)
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(3) > tbody > :nth-child(38) > :nth-child(3) > label",
    );
    /////////////////////////
    // table 5
    ///////////////////////// Требуется ремонт
    // Внешний вид
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(3) > :nth-child(3) > label",
    );
    // Блеск
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(4) > :nth-child(2) > label",
    );
    // Встроенный
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(5) > :nth-child(2) > label",
    );
    // Уборка помещений
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(6) > :nth-child(2) > label",
    );
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(6) > :nth-child(3) > label",
    );
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(6) > :nth-child(4) > label",
    );
    // Колесо
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(7) > :nth-child(2) > label",
    );
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(7) > :nth-child(3) > label",
    );
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(7) > :nth-child(4) > label",
    );
    // Шины
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(8) > :nth-child(2) > label",
    );
    // Стекло
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(9) > :nth-child(2) > label",
    );
    ///////////////////////// Основные предметы
    // Статус удержания
    let title = with_checked_label(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(10) > :nth-child(3) > label",
    );
    //kommentariy
    let title = extract_value(
        document,
        "div.type-data > :nth-child(4) > tbody > :nth-child(11) > :nth-child(3)",
    );
    //imgs
    let title = extract_attrs(
        document,
        "src",
        "div.type-data > :nth-child(5) > tbody > :nth-child(4) > td > ul > li > img",
    );
    println!("{title:?}");
    Ok(())
}
