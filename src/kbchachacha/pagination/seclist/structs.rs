use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecList {
    //*** Table 1 ***///
    // номер листа
    pub seclist_num: String,
    // название авто
    pub name: String,
    // расширенное название авто
    pub ext_name: String,
    // госномер
    pub license_plate: String,
    // год выпуска авто
    pub release_year: String,
    // срок действия техотчета
    pub validity_period: String,
    // дата первой регистрации авто
    pub first_reg_date: String,
    // номер шасси
    pub chassis_number: String,
    // тип трансмиссии
    pub transmission_types: Vec<String>,
    // тип топлива
    pub fuel_type: Vec<String>,
    // двигатель
    pub engine: String,
    // тип страховки
    pub warranty_type: String,
    //*** Table 2 ***///
    // статус одометра
    pub odometr_status: String,
    // статус пробега
    pub mileage_status: String,
    // пробег в км
    pub mileage_value: String,
    // статус таблички с вином
    pub vin_plate_status: String,
    // выбросы, названия
    pub emission_names: Vec<String>,
    // выбросы, показатели
    pub emission_values: Vec<String>,
    // тюнинг статус
    pub tuning_status: String,
    // тюнинг легальность
    pub tuning_legality: String,
    // тюнинг: structure or device
    pub tuning_type: String,
    // инциденты, статус
    pub incidents: String,
    // инциденты: пожары, наводнения
    pub incidents_flood_fire: String,
    // смена владельца: статус
    pub ownership_changes_status: String,
    // был ли в продаже / в аренде
    pub ownership_changes_value: String,
    // хром (от белого до черного) ахром(все цветные)
    pub color_changes_chrome: String,
    // монохром / крашена
    pub color_changes_type: String,
    // замена агрегатов
    pub options_status: String,
    // навигация, люк, другое
    pub options_list: Vec<String>,
    // Отзывалась ли на ремонт дилером да/ нет:
    pub feedback_status: String,
    // отремонтировано / не отремантировано
    pub feedback_value: String,
    // Table 3
    // схема
    pub point_scheme: CarSecListPointScheme,
    // Table 4
    pub extended_table: CarSecListExtendedTable,
    // Table 5
    pub description_table: CarSecListDescription,
    // Table 6
    pub opinions_table: CarSecListOpinions,
    pub images: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListPointScheme {
    pub out: CarSecListPointSchemeOut,
    pub bones: CarSecListPointSchemeBones,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListPointSchemeOut {
    pub first: Vec<CarSecListPointSchemeItem>,
    pub second: Vec<CarSecListPointSchemeItem>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListPointSchemeBones {
    pub a: Vec<CarSecListPointSchemeItem>,
    pub b: Vec<CarSecListPointSchemeItem>,
    pub c: Vec<CarSecListPointSchemeItem>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListPointSchemeItem {
    pub index: u8,
    pub mark: String,
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTable {
    pub diagnostics: CarSecListExtendedTableDiagnostics,
    pub engine: CarSecListExtendedTableEngine,
    pub transmission: CarSecListExtendedTableTransmission,
    pub chassis: CarSecListExtendedTableChassis,
    pub steering: CarSecListExtendedSteering,
    pub brakes: CarSecListExtendedBrakes,
    pub electics: CarSecListExtendedElectrics,
    pub wiring: CarSecListExtendedWiring,
    pub fuel: CarSecListExtendedFuel,
}
//////////// 1 Самодиагностика
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableDiagnostics {
    // Двигатель
    pub diagnostics_engine: String,
    // Коробка передач
    pub diagnostics_transmission: String,
}
/////////// 2 Двигатель
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableEngine {
    // холостой ход
    pub engine_idling: String,
    // Утечка масла
    pub engine_oil_leak: CarSecListExtendedTableEngineOilLeak,
    // Давление масла
    pub engine_oil_pressure: String,
    // Утечка охлаждающей жидкости
    pub engine_coolant_leak: CarSecListExtendedTableEngineCoolantLeak,
    // Общая магистраль
    pub engine_line: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableEngineOilLeak {
    // Клапанная крышка
    pub engine_oil_leak_valve_cover: String,
    // Прокладка ГБЦ
    pub engine_oil_leak_cylynder_head_gasket: String,
    // Поддон
    pub engine_oil_leak_pan: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableEngineCoolantLeak {
    // ГБЦ
    pub engine_coolant_leak_cylynder_head: String,
    // Помпа
    pub engine_coolant_leak_pump: String,
    // Радиатор
    pub engine_coolant_leak_radiator: String,
    // Количество охлаждающей жидкости
    pub engine_coolant_leak_coolant_amount: String,
}
//////////// 3 Коробка передач
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableTransmission {
    ///// Автоматическая коробка передач (A/T)
    pub automatic: CarSecListExtendedTableTransmissionAutomatic,
    ///// Механическая коробка передач (M/T)
    pub manual: CarSecListExtendedTableTransmissionManual,
}
/////// Механическая коробка передач (M/T)
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableTransmissionManual {
    // Утечка масла
    pub tm_oil_leak: String,
    // Переключение передач
    pub tm_gear_shift: String,
    // Расход и состояние масла
    pub tm_oil_consumption: String,
    // Рабочее состояние (холостой ход)
    pub tm_idling: String,
}
///// Автоматическая коробка передач (A/T)
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableTransmissionAutomatic {
    // Утечка масла Масло
    pub at_oil_leak: String,
    // Расход и состояние масла
    pub at_oil_consumption: String,
    // Рабочее состояние (холостой ход)
    pub at_idling: String,
}
/////////// 4 Ходовая часть
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedTableChassis {
    // Сцепление в сборе
    pub clutch_assembly: String,
    // ШРУС
    pub joints: String,
    // Карданный вал
    pub driveshaft: String,
    // Дифференциал
    pub differential: String,
}
/////////// 5 Рулевое управление
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedSteering {
    // Утечка ГУР
    pub power_steering: String,
    ///// Рабочее состояние
    pub condition: CarSecListExtendedSteeringCondition,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedSteeringCondition {
    // Насос ГУР
    pub steering_pump: String,
    // Рулевой механизм
    pub steering_gear: String,
    // Рулевой кардан
    pub steering_propshaft: String,
    // Шланги и трубки
    pub steering_hoses_n_tubes: String,
    // Наконечник рулевой тяги и Шаровой шарнир
    pub steering_rack: String,
}
/////////// 6 Тормозная система
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedBrakes {
    // Утечка масла в Главном тормозном цилиндре
    pub main_brake_cylinder_leak: String,
    // Утечка тормозной жидкости
    pub brake_fluid_leak: String,
    // Датчик тормозной жидкости
    pub brake_fluid_sensor: String,
}
////////// 7 Электричество
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedElectrics {
    // Генератор
    pub electrics_generator: String,
    // Стартер
    pub electrics_starter: String,
    // Функция двигателя стеклоочистителя
    pub electrics_wipers_electrics: String,
    // Вентилятор охлаждения Двигателя
    pub electrics_engine_fan: String,
    // Двигатель вентилятора радиатора
    pub electrics_engine_fan_motor: String,
    // Привод стеклоподъемника
    pub electrics_window_lifter_drive: String,
}
/////////// 8 Электропроводка
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedWiring {
    // Состояние изоляции зарядного порта
    pub charger_insulation: String,
    // Состояние изоляции аккумуляторной батареи
    pub battery_insulation: String,
    // Состояние электропроводки высокой мощности
    pub high_power_wiring: String,
}
/////////// 9 Топливо
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListExtendedFuel {
    // Утечка топлива (включая сжиженный газ)
    pub fuel_n_gas_leak: String,
}

// Table 5
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListDescription {
    pub repair_required: CarSecListDescriptionRepair,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListDescriptionRepair {
    // внешний вид
    pub appearance: String,
    // интерьер
    pub interior: String,
    // блеск
    pub gloss: String,
    // клининг
    pub cleaning: String,
    // колеса
    pub wheels_status: String,
    // колеса: 1,2,3,4, ЧС
    pub wheels: Vec<String>,
    // шины
    pub tires_status: String,
    // шины: 1,2,3,4, ЧС
    pub tires: Vec<String>,
    pub window: String,
    pub additional_items: String,
    pub user_manual: String,
    pub emergency_stop_sign: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecListOpinions {
    pub performance_n_health_inspector: String,
    pub price_survey: String,
}
