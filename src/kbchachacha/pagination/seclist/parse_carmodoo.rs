use super::structs::{
    CarSecList, CarSecListDescription, CarSecListDescriptionRepair, CarSecListExtendedBrakes,
    CarSecListExtendedElectrics, CarSecListExtendedFuel, CarSecListExtendedSteering,
    CarSecListExtendedSteeringCondition, CarSecListExtendedTable, CarSecListExtendedTableChassis,
    CarSecListExtendedTableDiagnostics, CarSecListExtendedTableEngine,
    CarSecListExtendedTableEngineCoolantLeak, CarSecListExtendedTableEngineOilLeak,
    CarSecListExtendedTableTransmission, CarSecListExtendedTableTransmissionAutomatic,
    CarSecListExtendedTableTransmissionManual, CarSecListExtendedWiring, CarSecListOpinions,
};
use crate::{
    extend::Cutter,
    extractor::extract::{
        extract_attrs, extract_ids_from_json_js, extract_value, extract_values,
        extract_with_sibling, with_checked_label,
    },
    kbchachacha::pagination::seclist::{
        scheme_constructor::convert, structs::CarSecListPointSchemeItem,
    },
};
use scraper::Html;
use std::{collections::HashMap, error::Error};

pub fn parse(document: &Html) -> Result<CarSecList, Box<dyn Error>> {
    let seclist_num = extract_value(document, "span.num");
    let name = extract_value(document, "tbody > :nth-child(1) > :nth-child(2)");
    let license_plate = extract_value(document, "tbody > :nth-child(1) > :nth-child(4)");
    let release_year = extract_value(document, "tbody > :nth-child(2) > :nth-child(2)");
    let validity_period = extract_value(document, "tbody > :nth-child(2) > :nth-child(4)");
    let first_reg_date = extract_value(document, "tbody > :nth-child(3) > :nth-child(2)");
    let chassis_number = extract_value(document, "tbody > :nth-child(4) > :nth-child(2)");
    let transmission_types = with_checked_label(
        document,
        "div.page_line > :nth-child(2) > tbody > :nth-child(3) > :nth-child(4) > label",
    );
    let fuel_type = with_checked_label(
        document,
        "div.page_line > :nth-child(2) > tbody > :nth-child(5) > :nth-child(2) > label",
    );
    let engine = extract_value(document, "tbody > :nth-child(6) > :nth-child(2)");
    let warranty_type = extract_value(
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
    let odometr_status = extract_with_sibling(document, &bc["11"]);
    let mileage_status = extract_with_sibling(document, &bc["12"]);
    let mileage_value = extract_value(document, "strong.km");
    let vin_plate_status = extract_with_sibling(document, &bc["2"]);
    let emission_names = with_checked_label(
        document,
        "div.page_line > :nth-child(3) > tbody > :nth-child(5) > :nth-child(2) > label",
    );
    let emission_values = extract_values(
        document,
        "div.page_line > :nth-child(3) > tbody > :nth-child(5) > :nth-child(3) > span",
    );
    let mut tuning_status: String = "".to_owned();
    if bc.contains_key("3") {
        tuning_status = extract_with_sibling(document, &bc["3"]);
    }
    let mut tuning_legality: String = "".to_owned();
    if bc.contains_key("31") {
        tuning_legality = extract_with_sibling(document, &bc["31"]);
    }
    let mut tuning_type: String = "".to_owned();
    if bc.contains_key("32") {
        tuning_type = extract_with_sibling(document, &bc["32"]);
    }
    let mut incidents: String = "".to_owned();
    if bc.contains_key("4") {
        incidents = extract_with_sibling(document, &bc["4"]);
    }
    let mut incidents_flood_fire: String = "".to_owned();
    if bc.contains_key("41") {
        incidents_flood_fire = extract_with_sibling(document, &bc["41"]);
    }
    let mut ownership_changes_status: String = "".to_owned();
    if bc.contains_key("5") {
        ownership_changes_status = extract_with_sibling(document, &bc["5"]);
    }
    let mut ownership_changes_value: String = "".to_owned();
    if bc.contains_key("51") {
        ownership_changes_value = extract_with_sibling(document, &bc["51"]);
    }
    let mut color_changes_chrome: String = "".to_owned();
    if bc.contains_key("61") {
        color_changes_chrome = extract_with_sibling(document, &bc["61"]);
    }
    let mut color_changes_type: String = "".to_owned();
    if bc.contains_key("62") {
        color_changes_type = extract_with_sibling(document, &bc["62"]);
    }
    let mut options_status: String = "".to_owned();
    if bc.contains_key("7") {
        options_status = extract_with_sibling(document, &bc["7"]);
    }
    let mut options_list: String = "".to_owned();
    if bc.contains_key("71") {
        options_list = extract_with_sibling(document, &bc["71"]);
    }
    let mut feedback_status: String = "".to_owned();
    if bc.contains_key("81") {
        feedback_status = extract_with_sibling(document, &bc["81"]);
    }
    let mut feedback_value: String = "".to_owned();
    if bc.contains_key("82") {
        feedback_value = extract_with_sibling(document, &bc["82"]);
    }
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
    let s: HashMap<String, String> = out_s.into_iter().chain(bones_s).collect();
    let mut scheme_unconverted: Vec<CarSecListPointSchemeItem> = vec![];
    for (key, value) in s {
        let s = key
            .chars()
            .filter(|char| char.is_digit(10))
            .collect::<String>();
        scheme_unconverted.push(CarSecListPointSchemeItem {
            index: s.parse::<u8>().unwrap(),
            mark: value.cut_off(),
            title: key.replace(&(s + "."), ""),
        })
    }
    //////////// 1 Самодиагностика
    // двигатель
    let mut diagnostics_engine: String = "".to_owned();
    if dc.contains_key("11") {
        diagnostics_engine = extract_with_sibling(document, &dc["11"]);
    }
    // Коробка передач
    let mut diagnostics_transmission: String = "".to_owned();
    if dc.contains_key("12") {
        diagnostics_transmission = extract_with_sibling(document, &dc["12"]);
    }
    /////////// 2 Двигатель
    // холостой ход
    let mut engine_idling: String = "".to_owned();
    if dc.contains_key("21") {
        engine_idling = extract_with_sibling(document, &dc["21"]);
    }
    ///////// Утечка масла
    // Клапанная крышка
    let mut engine_oil_leak_valve_cover: String = "".to_owned();
    if dc.contains_key("221") {
        engine_oil_leak_valve_cover = extract_with_sibling(document, &dc["221"]);
    }
    // Прокладка ГБЦ
    let mut engine_oil_leak_cylynder_head_gasket: String = "".to_owned();
    if dc.contains_key("222") {
        engine_oil_leak_cylynder_head_gasket = extract_with_sibling(document, &dc["222"]);
    }
    // Поддон
    let mut engine_oil_leak_pan: String = "".to_owned();
    if dc.contains_key("223") {
        engine_oil_leak_pan = extract_with_sibling(document, &dc["223"]);
    }
    //
    // Давление масла
    //
    let mut engine_oil_pressure: String = "".to_owned();
    if dc.contains_key("23") {
        engine_oil_pressure = extract_with_sibling(document, &dc["23"]);
    }
    ////// Охлаждающая жидкость Утечки
    // ГБЦ
    let mut engine_coolant_leak_cylynder_head: String = "".to_owned();
    if dc.contains_key("231") {
        engine_coolant_leak_cylynder_head = extract_with_sibling(document, &dc["231"]);
    }
    // Помпа
    let mut engine_coolant_leak_pump: String = "".to_owned();
    if dc.contains_key("232") {
        engine_coolant_leak_pump = extract_with_sibling(document, &dc["232"]);
    }
    // Радиатор
    let mut engine_coolant_leak_radiator: String = "".to_owned();
    if dc.contains_key("233") {
        engine_coolant_leak_radiator = extract_with_sibling(document, &dc["233"]);
    }
    // Количество охлаждающей жидкости
    let mut engine_coolant_leak_coolant_amount: String = "".to_owned();
    if dc.contains_key("234") {
        engine_coolant_leak_coolant_amount = extract_with_sibling(document, &dc["234"]);
    }
    //
    // Общая магистраль
    let mut engine_line: String = "".to_owned();
    if dc.contains_key("24") {
        engine_line = extract_with_sibling(document, &dc["24"]);
    }
    //////// Коробка передач
    //// АКПП
    // Утечка масла Масло
    let mut at_oil_leak: String = "".to_owned();
    if dc.contains_key("311") {
        at_oil_leak = extract_with_sibling(document, &dc["311"]);
    }
    // Расход и состояние масла
    let mut at_oil_consumption: String = "".to_owned();
    if dc.contains_key("312") {
        at_oil_consumption = extract_with_sibling(document, &dc["312"]);
    }
    // Рабочее состояние (холостой ход)
    let mut at_idling: String = "".to_owned();
    if dc.contains_key("313") {
        at_idling = extract_with_sibling(document, &dc["313"]);
    }
    //// МКПП
    // Утечка масла Масло
    let mut tm_oil_leak: String = "".to_owned();
    if dc.contains_key("321") {
        tm_oil_leak = extract_with_sibling(document, &dc["321"]);
    }
    // Переключение передач
    let mut tm_gear_shift: String = "".to_owned();
    if dc.contains_key("322") {
        tm_gear_shift = extract_with_sibling(document, &dc["322"]);
    }
    // Расход и состояние масла
    let mut tm_oil_consumption: String = "".to_owned();
    if dc.contains_key("323") {
        tm_oil_consumption = extract_with_sibling(document, &dc["323"]);
    }
    // Рабочее состояние (холостой ход)
    let mut tm_idling: String = "".to_owned();
    if dc.contains_key("324") {
        tm_idling = extract_with_sibling(document, &dc["324"]);
    }
    ///// 4 Ходовая часть
    // Сцепление в сборе
    let mut clutch_assembly: String = "".to_owned();
    if dc.contains_key("41") {
        clutch_assembly = extract_with_sibling(document, &dc["41"]);
    }
    // шрус
    let mut joints: String = "".to_owned();
    if dc.contains_key("42") {
        joints = extract_with_sibling(document, &dc["42"]);
    }
    // Карданный вал
    let mut driveshaft: String = "".to_owned();
    if dc.contains_key("43") {
        driveshaft = extract_with_sibling(document, &dc["43"]);
    }
    // Дифференциал
    let mut differential: String = "".to_owned();
    if dc.contains_key("44") {
        differential = extract_with_sibling(document, &dc["44"]);
    }
    /////////////// 5 Рулевое управление
    // Утечка масла при работе гидроусилителя рулевого управления
    let mut power_steering: String = "".to_owned();
    if dc.contains_key("51") {
        power_steering = extract_with_sibling(document, &dc["51"]);
    }
    //// Рабочее состояние
    // Насос рулевого управления
    let mut steering_pump: String = "".to_owned();
    if dc.contains_key("522") {
        steering_pump = extract_with_sibling(document, &dc["522"]);
    }
    // Рулевой механизм с MDPS
    let mut steering_gear: String = "".to_owned();
    if dc.contains_key("521") {
        steering_gear = extract_with_sibling(document, &dc["521"]);
    }
    // Шарнир рулевого управления
    let mut steering_propshaft: String = "".to_owned();
    if dc.contains_key("524") {
        steering_propshaft = extract_with_sibling(document, &dc["524"]);
    }
    // Силовой шланг высокого давления
    let mut steering_hoses_n_tubes: String = "".to_owned();
    if dc.contains_key("525") {
        steering_hoses_n_tubes = extract_with_sibling(document, &dc["525"]);
    }
    // Наконечник рулевой тяги и шаровой шарнир
    let mut steering_rack: String = "".to_owned();
    if dc.contains_key("523") {
        steering_rack = extract_with_sibling(document, &dc["523"]);
    }
    ///////////////// 6 Тормозная система
    // Утечка масла из Главного тормозного цилиндра
    let mut main_brake_cylinder_leak: String = "".to_owned();
    if dc.contains_key("61") {
        main_brake_cylinder_leak = extract_with_sibling(document, &dc["61"]);
    }
    // Утечка тормозного масла
    let mut brake_fluid_leak: String = "".to_owned();
    if dc.contains_key("62") {
        brake_fluid_leak = extract_with_sibling(document, &dc["62"]);
    }
    // Состояние источника питания
    let mut brake_fluid_sensor: String = "".to_owned();
    if dc.contains_key("63") {
        brake_fluid_sensor = extract_with_sibling(document, &dc["63"]);
    }
    ////////////// 7 Электричество
    // Выход генератора
    let mut electrics_generator: String = "".to_owned();
    if dc.contains_key("71") {
        electrics_generator = extract_with_sibling(document, &dc["71"]);
    }
    // Пусковой двигатель
    let mut electrics_starter: String = "".to_owned();
    if dc.contains_key("72") {
        electrics_starter = extract_with_sibling(document, &dc["72"]);
    }
    // Функция двигателя стеклоочистителя
    let mut electrics_wipers_electrics: String = "".to_owned();
    if dc.contains_key("73") {
        electrics_wipers_electrics = extract_with_sibling(document, &dc["73"]);
    }
    // Двигатель для вентиляции помещений
    let mut electrics_engine_fan: String = "".to_owned();
    if dc.contains_key("74") {
        electrics_engine_fan = extract_with_sibling(document, &dc["74"]);
    }
    // Двигатель вентилятора радиатора
    let mut electrics_engine_fan_motor: String = "".to_owned();
    if dc.contains_key("75") {
        electrics_engine_fan_motor = extract_with_sibling(document, &dc["75"]);
    }
    // Привод стеклоподъемника
    let mut electrics_window_lifter_drive: String = "".to_owned();
    if dc.contains_key("76") {
        electrics_window_lifter_drive = extract_with_sibling(document, &dc["76"]);
    }
    //////////////// 8 Электропроводка
    // Состояние изоляции зарядного порта
    let mut charger_insulation: String = "".to_owned();
    if dc.contains_key("91") {
        charger_insulation = extract_with_sibling(document, &dc["91"]);
    }
    // Состояние изоляции аккумуляторной батареи привода
    let mut battery_insulation: String = "".to_owned();
    if dc.contains_key("92") {
        battery_insulation = extract_with_sibling(document, &dc["92"]);
    }
    // Состояние электропроводки высокой мощности (соединительная клемма, ткань, защитный механизм)
    let mut high_power_wiring: String = "".to_owned();
    if dc.contains_key("93") {
        high_power_wiring = extract_with_sibling(document, &dc["93"]);
    }
    /////////// 9 Топливо
    // Утечка топлива (включая сжиженный газ)
    let mut fuel_n_gas_leak: String = "".to_owned();
    if dc.contains_key("81") {
        fuel_n_gas_leak = extract_with_sibling(document, &dc["81"]);
    }

    //////////// Table 5
    ///////   Требуется ремонт
    // Внешний вид
    let mut appearance: String = "".to_owned();
    if eac.contains_key("1") {
        appearance = extract_with_sibling(document, &eac["1"]);
    }
    // Встроенный
    let mut interior: String = "".to_owned();
    if eac.contains_key("2") {
        interior = extract_with_sibling(document, &eac["2"]);
    }
    // Блеск
    let mut gloss: String = "".to_owned();
    if eac.contains_key("3") {
        gloss = extract_with_sibling(document, &eac["3"]);
    }
    // клининг
    let mut cleaning: String = "".to_owned();
    if eac.contains_key("4") {
        cleaning = extract_with_sibling(document, &eac["4"]);
    }
    // Колеса
    // status
    let mut wheels_status: String = "".to_owned();
    if eac.contains_key("5") {
        wheels_status = extract_with_sibling(document, &eac["5"]);
    }
    // 1
    let mut wheels1: String = "".to_owned();
    if eac.contains_key("51") {
        wheels1 = extract_with_sibling(document, &eac["51"]);
    }
    // 2
    let mut wheels2: String = "".to_owned();
    if eac.contains_key("52") {
        wheels2 = extract_with_sibling(document, &eac["52"]);
    }
    // 3
    let mut wheels3: String = "".to_owned();
    if eac.contains_key("53") {
        wheels3 = extract_with_sibling(document, &eac["53"]);
    }
    // 4
    let mut wheels4: String = "".to_owned();
    if eac.contains_key("54") {
        wheels4 = extract_with_sibling(document, &eac["54"]);
    }
    // ЧС
    let mut wheels5: String = "".to_owned();
    if eac.contains_key("55") {
        wheels5 = extract_with_sibling(document, &eac["55"]);
    }
    // Шины
    let mut tires_status: String = "".to_owned();
    if eac.contains_key("6") {
        tires_status = extract_with_sibling(document, &eac["6"]);
    }
    // 1
    let mut tires1: String = "".to_owned();
    if eac.contains_key("61") {
        tires1 = extract_with_sibling(document, &eac["61"]);
    }
    // 2
    let mut tires2: String = "".to_owned();
    if eac.contains_key("62") {
        tires2 = extract_with_sibling(document, &eac["62"]);
    }
    // 3
    let mut tires3: String = "".to_owned();
    if eac.contains_key("63") {
        tires3 = extract_with_sibling(document, &eac["63"]);
    }
    // 4
    let mut tires4: String = "".to_owned();
    if eac.contains_key("64") {
        tires4 = extract_with_sibling(document, &eac["64"]);
    }
    // чc
    let mut tires5: String = "".to_owned();
    if eac.contains_key("65") {
        tires5 = extract_with_sibling(document, &eac["65"]);
    }
    // Стекло
    let mut window: String = "".to_owned();
    if eac.contains_key("7") {
        window = extract_with_sibling(document, &eac["7"]);
    }
    // Основные предметы
    // Статус удержания
    // нет есть
    let mut additional_items: String = "".to_owned();
    if eac.contains_key("8") {
        additional_items = extract_with_sibling(document, &eac["8"]);
    }
    // Руководство по эксплуатации, джек
    let mut user_manual: String = "".to_owned();
    if eac.contains_key("83") {
        user_manual = extract_with_sibling(document, &eac["83"]);
    }
    // Защитный штатив
    let mut emergency_stop_sign: String = "".to_owned();
    if eac.contains_key("84") {
        emergency_stop_sign = extract_with_sibling(document, &eac["84"]);
    }
    // Особенности и мнения инспекторов
    let performance_n_health_inspector = extract_value(
        document,
        "div.page_col2 > div.page_line > table.fuc_normal > tbody > :nth-child(1) > td",
    );
    // цена и обзор
    let price_survey = extract_value(
        document,
        "div.page_col2 > div.page_line > table.fuc_normal > tbody > :nth-child(2) > td",
    );
    // фотографии
    let images = extract_attrs(document, "src", "table.height_set3 > tbody > tr > td > img")?;

    Ok(CarSecList {
        seclist_num,
        name,
        ext_name: "".to_owned(),
        license_plate,
        release_year,
        validity_period,
        first_reg_date,
        chassis_number,
        transmission_types,
        fuel_type,
        engine,
        warranty_type,
        odometr_status,
        mileage_status,
        mileage_value,
        vin_plate_status,
        emission_names,
        emission_values,
        tuning_status,
        tuning_legality,
        tuning_type,
        incidents,
        incidents_flood_fire,
        ownership_changes_status,
        ownership_changes_value,
        color_changes_chrome,
        color_changes_type,
        options_status,
        options_list: vec![options_list],
        feedback_status,
        feedback_value,
        point_scheme: convert(scheme_unconverted),
        extended_table: CarSecListExtendedTable {
            diagnostics: CarSecListExtendedTableDiagnostics {
                diagnostics_engine,
                diagnostics_transmission,
            },
            engine: CarSecListExtendedTableEngine {
                engine_idling,
                engine_oil_leak: CarSecListExtendedTableEngineOilLeak {
                    engine_oil_leak_valve_cover,
                    engine_oil_leak_cylynder_head_gasket,
                    engine_oil_leak_pan,
                },
                engine_oil_pressure,
                engine_coolant_leak: CarSecListExtendedTableEngineCoolantLeak {
                    engine_coolant_leak_cylynder_head,
                    engine_coolant_leak_pump,
                    engine_coolant_leak_radiator,
                    engine_coolant_leak_coolant_amount,
                },
                engine_line,
            },
            transmission: CarSecListExtendedTableTransmission {
                automatic: CarSecListExtendedTableTransmissionAutomatic {
                    at_oil_leak,
                    at_oil_consumption,
                    at_idling,
                },
                manual: CarSecListExtendedTableTransmissionManual {
                    tm_oil_leak,
                    tm_gear_shift,
                    tm_oil_consumption,
                    tm_idling,
                },
            },
            chassis: CarSecListExtendedTableChassis {
                clutch_assembly,
                joints,
                driveshaft,
                differential,
            },
            steering: CarSecListExtendedSteering {
                power_steering,
                condition: CarSecListExtendedSteeringCondition {
                    steering_pump,
                    steering_gear,
                    steering_propshaft,
                    steering_hoses_n_tubes,
                    steering_rack,
                },
            },
            brakes: CarSecListExtendedBrakes {
                main_brake_cylinder_leak,
                brake_fluid_leak,
                brake_fluid_sensor,
            },
            electics: CarSecListExtendedElectrics {
                electrics_generator,
                electrics_starter,
                electrics_wipers_electrics,
                electrics_engine_fan,
                electrics_engine_fan_motor,
                electrics_window_lifter_drive,
            },
            wiring: CarSecListExtendedWiring {
                charger_insulation,
                battery_insulation,
                high_power_wiring,
            },
            fuel: CarSecListExtendedFuel { fuel_n_gas_leak },
        },
        description_table: CarSecListDescription {
            repair_required: CarSecListDescriptionRepair {
                appearance,
                interior,
                gloss,
                cleaning,
                wheels_status,
                wheels: vec![wheels1, wheels2, wheels3, wheels4, wheels5],
                tires_status,
                tires: vec![tires1, tires2, tires3, tires4, tires5],
                window,
                additional_items,
                user_manual,
                emergency_stop_sign,
            },
        },
        opinions_table: CarSecListOpinions {
            performance_n_health_inspector,
            price_survey,
        },
        images,
    })
}
