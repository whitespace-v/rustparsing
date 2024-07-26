use crate::{
    extend::Cutter, extractor::extract::{
        extract_attr, extract_attrs, extract_value, extract_values, with_checked,
    }, http, kbchachacha::{
        pagination::seclist,
        structs::{Car, CarData, CarDataSeclist},
    }
};
use hyper::client;
use scraper::Html;
use std::{collections::HashMap, error::Error, sync::Mutex, thread};
use url::Url;

pub fn parse(cars: Vec<Car>) -> Result<Vec<CarData>, Box<dyn Error>> {
   

    let mutex_data_list: Mutex<Vec<CarData>> = Mutex::new(vec![]);

    thread::scope(|scope| {
        for chunk in cars.chunks(50) {
            for car in chunk {
                scope.spawn(|| {
                    let agent = http::builder::build_ureq_client().unwrap();
                    // with ck.carmodoo.com
                    // let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24631894"
                    //     .to_owned();
                    // with checkpaper.iwsp.co.kr
                    // let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25956913"
                    // .to_owned();
                    // with encar
                    // let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24955004".to_owned();
                    // with checkpaper.jmenetworks.co.kr
                    // let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25941145".to_owned();
                    // with djauto
                    // let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25496599".to_owned();
                    // with m-park.co.kr
                    // let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25837384".to_owned();
                    // with ext.m-cube.co
                    // let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=26071714".to_owned();
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
                            let proxy_uri = "7PfBJU:XKhvwQghEL@46.8.193.66:1050";
                            let proxy = ureq::Proxy::new(proxy_uri).unwrap();
                            let agent = ureq::AgentBuilder::new()
                                    .user_agent("Mozilla/5.0 (Windows NT 6.0; rv:14.0) Gecko/20100101 Firefox/14.0.1")
                                    // .proxy(proxy)
                                    .build();

                            match agent.get(&data.seclist.url).call() {
                                Ok(sec_response) => {
                                    let res_data: [String;2] = [sec_response.get_url().to_owned(), sec_response.into_string().expect("couldn't parse string")];
                                    let document = &scraper::Html::parse_document(&res_data[1]);
                                    match Url::parse(&res_data[0]).unwrap().domain().unwrap() {
                                        // done
                                        "checkpaper.iwsp.co.kr" => {
                                            println!("Parsing checkpaper...");
                                            let s = seclist::parse_checkpaper::parse(document);
                                        }
                                        // done
                                        "ck.carmodoo.com" => {
                                            println!("Parsing ck.carmodoo.com...");
                                            let s = seclist::parse_carmodoo::parse(document);
                                        }
                                        // done
                                        "www.encar.com" => {
                                            println!("Parsing encar...");
                                            let s = seclist::parse_encar::parse(document);
                                        }
                                        // done
                                        "checkpaper.jmenetworks.co.kr" => {
                                            println!("Parsing jmenetworks...");
                                            let s = seclist::parse_jmenetworks::parse(document);
                                        }
                                        // done
                                        "www.djauto.co.kr" => {
                                            println!("Parsing djauto...");
                                            let s = seclist::parse_djauto::parse(document);
                                        }
                                        // done 
                                        "www.m-park.co.kr" => {
                                            println!("Parsing m-park...");
                                            let s = seclist::parse_mpark::parse(
                                                Url::parse(&res_data[0]).unwrap().path()
                                            );
                                        }
                                        // done
                                        "ext.m-cube.co" => {
                                            println!("Parsing extmcube");
                                            let s = seclist::parse_extmcube::parse(document);
                                        }
                                        _ => {    
                                            // static src :
                                            // http://moldeoncar.com/usedCar/cklist.asp?usedCarID=1301612
                                            // http://ai.kaai.or.kr/view/carview.do?car_no=180%uB2045114
                                            // https://ai.carinfo.co.kr/view/carinfo?check_no=2408711155
                                            // http://ext.kaat.kr/office/rest/extservice/OUT4511?CHECK_NO=6730400579
                                            // https://erp.carmon.co.kr/office/rest/extservice/OUT4511?CHECK_NO=6780409082
                                        
                                            //// popups:
                                            // able to parse:
                                            // https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24633080
                                            // images
                                            // https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24663799
                                            println!("! seclist source is never known or data is in popup !")
                                            // https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25986827
                                            // http://221.143.49.206/CarCheck/popupCheck.asp?ckno=2006017378

                                            // not found: 
                                            // https://www.kbchachacha.com/public/car/detail.kbc?carSeq=23220785 -> https://www.kbchachacha.com/public/car/www.autocafe.co.kr
                                            // https://www.kbchachacha.com/public/car/detail.kbc?carSeq=23469260 - here but with text
                                            //  
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
