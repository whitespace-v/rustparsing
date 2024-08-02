use crate::extractor::extract::{extract_attr, extract_attrs, extract_value, extract_values};
use std::error::Error;

pub fn parse(
    car_seq: &str,
    diag_car_yn: &str,
    diag_car_seq: &str,
    premium_car_yn: &str,
) -> Result<(), Box<dyn Error>> {
    let resp: String = ureq::post("https://www.kbchachacha.com/public/layer/car/check/info.kbc")
        .send_form(&[
            ("layerId", "layerCarCheckInfo"),
            ("carSeq", car_seq),
            ("diagCarYn", diag_car_yn),
            ("diagCarSeq", diag_car_seq),
            ("premiumCarYn", premium_car_yn),
        ])
        .unwrap()
        .into_string()
        .unwrap();
    let document = &scraper::Html::parse_document(&resp);
    // name
    let title = extract_value(document, "div.ch-car-name");
    // number
    let title = extract_value(document, "div.ch-car-data > span.cd-right > span.txt-ho");
    // date of seclist
    let title = extract_value(
        document,
        "div.ch-car-data > span.cd-left > div.data-line > :nth-child(1)",
    );
    // probeg
    let title = extract_value(
        document,
        "div.ch-car-data > span.cd-left > div.data-line > :nth-child(2)",
    );
    // toplivo
    let title = extract_value(
        document,
        "div.ch-car-data > span.cd-left > div.data-line > :nth-child(3)",
    );
    // city
    let title = extract_value(
        document,
        "div.ch-car-data > span.cd-left > div.data-line > :nth-child(4)",
    );
    // Название автомобиля
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(1) > td",
    );
    // grz
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(2) > td",
    );
    // god vypuska
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(3) > td",
    );
    // Срок действия проверки
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(4) > td",
    );
    // Дата первой регистрации
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(5) > td",
    );
    // Номер ходовой части
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(6) > td",
    );
    // Типы коробок передач
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(7) > td > div",
    );
    // Использовать топливо
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(8) > td > div",
    );
    // Тип первичного двигателя
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(7) > td",
    );
    // Тип гарантии
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(10) > td > div",
    );
    // Баланс цен Справочная цена
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(8) > table > tbody > :nth-child(11) > td",
    );
    // Состояние датчика пробега
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(1) > td > div",
    );
    // Статус пробега
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(2) > :nth-child(2) > div",
    );
    // probeg
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(1) > :nth-child(3)",
    );
    // Обозначение номера транспортного средства
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(3) > :nth-child(2) > div",
    );
    // vibrosy nazvanie
    let title = extract_values(
        document,
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(4) > :nth-child(2) > div",
    );
    // // pokazateli vibrosov
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(4) > :nth-child(3)",
    );
    // tuning
    let title = extract_attrs(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(5) > :nth-child(2) > div",
    );
    // Особая история
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(6) > :nth-child(2) > div",
    );
    // navodnenie / ogon
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(6) > :nth-child(3) > div",
    );
    // Изменение способа использования
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(7) > :nth-child(2) > div",
    );
    // arenda / prodaja
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(7) > :nth-child(3) > div",
    );
    // Цвет hrom/ahrom
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(8) > :nth-child(2) > div",
    );
    // polnocvet / Изменение цвета
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(8) > :nth-child(3) > div",
    );
    // Основные опции
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(9) > :nth-child(3) > div",
    );
    println!("{title:?}");
    // navigacia / luk / drugoe
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(10) > table > tbody > :nth-child(9) > :nth-child(3) > div",
    );
    // table 4
    // История несчастных случаев (Примечание 5)
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(12) > table > tbody > :nth-child(1) > :nth-child(2) > div",
    );
    // Простой ремонт
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(12) > table > tbody > :nth-child(1) > :nth-child(4) > div",
    );
    // Наружная пластина 1 ранга
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(12) > table > tbody > :nth-child(2) > :nth-child(3) > div",
    );
    // Наружная пластина 2 ранга
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(12) > table > tbody > :nth-child(3) > :nth-child(2) > div",
    );
    // Основной блок
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(12) > table > tbody > :nth-child(4) > :nth-child(2) > div",
    );
    // дополнение к основному блоку
    let title = extract_values(
        document,
        "div.cmm-table:nth-child(12) > table > tbody > :nth-child(4) > :nth-child(2) > div > span.on",
    );
    // ранги
    let title = extract_attrs(
        document,
        "class",
        "div.repair-check-area > div > span > span:not(.blind)",
    );
    let title = extract_values(
        document,
        "div.repair-check-area > div > span > span:not(.blind)",
    );
    let title = extract_attr(document, "value", "input[id=carCheck]");
    let mut s = title.split("").collect::<Vec<&str>>();
    s.remove(0);
    s.pop();
    // X - Замена, C - Коррозия, W - листовой металл/сварка, A - Царапины, U - неровности, T - повреждения
    let names: Vec<&str> = vec![
        // 1
        "Капот",
        // 2
        "Левая Передняя планка",
        "Правая Передняя планка",
        // 3
        "Передняя левая дверь",
        "Передняя правая дверь",
        "Задняя левая дверь",
        "Задняя правая Дверь",
        // 4
        "Решетка багажника",
        // 5
        "Опора радиатора",
        // 6
        "Левая Боковая панель (задняя часть)",
        "Правая Боковая панель (задняя часть)",
        // 7
        "Петлевая панель",
        // 8
        "Левая Боковая панель салона",
        "Правая Боковая панель салона",
        // 9
        "Передняя панель",
        // 10
        "Поперечины",
        // 11
        "Передняя левая Внутренняя панель",
        "Передняя правая Внутренняя панель",
        // 12
        "(Слева) Боковые элементы",
        "(Справа) Боковые элементы",
        "(Переднее сиденье) Боковые элементы",
        "(Передний правый) Боковые элементы",
        // 13
        "Передняя рулевая рубка (слева)",
        "Передняя рулевая рубка (справа)",
        "Задняя рулевая рубка (слева)",
        "Задняя рулевая рубка (справа)",
        // 14
        "Панель стойки (передняя левая)",
        "Панель стойки (передняя правая)",
        "Панель стойки (центральная слева)",
        "Панель стойки (центральная справа)",
        "Панель стойки (задняя)",
        // 15
        "Приборная панель",
        "", // 34 - empty
        // 16
        "Панель пола",
        "", // 36 - empty
        // 17
        "Пол багажника",
        // 18
        "Задняя панель",
        // 19
        "Лоток для упаковки",
    ];
    for (idx, mark) in s.iter().enumerate() {
        match mark.to_owned() {
            " " => {}
            _ => {
                println!("[{}]- {}", mark, names[idx]);
            }
        }
    }
    ////////////////// Самодиагностика
    // Первичный двигатель
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(1) > :nth-child(3) > div",
    );
    // Коробка передач
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(2) > :nth-child(2) > div",
    );
    /////////////////// Первичный двигатель
    // Рабочее состояние (холостой ход)
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(3) > :nth-child(3) > div",
    );
    ///////// Утечка масла Масло
    // Крышка цилиндра (крышка коромысла)
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(4) > :nth-child(3) > div",
    );
    // Головка блока цилиндров / прокладка
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(5) > :nth-child(2) > div",
    );
    // Блок цилиндров / Масляный поддон Производитель Китай
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(6) > :nth-child(2) > div",
    );
    //
    // Утечка нефти и загрязнение окружающей среды
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(7) > :nth-child(3) > div",
    );
    //
    ////////// Утечка охлаждающей жидкости
    // Головка блока цилиндров / прокладка
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(8) > :nth-child(3) > div",
    );
    // Боевой насос
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(9) > :nth-child(3) > div",
    );
    // Радиатор
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(10) > :nth-child(3) > div",
    );
    // Количество охлаждающей жидкости
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(11) > :nth-child(3) > div",
    );
    //
    // Насос высокого давления (Common Rail) - Дизельный двигатель
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(12) > :nth-child(2) > div",
    );
    /////////////////// Коробка передач
    ////////// автомат
    // Утечка масла Масло
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(13) > :nth-child(4) > div",
    );
    // Расход масла и детали
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(14) > :nth-child(2) > div",
    );
    // Рабочее состояние (холостой ход)
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(15) > :nth-child(2) > div",
    );
    ////////// палка
    // Утечка масла Масло
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(16) > :nth-child(3) > div",
    );
    // Переключение передач
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(17) > :nth-child(2) > div",
    );
    // Расход и состояние масла
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(18) > :nth-child(2) > div",
    );
    // Рабочее состояние (холостой ход)
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(19) > :nth-child(2) > div",
    );
    /////////////////// Передача электроэнергии
    // Сборщик сцепления
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(20) > :nth-child(3) > div",
    );
    // Соединение с постоянной скоростью
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(21) > :nth-child(2) > div",
    );
    // Приводной вал и подшипник
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(22) > :nth-child(2) > div",
    );
    // Дифференциальная передача
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(23) > :nth-child(2) > div",
    );
    /////////////////// Рулевое управление
    // Рабочее масло для гидроусилителя рулевого управления Без масла
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(24) > :nth-child(3) > div",
    );
    /////// рабочее состояние
    // Насос рулевого управления
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(25) > :nth-child(3) > div",
    );
    // Рулевой механизм с MDPS
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(26) > :nth-child(2) > div",
    );
    // Шарнир рулевого управления
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(27) > :nth-child(2) > div",
    );
    // Силовой шланг высокого давления
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(28) > :nth-child(2) > div",
    );
    // Конец рулевой тяги и Шаровая зона
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(29) > :nth-child(2) > div",
    );
    /////////////////// Тормозной
    // Утечка масла из Главного тормозного цилиндра
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(30) > :nth-child(3) > div",
    );
    // Утечка тормозного масла
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(31) > :nth-child(2) > div",
    );
    // Состояние источника питания
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(32) > :nth-child(2) > div",
    );
    /////////////////// Электричество
    // Выход генератора
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(33) > :nth-child(3) > div",
    );
    // Пусковой двигатель
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(34) > :nth-child(2) > div",
    );
    // Функция двигателя стеклоочистителя
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(35) > :nth-child(2) > div",
    );
    // Двигатель для вентиляции помещений
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(36) > :nth-child(2) > div",
    );
    // Двигатель вентилятора радиатора
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(37) > :nth-child(2) > div",
    );
    // Работа стеклоподъемника
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(38) > :nth-child(2) > div",
    );
    /////////////////// Электрическое устройство высокой мощности
    // Состояние изоляции зарядного порта
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(39) > :nth-child(3) > div",
    );
    // Состояние изоляции аккумуляторной батареи привода
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(40) > :nth-child(2) > div",
    );
    // Состояние электропроводки высокой мощности (Соединительная клемма, ткань, защитный механизм)
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(41) > :nth-child(2) > div",
    );
    /////////////////// Топливо
    // Утечка топлива (включая сжиженный газ)
    let title = extract_attr(
        document,
        "value",
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(42) > :nth-child(3) > div",
    );
    /////////////////// Специфика и обратная связь с инспекторами
    // Эффективность работы -Санитарный инспектор
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(43) > :nth-child(3)",
    );
    // Обзор цен на сперму
    let title = extract_value(
        document,
        "div.cmm-table:nth-child(16) > table > tbody > :nth-child(44) > :nth-child(2)",
    );
    println!("{title}");
    Ok(())
}
