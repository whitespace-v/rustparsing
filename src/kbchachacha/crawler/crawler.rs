use std::sync::{Arc, Mutex};
use std::thread;

pub fn collect_parm_list(url_list: Vec<String>) -> Vec<String> {
    let id_list = Mutex::new(vec![]);
    thread::scope(|s| {
        for chunk in url_list.chunks(20) {
            for url in chunk {
                s.spawn(|| {
                    // total: optionSale -> result -> total

                    let mut id_list_v = id_list.lock().unwrap();
                    id_list_v.push(url.to_string())
                });
            }
        }
    });

    id_list.into_inner().unwrap()
}
