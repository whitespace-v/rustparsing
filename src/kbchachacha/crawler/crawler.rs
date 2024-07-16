use crate::http;
use std::error::Error;
use std::sync::Mutex;
use std::thread;

use super::structs::OptionResponse;
// total: optionSale -> result -> total
pub fn collect_param_list(url_list: Vec<String>) -> Result<Vec<OptionResponse>, Box<dyn Error>> {
    let client = http::builder::build()?;
    let id_list = Mutex::new(vec![]);

    thread::scope(|scope| {
        for chunk in url_list.chunks(20) {
            for url in chunk {
                scope.spawn(|| {
                    match client.get(url.to_string()).send() {
                        Ok(response) => match response.json::<OptionResponse>() {
                            Ok(serded_json) => {
                                let mut id_list_v = id_list.lock().unwrap();
                                id_list_v.push(serded_json)
                            }
                            Err(e) => {
                                eprintln!("{e:#?}")
                            }
                        },
                        Err(e) => {
                            eprintln!("{e:#?}")
                        }
                    };
                });
            }
        }
    });

    Ok(id_list.into_inner().unwrap())
}
