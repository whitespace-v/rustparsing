use crate::{
    extractor::extract::{
        extract_attr, extract_attrs, extract_value, extract_values, with_checked,
    },
    http,
    kbchachacha::{
        pagination::seclist,
        structs::{Car, CarData, CarDataSeclist},
    },
};
use hyper::client;
use scraper::Html;
use std::{collections::HashMap, error::Error, sync::Mutex, thread};
use url::Url;

pub fn parse(cars: Vec<Car>) -> Result<Vec<CarData>, Box<dyn Error>> {
    let agent = http::builder::build_ureq_client()?;

    let mutex_data_list: Mutex<Vec<CarData>> = Mutex::new(vec![]);

    thread::scope(|scope| {
        for chunk in cars.chunks(20) {
            for car in chunk {
                scope.spawn(|| {
                    let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24631894"
                        .to_owned();
                    // let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25956913"
                    // .to_owned();
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

                            match agent.get(&data.seclist.url).call() {
                                Ok(sec_response) => {
                                    let res_data: [String;2] = [sec_response.get_url().to_owned(), sec_response.into_string().expect("couldn't parse string")];
                                    // let url = sec_response.get_url();  
                                    // let html = ;
                                    let document = &scraper::Html::parse_document(&res_data[1]);
                                    match Url::parse(&res_data[0]).unwrap().domain().unwrap() {
                                        "checkpaper.iwsp.co.kr" => {
                                            println!("Parsing checkpaper...");
                                            // http://checkpaper.iwsp.co.kr/Service/JohabCheckPaper?code=KB&checkNo=0213059102
                                            let s = seclist::parse_checkpaper::parse(document);
                                        }
                                        "autocafe.co.kr" => {
                                            println!("Parsing autocafe...");
                                            let s = seclist::parse_autocafe::parse(document);
                                        }
                                        "m-park.co.kr" => {
                                            println!("Parsing mpark...");
                                            let s = seclist::parse_mpark::parse(document);
                                        }
                                        "ck.carmodoo.com" => {
                                            println!("Parsing ck.carmodoo.com...");
                                            // target architecture
                                            let s = seclist::parse_carmodoo::parse(document);
                                        }
                                        _ => {
                                            {
                                                // src: http://autocafe.co.kr/ASSO/CarCheck_Form.asp?OnCarNo=2023300220771 ->
                                                // https://checkpaper.jmenetworks.co.kr/Service/CheckPaper?checkNo=3401023338
                                                // https://ck.carmodoo.com/carCheck/carmodooPrint.do?print=0&checkNum=6623017101
                                                // https://ai.carinfo.co.kr/view/carinfo?check_no=2408711155
                                            }
                                            {
                                                // static src :
                                                // https://www.m-park.co.kr/popup/performance/24061010025
                                                // https://checkpaper.iwsp.co.kr/Service/ICheckPaper?key=6516377065&iframe=0&mobile=0&type=90
                                                // http://www.djauto.co.kr/car/carViewFrameUsedCarCheck.html?checkFlag=443255
                                                // http://www.encar.com/md/sl/mdsl_regcar.do?method=inspectionViewNew&carid=36267727&wtClick_carview=015
                                                // http://moldeoncar.com/usedCar/cklist.asp?usedCarID=1301612
                                                // http://ai.kaai.or.kr/view/carview.do?car_no=180%uB2045114
                                                // http://ext.kaat.kr/office/rest/extservice/OUT4511?CHECK_NO=6730400579
                                                // https://erp.carmon.co.kr/office/rest/extservice/OUT4511?CHECK_NO=6780409082
                                                // could be http://checkpaper.jmenetworks.co.kr/Service/CheckPaper?checkNo=6003015431&print=0
                                            }
                                            {
                                                // doensn't work
                                                // http://moldeoncar.com/usedCar/cklist.asp?usedCarID=1301612
                                                // http://ai.kaai.or.kr/view/carview.do?car_no=180%uB2045114
                                                // http://ext.kaat.kr/office/rest/extservice/OUT4511?CHECK_NO=6730400579
                                            }

                                            //// popups:
                                            // able to parse:
                                            // https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24633080
                                            // images
                                            // https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24663799
                                            println!(
                                                "! seclist source is never known or data is in popup !"
                                            )
                                        }
                                    }
                                }
                                Err(e) => eprintln!("{e:#?}"),
                            }

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

fn parse_car_page(document: &Html) -> CarData {
    // car:
    let car_price = extract_value(document, "dd > strong");
    let car_name = extract_value(document, "strong.car-buy-name");
    let imgs = extract_attrs(document, "src", "div.page01 > a > img");
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

    let sec_list = extract_attr(document, "data-link-url", "a[id=btnCarCheckView1]");
    // println!("\n [Seclist]:\n{sec_list}");

    CarData {
        title: "".to_owned(),
        maker_code: "".to_owned(),
        class_code: "".to_owned(),
        seclist: CarDataSeclist {
            url: sec_list.to_owned(),
        },
    }
}
