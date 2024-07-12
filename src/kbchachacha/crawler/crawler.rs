use std::sync::{Arc, Mutex};
use std::thread;
pub fn collect_id_list(url_list: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let shared_id_list = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];
    for i in 0..20 {
        let id_list = Arc::clone(&shared_id_list);
        let handle = thread::spawn(move || {
            // total: optionSale -> result -> total
            let mut id_list_v = id_list.lock().unwrap();
            id_list_v.push(i.to_string())
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    let x = shared_id_list.lock().unwrap();
    Ok(x.to_vec())
}
