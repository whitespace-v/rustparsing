use crate::http;
use std::sync::{Arc, Mutex};
use std::thread;

/* Here we create links for car page */
pub fn collect_parm_list(url_list: Vec<String>) -> Vec<String> {
    let client = http::builder::build();
    let id_list = Mutex::new(vec![]);
    thread::scope(|s| {
        for chunk in url_list.chunks(20) {
            for url in chunk {
                s.spawn(|| {
                    // total: optionSale -> result -> total
                    println!("thread, {}", url.to_string());
                    match client.get(url.to_string()) {
                        Ok(r) => println!("{r:?}"),
                        Err(e) => println!("{e}"),
                    }
                    // let mut id_list_v = id_list.lock().unwrap();
                    // id_list_v.push(url.to_string())
                });
            }
        }
    });

    id_list.into_inner().unwrap()
}
