use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CarSecList {
    // Table 1
    seclist_num: String,
    name: String,
    ext_name: String,
    license_plate: String,
    release_year: String,
    validity_period: String,
    first_reg_date: String,
    chassis_number: String,
    transmission_types: Vec<String>,
    fuel_type: Vec<String>,
    engine: String,
    warranty_type: String,
    // Table 2
    odometr_status: String,
    mileage_status: String,
    mileage_value: String,
    vin_plate_status: String,
    emission_names: Vec<String>,
    emission_values: String,
    tuning_status: String,
    tuning_legality: String,
    // structure or device
    tuning_type: Vec<String>,
    incidents: String,
    incidents_flood_fire: Vec<String>,
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
}
