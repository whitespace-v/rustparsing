use crate::{
    http,
    kbchachacha::structs::{Car, CarData},
};
use std::str;
use std::sync::Mutex;
use std::{error::Error, thread};
pub fn parse(cars: Vec<Car>) -> Result<Vec<CarData>, Box<dyn Error>> {
    let client = http::builder::build()?;

    let mutex_data_list: Mutex<Vec<CarData>> = Mutex::new(vec![]);
    // https://www.kbchachacha.com/public/layer/common/finance/monthly/calc.json
    thread::scope(|scope| {
        for chunk in cars.chunks(20) {
            for car in chunk {
                scope.spawn(|| {
                    let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25495764"
                        .to_owned();
                    match client.get(url).send() {
                        Ok(response) => {
                            // WMONID=MDBkYarIxKD;
                            // cha-cid=79b37b4a-52af-418d-bb86-9c6cbceed235;
                            // _fwb=126EvKROapLPLDBB4eWUUET.1718058826219;
                            // _ga_BQD4DF40J4=GS1.1.1721179787.25.1.1721185129.60.0.1784646260;
                            // _ga=GA1.2.1124855167.1718058828;
                            // _clck=1nkp38h%7C2%7Cfnj%7C0%7C1622;
                            // _fcOM={"k":"b6e8d1c096e14821-60f4376c1900448852f6fb2","i":"45.152.170.77.613506","r":1718058831420};
                            // wcs_bt=unknown:1721185131; _m_uid=1d93d39e-ff34-329f-b2db-137363e27b8d;
                            // _m_uidt=C;
                            // _m_uid_type=A;
                            // TR10062602448_t_uid=49151715018094327.1721128398637;
                            // _M_CS[T]=1;
                            // _gid=GA1.2.1537194747.1721108981;
                            // page-no-action-count=-1;
                            // JSESSIONID=4Vtf5QRxYQFrtmViO5S3l0FWCK5xfCVnQ5Libym4vdJ1mafVzrA3rF7K0fyNpcn5.cGNoYWFwbzFfZG9tYWluL0NBUjNBUF9zZXJ2ZXIyX2Nz;
                            // TR10062602448_t_sst=49151130033563943.1721185209650;
                            // TR10062602448_t_if=15.0.0.0.null.null.null.49151127600001943;
                            // recent-visited-car=25919156%2C25495764;
                            // _m_analytics_off=true;
                            // _clsk=k9e48w%7C1721186687160%7C12%7C1%7Cv.clarity.ms%2Fcollect
                            let mut u_mutex_data_list = mutex_data_list.lock().unwrap();
                            println!("{:?}", response.status());
                            println!("{:?}", response.headers());

                            // match response.text() {
                            //     Ok(r) => {
                            //         let l = str::from_utf8(r.as_bytes()).unwrap();
                            //         println!("{l}")
                            //     }
                            //     Err(r) => println!("{r:#?}"),
                            // }

                            // let document =
                            //     &scraper::Html::parse_document(&response.text().unwrap());
                            // // println!("{document:#?}");
                            // // Финансовые консультации
                            // let seller_commerce = &scraper::Selector::parse("li.price").unwrap();

                            // // рабочее время
                            // let seller_time =
                            //     &scraper::Selector::parse("span.tl-area__foot").unwrap();

                            // //works
                            // let selectors = vec![seller_commerce, seller_time];
                            // for selector in selectors {
                            //     let title = document
                            //         .select(&selector)
                            //         .next()
                            //         .map(|price| price.text().collect::<String>());

                            //     println!("{title:?}");
                            // }

                            // extract data
                            let car_data = CarData {
                                title: String::from("sds"),
                                maker_code: car.maker_code.to_string(),
                                class_code: car.class_code.to_string(),
                            };
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
