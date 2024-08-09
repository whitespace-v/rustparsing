use super::structs::{
    CarSecListPointScheme, CarSecListPointSchemeBones, CarSecListPointSchemeItem,
    CarSecListPointSchemeOut,
};

pub fn convert(scheme_unconverted: Vec<CarSecListPointSchemeItem>) -> CarSecListPointScheme {
    let mut first: Vec<CarSecListPointSchemeItem> = vec![];
    let mut second: Vec<CarSecListPointSchemeItem> = vec![];
    let mut a: Vec<CarSecListPointSchemeItem> = vec![];
    let mut b: Vec<CarSecListPointSchemeItem> = vec![];
    let mut c: Vec<CarSecListPointSchemeItem> = vec![];
    for i in scheme_unconverted {
        match i.index {
            1 | 2 | 3 | 4 | 5 => first.push(i),
            6 | 7 | 8 => second.push(i),
            9 | 10 | 11 | 17 | 18 => a.push(i),
            12 | 13 | 14 | 19 => b.push(i),
            15 | 16 => c.push(i),
            _ => {}
        }
    }
    CarSecListPointScheme {
        out: CarSecListPointSchemeOut { first, second },
        bones: CarSecListPointSchemeBones { a, b, c },
    }
}

//// X - Замена детали
//// W - Листовой металл или сварка
//// A - Царапины
//// U - Неровности
//// C - Коррозия
//// T - Ущерб
//// внешка
//// 1 класс - 1,2,3,4,5
//// 2 класс - 6,7,8
//// скелет
//// A класс - 9,10,11,17,18
//// B класс - 12,13,14,19
//// C класс - 15,16
