use crate::{
    extractor::extract::{
        extract_attr, extract_attrs, extract_from_js, extract_value, extract_values,
    },
    http,
    kbchachacha::{
        pagination::{popup, seclist},
        structs::{Car, CarData, CarDataParams, CarDataSeclist},
    },
};
use scraper::Html;
use std::{error::Error, sync::Mutex, thread};
use url::Url;

pub fn parse(cars: Vec<Car>) -> Result<Vec<CarData>, Box<dyn Error>> {
    let mutex_data_list: Mutex<Vec<CarData>> = Mutex::new(vec![]);
    thread::scope(|scope| {
        for chunk in cars.chunks(50) {
            for car in chunk {
                scope.spawn(|| {
                    let agent = http::builder::build_ureq_client().unwrap();
                    // let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24663799".to_owned();
                    let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=24633080".to_owned();
                    match agent.get(&url).call() {
                        Ok(response) => {
                            let mut u_mutex_data_list = mutex_data_list.lock().unwrap();
                            let html = response.into_string().expect("couldn't parse string");
                            let document = &scraper::Html::parse_document(&html);
                            let data = parse_car_page(document);
                            // parse options
                            ////// pass params ->
                            let options = popup::parse_options::parse();
                            let agent = ureq::AgentBuilder::new()
                                    .user_agent("Mozilla/5.0 (Windows NT 6.0; rv:14.0) Gecko/20100101 Firefox/14.0.1")
                                    .build();
                            if data.seclist.url.is_empty(){
                                let s = popup::parse_seclist::parse();
                            } else if data.seclist.url == "www.autocafe.co.kr" {
                                    //e.g https://www.kbchachacha.com/public/car/detail.kbc?carSeq=23220785
                                    println!("Nothing to parse....")
                            } else {
                                // format it
                                let resp: String = ureq::post("https://www.kbchachacha.com/public/layer/car/check/info.kbc")
                                .send_form(&[
                                    ("layerId", "layerCarCheckInfo"),
                                    ("carSeq", "24663799"),
                                    ("diagCarYn", "N"),
                                    ("diagCarSeq", ""),
                                    ("premiumCarYn", "N"),
                                  ])
                                .unwrap()
                                .into_string()
                                .unwrap();
                                let document = &scraper::Html::parse_document(&resp);
                                // images
                                let t = extract_attrs(document, "src", "div.ch-img > img");
                                match agent.get(&data.seclist.url).call() {
                                    Ok(sec_response) => {
                                        let res_data: [String;2] = [sec_response.get_url().to_owned(), sec_response.into_string().expect("couldn't parse string")];
                                        let document = &scraper::Html::parse_document(&res_data[1]);
                                        match Url::parse(&res_data[0]).unwrap().domain() {
                                            Some(domain) => {
                                                match domain {
                                                    // done
                                                    "checkpaper.iwsp.co.kr" => {
                                                        println!("Parsing checkpaper...");
                                                        let s = seclist::parse_checkpaper::parse(document);
                                                    }
                                                    // done
                                                    "checkpaper.jmenetworks.co.kr" => {
                                                        println!("Parsing jmenetworks...");
                                                        let s = seclist::parse_jmenetworks::parse(document);
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
                                                    // done
                                                    "www.autocafe.co.kr" => {
                                                        println!("Parsing autocafe");
                                                        let s = seclist::parse_autocafe::parse(document);
                                                    }
                                                    // done
                                                    "ai.carinfo.co.kr" => {
                                                        println!("Parsing carinfo");
                                                        let s = seclist::parse_carinfo::parse(document);
                                                    }
                                                    // done
                                                    "ai2.kaai.or.kr" => {
                                                        println!("Parsing ai2.kaai.or.kr");
                                                        let s = seclist::parse_ai2kaai::parse(document);
                                                    }
                                                    _ => println!("! Seclist source is never known: {}", domain) 
                                                }
                                            },
                                            None => {
                                                match Url::parse(&res_data[0]).unwrap().host().unwrap() {
                                                    host => {
                                                        match host.to_string().as_str() {
                                                            "221.143.49.206" => {
                                                                println!("Parsing 221.143.49.206");
                                                                let s = seclist::parse_221::parse(document);
                                                            },
                                                            _ => println!("")
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => eprintln!("{e:#?}"),
                                }
                            }
                            let car_data = CarData {
                                title: String::from("sds"),
                                maker_code: car.maker_code.to_string(),
                                class_code: car.class_code.to_string(),
                                seclist: CarDataSeclist { url: "".to_owned() },
                            };
                            u_mutex_data_list.push(car_data)
                        }
                        Err(e) => eprintln!("{e:#?}")
                    }
                });
            }
        }
    });
    Ok(mutex_data_list.into_inner().unwrap())
}

fn parse_car_page(document: &Html) -> CarData {
    let car_price = extract_value(document, "dd > strong");
    let car_name = extract_value(document, "strong.car-buy-name");
    let imgs = extract_attrs(document, "src", "div.page01 > a > img");
    // ГРЗ; год выпуска; пробег; топливо; КПП; эффективность топлива ?; тип кузова; перемещение; цвет; неуплата налогов
    // лишение права выкупа (ограничения); ипотека ??; номер лота
    let detail01 = extract_values(document, "table.detail-info-table > tbody > tr > td");
    // общая история потерь, наводнения, история использования, смена владельца
    let detail02 = extract_values(document, "div.detail-info02 > div.mg-t40 > dl > dd");
    // Дата запроса отчета о страховых случаях
    let detail03 = extract_value(document, "div.detail-info02 > div.mg-t40 > span");
    // boss info: https://www.kbchachacha.com/public/layer/shop/info.kbc
    // dealer:
    let dealer_name = extract_value(document, "div.dealer-cnt > span.name");
    let dealer_place = extract_value(document, "span.place-add");
    let dealer_tel = extract_value(document, "div.dealer-tel-num");
    let dealer_location = extract_value(document, "div.map-txt");
    let dealer_info = extract_value(document, "div.dealer-scroll");
    let dealer_selling = extract_value(document, "span[id=btnDealerHome3]");
    let dealer_sold = extract_value(document, "span[id=btnDealerHome4]");

    let sec_list = extract_attr(document, "data-link-url", "a[id=btnCarCheckView1]");

    let param_diag_car_yn = extract_from_js(
        document,
        "div.wrap > div.container > div[id=content] > :nth-child(28)",
        "var diagCarYn = ",
        ";",
    );
    let param_diag_car_seq = extract_from_js(
        document,
        "div.wrap > div.container > div[id=content] > :nth-child(28)",
        "var diagCarSeq = ",
        ";",
    );
    let param_premium_car_yn = extract_from_js(
        document,
        "div.wrap > div.container > div[id=content] > :nth-child(28)",
        "var premiumCarYn = ",
        ";",
    );
    CarData {
        title: "".to_owned(),
        maker_code: "".to_owned(),
        class_code: "".to_owned(),
        seclist: CarDataSeclist {
            url: sec_list.to_owned(),
        },
        params: CarDataParams {
            param_diag_car_yn,
            param_diag_car_seq,
            param_premium_car_yn,
        },
    }
}
