use crate::kbchachacha::maker::structs::ProcessorMessage;
use headless_chrome::Tab;
use serde::de::Error;
use std::{os::unix::thread, sync::mpsc};

pub async fn collect_id_list(url_list: Vec<String>) {
    for chunk in url_list.chunks(20) {
        println!("{:?}", &chunk);
        // let (tx, rx) = mpsc::channel();

        // let processors = convert_parallel(chunk.to_vec(), tx);

        // for (index, handle) in processors.into_iter().enumerate() {
        //     // wait chunk thread
        //     match handle.join() {
        //         // thread successfully ended
        //         Ok(()) => println!("Processor {index} is finished!"),
        //         // thread downed
        //         Err(e) => {
        //             // convert error to String with downcaster
        //             if let Some(s) = e.downcast_ref::<String>() {
        //                 // if error converted to String
        //                 println!("Thread {index} panicked: {s:?}");
        //             } else {
        //                 // if we couldn't convert error to String
        //                 println!("Unknown error when processing a thread {index}");
        //             }
        //         }
        //     }
        // }
        // // messages from senders
        // for received in rx {
        //     match received {
        //         ProcessorMessage::Success(msg) => println!("Message incoming: {msg}"),
        //         ProcessorMessage::Error(e) => println!("Error incoming: {e}"),
        //     }
        // }
    }
}
// fn fetch_car_list_page(link: &String) {
//     println!("started");
//     let browser = headless_chrome::Browser::default().unwrap();

//     let tab = browser.new_tab().unwrap();

//     let active: Result<&Tab, Box<dyn Error>> = match tab.navigate_to(&link) {
//         Ok(unloaded_page) => match unloaded_page.wait_until_navigated() {
//             Ok(loaded_page) => Ok(loaded_page),
//             Err(load_error) => {
//                 println!("couldn't waited {}, {load_error}", &link);
//                 Err(load_error.into())
//             }
//         },
//         Err(open_error) => {
//             println!("couldn't open {}, {open_error}", &link);
//             Err(open_error.into())
//         }
//     };

//     if active.is_ok() {
//         let b = &active.unwrap().get_content().expect("sdfsdf");
//         let html = Html::parse_document(b);
//         let total_selector = Selector::parse("span.__total").unwrap();
//         let total_count_str = html
//             .select(&total_selector)
//             .map(|el| el.inner_html())
//             .collect::<String>();
//         println!("{total_count_str}");
//         let total_count: u16 = total_count_str
//             .trim()
//             .replace(",", "")
//             .parse()
//             .expect("please give me correct string number!");
//         let pages: u16 = (total_count as f32 / 25 as f32).ceil() as u16;
//         println!("Total count: {}, Pages: {} ", total_count, pages);
//     }
// }

// fn convert_parallel(
//     url_list: Vec<String>,
//     tx: mpsc::Sender<ProcessorMessage>,
// ) -> Vec<thread::JoinHandle<()>> {
//     url_list
//         .into_iter()
//         .enumerate()
//         .map(|(index, url)| {
//             let new_tx = tx.clone();
//             thread::spawn(move || {
//                 fetch_car_list_page(&url);
//                 println!("{}", &url);
//                 new_tx
//                     .send(ProcessorMessage::Success(format!(
//                         "{index}. {url:?} is processed!"
//                     )))
//                     .expect("Failed to send message");
//             })
//         })
//         .collect()
// }
