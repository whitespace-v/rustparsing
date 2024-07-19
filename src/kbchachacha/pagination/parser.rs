use crate::{
    extend::Cutter,
    http,
    kbchachacha::structs::{Car, CarData, CarDataSeclist},
};
use scraper::{ElementRef, Html};
use std::{error::Error, str, sync::Mutex, thread};

pub fn parse(cars: Vec<Car>) -> Result<Vec<CarData>, Box<dyn Error>> {
    let agent = http::builder::build_ureq_client()?;

    let mutex_data_list: Mutex<Vec<CarData>> = Mutex::new(vec![]);

    thread::scope(|scope| {
        for chunk in cars.chunks(20) {
            for car in chunk {
                scope.spawn(|| {
                    let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24631894"
                        .to_owned();
                    match agent.get(&url).call() {
                        Ok(response) => {
                            let mut u_mutex_data_list = mutex_data_list.lock().unwrap();
                            let html = response.into_string().expect("couldn't parse string");
                            let document = &scraper::Html::parse_document(&html);
                            let data = parse_car_page(document);

                            let car_data = CarData {
                                title: String::from("sds"),
                                maker_code: car.maker_code.to_string(),
                                class_code: car.class_code.to_string(),
                                seclist: CarDataSeclist { url: "".to_owned() },
                            };

                            // if !data.seclist.url.is_empty() {
                            // http://checkpaper.iwsp.co.kr -> ok
                            // http://autocafe.co.kr/ASSO/CarCheck_Form.asp?OnCarNo=2024300226007 -> not found
                            //&data.seclist.url
                            let url = "http://checkpaper.iwsp.co.kr/Service/JohabCheckPaper?code=KB&checkNo=0213059102".to_owned();
                            let s = parse_sec_list(&url);
                            // }
                            // // extract data

                            u_mutex_data_list.push(car_data)
                        }
                        Err(e) => {
                            eprintln!("{e:#?}")
                        }
                    }
                });
            }
        }
    });
    Ok(mutex_data_list.into_inner().unwrap())
}

fn parse_sec_list(url: &String) -> Result<(), Box<dyn Error>> {
    let client = http::builder::build_reqwest_client()?;
    match client.get(url).send() {
        Ok(response) => {
            let html = response.text().expect("couldn't parse string");
            let document = &scraper::Html::parse_document(&html);
            let title = extract_value(document, "div.docu_title");
            // 1. название; 2. ГРЗ; 3. год выпуска; 4. срок действия тех.отчета;
            // 5. дата первой регистрации 7.идентификационный номер авто; 10. мотор
            let table1 = extract_values(document, "table.ins_tbl1 > tbody > tr > td");
            // 7. КПП
            let table1_1 = with_checked(
                document,
                "table.ins_tbl1 > tbody > tr:nth-child(3) > td > ul.chkbox_list > li",
            );
            // топливо
            let table1_2 = with_checked(
                document,
                "table.ins_tbl1 > tbody > tr:nth-child(5) > td > ul.chkbox_list > li",
            );
            // тип гарантии
            let table1_3 = with_checked(
                document,
                "table.ins_tbl1 > tbody > tr:nth-child(6) > td > ul.chkbox_list > li",
            );
            println!("\n[Seclist]:\ntitle:{title}\n[table1]:\n{table1:#?}\nTransmission: {table1_1:?}\ngas: {table1_2:?},\nWarranty: {table1_3:?}");
            // состояние Пробега
            let table2_1 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(3) > td > ul.chkbox_list > li",
            );
            // общее состояние авто
            let table2_2 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(4) > td > ul.chkbox_list > li ",
            );
            // пробег в км
            let table2_3 = extract_value(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(2) > :nth-child(3)",
            );
            // состояние шильдика с вином
            let table2_4 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(4) > td > ul.chkbox_list > li",
            );
            // выбросы
            let table2_5 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(5) > td > ul.chkbox_list > li",
            );
            // показатели выбросов
            let table2_5_1 = extract_values(document, "td.exhaust_gas");

            // тюнинг/модификации
            let table2_6 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(6) > :nth-child(2) > ul.chkbox_list > li",
            );
            // законность тюнинга если есть
            let table2_6_1 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(6) > :nth-child(3) > ul.chkbox_list > li",
            );
            // хуй знает что в тюнинге если есть
            let table2_6_2 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(6) > :nth-child(4) > ul.chkbox_list > li",
            );
            // особая история
            let table2_7 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(7) > :nth-child(2) > ul.chkbox_list > li",
            );
            // наводнение/огонь
            let table2_7_1 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(7) > :nth-child(3) > ul.chkbox_list > li",
            );
            // Изменение способа использования ??
            let table2_8 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(8) > :nth-child(2) > ul.chkbox_list > li",
            );
            // аренда/ продажа
            let table2_8_1 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(8) > :nth-child(3) > ul.chkbox_list > li",
            );
            // Цвет (а)/хроматический
            let table2_9 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(9) > :nth-child(2) > ul.chkbox_list > li",
            );
            // полная покраска // изменение цвета
            let table2_9_1 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(9) > :nth-child(3) > ul.chkbox_list > li",
            );
            // Основные опции
            let table2_10 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(10) > :nth-child(2) > ul.chkbox_list > li",
            );
            // люк на крыше/навигация/ другое
            let table2_10_1 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(10) > :nth-child(3) > ul.chkbox_list > li",
            );
            // подлежит отзыву: неприменимо/ применимо
            let table2_11 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(11) > :nth-child(2) > ul.chkbox_list > li",
            );
            // реализация отзыва: было/ не было
            let table2_11_1 = with_checked(
                document,
                "table.ins_tbl2 > tbody > tr:nth-child(12) > :nth-child(4) > ul.chkbox_list > li",
            );
            println!("\nprobeg:{table2_1:?}\nsostoyanie: {table2_2:?}\nprobeg: {table2_3:?}\nshield: {table2_4:?}\nvibrosy: {table2_5:?} {table2_5_1:?}
            \ntuning: {table2_6:?} {table2_6_1:?} {table2_6_2:?}\nadvanced exp: {table2_7:?} {table2_7_1:?}\ntable2_8: {table2_8:?}\ntable2_8_1: {table2_8_1:?}\ntable2_9: {table2_9:?}\ntable2_9_1: {table2_9_1:?}\ntable2_10: {table2_10:?}\ntable2_10_1: {table2_10_1:?}\ntable2_11: {table2_11:?}\ntable2_11_1: {table2_11_1:?}
            ")
        }
        Err(e) => {
            eprintln!("{e:#?}")
        }
    }
    Ok(())
}

fn parse_car_page(document: &Html) -> CarData {
    // car:
    let car_price = extract_value(document, "dd > strong");
    let car_name = extract_value(document, "strong.car-buy-name");
    let imgs = extract_attrs(
        document,
        "src".to_owned(),
        "div.page01 > a > img".to_owned(),
    );
    println!("\n[Car]:\nname:{car_name}, \nprice: {car_price}, \nimg: {imgs:?}");

    let detail01 = extract_values(document, "table.detail-info-table > tbody > tr > td");
    // ГРЗ; год выпуска; пробег; топливо; КПП; эффективность топлива ?; тип кузова; перемещение; цвет; неуплата налогов
    // лишение права выкупа (ограничения); ипотека ??; номер лота
    println!("\n[detail01]:\n{detail01:?}");

    let detail02 = extract_values(document, "div.detail-info02 > div.mg-t40 > dl > dd");
    // общая история потерь, наводнения, история использования, смена владельца
    println!("\n[detail02]:\n{detail02:?}");
    // Дата запроса отчета о страховых случаях
    let detail03 = extract_value(document, "div.detail-info02 > div.mg-t40 > span");
    println!("\n[detail03]:\n{detail03:?}");
    // boss info: https://www.kbchachacha.com/public/layer/shop/info.kbc
    // dealer:
    let dealer_name = extract_value(document, "div.dealer-cnt > span.name");
    let dealer_place = extract_value(document, "span.place-add");
    let dealer_tel = extract_value(document, "div.dealer-tel-num");
    let dealer_location = extract_value(document, "div.map-txt");
    let dealer_info = extract_value(document, "div.dealer-scroll");
    let dealer_sell = extract_value(document, "span[id=btnDealerHome3]");
    let dealer_sold = extract_value(document, "span[id=btnDealerHome4]");
    println!("\n[Dealer]:\nname: {dealer_name},\ntel: {dealer_tel}\nplace: {dealer_place},\nlocation: {dealer_location},\nselling: {dealer_sell}, \nsold: {dealer_sold},\ninfo: {dealer_info}");

    let sec_list = extract_attr(
        document,
        "data-link-url".to_owned(),
        "a[id=btnCarCheckView1]",
    );
    println!("\n [Seclist]:\n{sec_list}");

    CarData {
        title: "".to_owned(),
        maker_code: "".to_owned(),
        class_code: "".to_owned(),
        seclist: CarDataSeclist {
            url: sec_list.to_owned(),
        },
    }
}
fn extract_attrs(document: &Html, attr: String, selector_str: String) -> Vec<&str> {
    let mut res: Vec<&str> = vec![];
    for e in document.select(&scraper::Selector::parse(&selector_str).unwrap()) {
        res.push(e.value().attr(&attr).unwrap())
    }
    res
}
fn extract_attr(document: &Html, attr: String, selector_str: &str) -> String {
    document
        .select(&scraper::Selector::parse(&selector_str).unwrap())
        .next()
        .map(|e| e.value().attr(&attr))
        .unwrap()
        .unwrap()
        .cut_off()
}
fn extract_value(document: &Html, selector_str: &str) -> String {
    document
        .select(&scraper::Selector::parse(&selector_str).unwrap())
        .next()
        .map(|e| e.text().collect::<String>())
        .unwrap()
        .cut_off()
}
fn extract_values(document: &Html, selector_str: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for e in document.select(&scraper::Selector::parse(&selector_str).unwrap()) {
        res.push(e.text().collect::<String>().trim().cut_off());
    }
    res
}
fn with_checked(document: &Html, selector_str: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for parent in document.select(&scraper::Selector::parse(&selector_str).unwrap()) {
        for child in parent
            .children()
            .filter_map(|child| ElementRef::wrap(child))
        {
            match child.value().attr("checked") {
                Some(_) => {
                    let bbb = parent
                        .children()
                        .filter_map(|child| ElementRef::wrap(child))
                        .flat_map(|el| el.text())
                        .collect::<String>();
                    res.push(bbb)
                }
                _ => (),
            }
        }
    }
    res
}
