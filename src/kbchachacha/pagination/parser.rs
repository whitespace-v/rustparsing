use crate::{
    http,
    kbchachacha::{
        pagination::{car_page, popup, seclist},
        structs::{Car, CarData, CarDataDealer, CarDataParams, CarDataSeclist, ParamSecListType},
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
                    let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq="
                        .to_owned()
                        + &car.car_seq;
                    match agent.get(&url).call() {
                        Ok(response) => {
                            let mut u_mutex_data_list = mutex_data_list.lock().unwrap();
                            let html = response.into_string().expect("couldn't parse string");
                            // println!("{html}");
                            let document = &scraper::Html::parse_document(&html);
                            let mut car_data = car_page::parse(document);
                            car_data.car_seq = car.car_seq.to_string();
                            car_data.maker_code = car.maker_code.to_string();
                            car_data.class_code = car.class_code.to_string();
                            let options = popup::parse_options::parse(&car.car_seq).unwrap();
                            println!("{car_data:#?}");
                            println!("{options:#?}");
                            let agent = ureq::AgentBuilder::new()
                                    .user_agent("Mozilla/5.0 (Windows NT 6.0; rv:14.0) Gecko/20100101 Firefox/14.0.1")
                                    .build();
                            match car_data.params.param_sec_list_type {
                                ParamSecListType::Nothing => println!("nothing to parse"),
                                ParamSecListType::SecListUrl => {
                                    println!("SecListUrl");
                                    match agent.get(&car_data.seclist.url).call() {
                                        Ok(sec_response) => {
                                            let res_data: [String; 2] = [
                                                sec_response.get_url().to_owned(),
                                                sec_response
                                                    .into_string()
                                                    .expect("couldn't parse string"),
                                            ];
                                            let document =
                                                &scraper::Html::parse_document(&res_data[1]);
                                            match Url::parse(&res_data[0]).unwrap().domain() {
                                                Some(domain) => {
                                                    match domain {
                                                        // done
                                                        "checkpaper.iwsp.co.kr" => {
                                                            println!("Parsing checkpaper...");
                                                            let s =
                                                                seclist::parse_checkpaper::parse(
                                                                    document,
                                                                );
                                                        }
                                                        // done
                                                        "checkpaper.jmenetworks.co.kr" => {
                                                            println!("Parsing jmenetworks...");
                                                            let s =
                                                                seclist::parse_jmenetworks::parse(
                                                                    document,
                                                                );
                                                        }
                                                        // done
                                                        "ck.carmodoo.com" => {
                                                            println!("Parsing ck.carmodoo.com...");
                                                            let s = seclist::parse_carmodoo::parse(
                                                                document,
                                                            );
                                                        }
                                                        // done
                                                        "www.encar.com" => {
                                                            println!("Parsing encar...");
                                                            let s = seclist::parse_encar::parse(
                                                                document,
                                                            );
                                                        }
                                                        // done
                                                        "www.djauto.co.kr" => {
                                                            println!("Parsing djauto...");
                                                            let s = seclist::parse_djauto::parse(
                                                                document,
                                                            );
                                                        }
                                                        // done
                                                        "www.m-park.co.kr" => {
                                                            println!("Parsing m-park...");
                                                            let s = seclist::parse_mpark::parse(
                                                                Url::parse(&res_data[0])
                                                                    .unwrap()
                                                                    .path(),
                                                            );
                                                        }
                                                        // done
                                                        "ext.m-cube.co" => {
                                                            println!("Parsing extmcube");
                                                            let s = seclist::parse_extmcube::parse(
                                                                document,
                                                            );
                                                        }
                                                        // done
                                                        "www.autocafe.co.kr" => {
                                                            println!("Parsing autocafe");
                                                            let s = seclist::parse_autocafe::parse(
                                                                document,
                                                            );
                                                        }
                                                        // done
                                                        "ai.carinfo.co.kr" => {
                                                            println!("Parsing carinfo");
                                                            let s = seclist::parse_carinfo::parse(
                                                                document,
                                                            );
                                                        }
                                                        // done
                                                        "ai2.kaai.or.kr" => {
                                                            println!("Parsing ai2.kaai.or.kr");
                                                            let s = seclist::parse_ai2kaai::parse(
                                                                document,
                                                            );
                                                        }
                                                        _ => println!(
                                                            "! Seclist source is never known: {}",
                                                            domain
                                                        ),
                                                    }
                                                }
                                                None => {
                                                    match Url::parse(&res_data[0])
                                                        .unwrap()
                                                        .host()
                                                        .unwrap()
                                                    {
                                                        host => match host.to_string().as_str() {
                                                            "221.143.49.206" => {
                                                                println!("Parsing 221.143.49.206");
                                                                let s = seclist::parse_221::parse(
                                                                    document,
                                                                );
                                                            }
                                                            _ => println!(""),
                                                        },
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => eprintln!("{e:#?}"),
                                    }
                                }
                                ParamSecListType::CheckInfo => {
                                    println!("checkinfo");
                                    // also try to throw like if seclisturl
                                    let s = popup::parse_seclist::parse(
                                        &car.car_seq,
                                        &car_data.params.param_diag_car_yn,
                                        &car_data.params.param_diag_car_seq,
                                        &car_data.params.param_premium_car_yn,
                                    );
                                }
                            }
                            u_mutex_data_list.push(car_data)
                        }
                        Err(e) => eprintln!("{e:#?}"),
                    }
                });
            }
        }
    });
    Ok(mutex_data_list.into_inner().unwrap())
}
