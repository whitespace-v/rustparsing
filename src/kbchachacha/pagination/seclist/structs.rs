use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecList {
    //*** Table 1 ***///
    // url источник
    origin: String,
    // номер листа
    seclist_num: String,
    // название авто
    name: String,
    // расширенное название авто
    ext_name: String,
    // госномер
    license_plate: String,
    // год выпуска авто
    release_year: String,
    // срок действия техотчета
    validity_period: String,
    // дата первой регистрации авто
    first_reg_date: String,
    // номер шасси
    chassis_number: String,
    // тип трансмиссии
    transmission_types: Vec<String>,
    // тип топлива
    fuel_type: Vec<String>,
    // двигатель
    engine: String,
    // тип страховки
    warranty_type: String,
    //*** Table 2 ***///
    // статус одометра
    odometr_status: String,
    // статус пробега
    mileage_status: String,
    // пробег в км
    mileage_value: String,
    // статус таблички с вином
    vin_plate_status: String,
    // выбросы, названия
    emission_names: Vec<String>,
    // выбросы, показатели
    emission_values: String,
    // тюнинг статус
    tuning_status: String,
    // тюнинг легальность
    tuning_legality: String,
    // тюнинг: structure or device
    tuning_type: Vec<String>,
    // инциденты, статус
    incidents: String,
    // инциденты: пожары, наводнения
    incidents_flood_fire: Vec<String>,
    // смена владельца: статус
    ownership_changes_status: String,
    // был ли в продаже / в аренде
    ownership_changes_value: Vec<String>,
    // хром (от белого до черного) ахром(все цветные)
    color_changes_chrome: String,
    // монохром / крашена
    color_changes_type: Vec<String>,
    // замена агрегатов
    options_status: String,
    // навигация, люк, другое
    options_list: Vec<String>,
    // Отзывалась ли на ремонт дилером да/ нет:
    feedback_status: String,
    // отремонтировано / не отремантировано
    feedback_value: String,
    // Table 3
    // схема
    point_scheme: CarSecListPointScheme,
    // Table 4
    extended_table: CarSecListExtendedTable,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListPointScheme {
    out: CarSecListPointSchemeOut,
    bones: CarSecListPointSchemeBones,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListPointSchemeOut {
    #[serde(rename = "1")]
    first: String,
    #[serde(rename = "2")]
    second: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListPointSchemeBones {
    #[serde(rename = "A")]
    a: String,
    #[serde(rename = "B")]
    b: String,
    #[serde(rename = "C")]
    c: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListPointSchemeItem {
    idx: u8,
    mark: String,
    title: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTable {
    diagnostics: CarSecListExtendedTableDiagnostics,
    engine: CarSecListExtendedTableEngine,
    transmission: CarSecListExtendedTableTransmission,
    chassis: CarSecListExtendedTableChassis,
    steering: CarSecListExtendedSteering,
    brakes: CarSecListExtendedBrakes,
    electics: CarSecListExtendedElectrics,
    fuel: CarSecListExtendedFuel,
}
//////////// 1 Самодиагностика
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableDiagnostics {
    // Двигатель
    engine: String,
    // Коробка передач
    transmission: String,
}
/////////// 2 Двигатель
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableEngine {
    // Рабочее состояние (холостой ход)
    idling: String,
    // Утечка масла
    oil_leak: CarSecListExtendedTableEngineOilLeak,
    // Давление масла
    oil_pressure: String,
    // Утечка охлаждающей жидкости
    coolant_leak: CarSecListExtendedTableEngineCoolantLeak,
    // Общая магистраль
    engine_line: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableEngineOilLeak {
    // Клапанная крышка
    valve_cover: String,
    // Прокладка ГБЦ
    cylynder_head_gasket: String,
    // Поддон
    pan: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableEngineCoolantLeak {
    // ГБЦ
    cylynder_head: String,
    // Помпа
    pump: String,
    // Радиатор
    radiator: String,
    // Количество охлаждающей жидкости
    coolant_amount: String,
}
//////////// 3 Коробка передач
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableTransmission {
    ///// Автоматическая коробка передач (A/T)
    automatic: CarSecListExtendedTableTransmissionAutomatic,
    ///// Механическая коробка передач (M/T)
    manual: CarSecListExtendedTableTransmissionManual,
}
/////// Механическая коробка передач (M/T)
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableTransmissionManual {
    // Утечка масла
    oil_leak: String,
    // Переключение передач
    gear_shift: String,
    // Расход и состояние масла
    oil_consumption: String,
    // Рабочее состояние (холостой ход)
    idling: String,
}
///// Автоматическая коробка передач (A/T)
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableTransmissionAutomatic {
    // Утечка масла Масло
    oil_leak: String,
    // Расход и состояние масла
    oil_consumption: String,
    // Рабочее состояние (холостой ход)
    idling: String,
}
/////////// 4 Ходовая часть
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableChassis {
    // Сцепление в сборе
    clutch_assembly: String,
    // ШРУС
    joints: String,
    // Карданный вал
    driveshaft: String,
    // Дифференциал
    differential: String,
}
/////////// 5 Рулевое управление
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedSteering {
    // Утечка масла при работе гидроусилителя рулевого управления
    ///// Рабочее состояние
    // Насос рулевого управления
    // Рулевой механизм с MDPS
    // Шарнир рулевого управления
    // Силовой шланг высокого давления
    // Наконечник рулевой тяги и Шаровой шарнир
}
/////////// 6 Тормозной
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedBrakes {
    // Утечка масла из Главного тормозного цилиндра
    // Утечка тормозного масла
    // Состояние источника питания
}
////////// 7 Электричество
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedElectrics {
    // Выход генератора
    // Пусковой двигатель
    // Функция двигателя стеклоочистителя
    // Двигатель для вентиляции помещений
    // Двигатель вентилятора радиатора
    // Привод стеклоподъемника
}
/////////// 8 Топливо
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedFuel {
    // Утечка топлива (включая сжиженный газ)
}
