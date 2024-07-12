use super::structs::ProxyBuilder;
use crate::kbchachacha::maker::structs::{Maker, ProcessorMessage};
use headless_chrome::Tab;
use reqwest::{self, Proxy};
use scraper::{Html, Selector};
use std::error::Error;
use std::sync::mpsc;
use std::thread;

// open:
// https://www.kbchachacha.com/public/search/main.kbc#!?makerCode=101&classCode=1101&carCode= //filter by brand+model
// -> be careful expecting "한줄광고 매물" - однострочное объявление
// get max cars -> get max pages (max cars / 25)
// iterate car list pages and grab "carSeq":
// https://www.kbchachacha.com/public/search/main.kbc#!?makerCode=101&classCode=1101&carCode=&page=2&sort=-orderDate
// then iterate pages and grab full info:
// https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25919156
pub async fn parse() {
    let model_list_result = fetch_models().await;
    if model_list_result.is_ok() {
        let maker: Maker = model_list_result.unwrap();
        let mut url_list: Vec<String> = vec![];
        for code in maker.result.code {
            url_list.push(car_list_link_generator(&code.maker_code, &code.class_code));
        }
        for chunk in url_list.chunks(20) {
            let (tx, rx) = mpsc::channel();
            let processors = convert_parallel(chunk.to_vec(), tx);

            for (index, handle) in processors.into_iter().enumerate() {
                // wait chunk thread
                match handle.join() {
                    // thread successfully ended
                    Ok(()) => println!("Processor {index} is finished!"),
                    // thread downed
                    Err(e) => {
                        // convert error to String with downcaster
                        if let Some(s) = e.downcast_ref::<String>() {
                            // if error converted to String
                            println!("Thread {index} panicked: {s:?}");
                        } else {
                            // if we couldn't convert error to String
                            println!("Unknown error when processing a thread {index}");
                        }
                    }
                }
            }
            // messages from senders
            for received in rx {
                match received {
                    ProcessorMessage::Success(msg) => println!("Message incoming: {msg}"),
                    ProcessorMessage::Error(e) => println!("Error incoming: {e}"),
                }
            }
        }
    }
}

async fn fetch_models() -> Result<Maker, reqwest::Error> {
    let proxy = proxy_builder().await?;
    let client = reqwest::Client::builder().proxy(proxy).build()?;
    match client
        .get("https://www.kbchachacha.com/public/search/carClass.json?makerCode=")
        .send()
        .await
    {
        Ok(response) => match response.json().await {
            Ok(maker) => Ok(maker),
            Err(decode_error) => {
                eprintln!("Error decoding maker: {decode_error}");
                Err(decode_error)
            }
        },

        Err(request_error) => {
            eprintln!("Error requesting maker\n Err: {request_error}\n ");
            Err(request_error)
        }
    }
}

fn car_list_link_generator(maker_code: &str, class_code: &str) -> String {
    let class_code_prefix = String::from("&classCode=");
    let car_code_postfix = String::from("&carCode=");
    let link = String::from("https://www.kbchachacha.com/public/search/main.kbc#!?makerCode=")
        + &maker_code
        + &class_code_prefix
        + &class_code
        + &car_code_postfix;
    link
}

fn fetch_car_list_page(link: &String) {
    println!("started");
    let browser = headless_chrome::Browser::default().unwrap();

    let tab = browser.new_tab().unwrap();

    let active: Result<&Tab, Box<dyn Error>> = match tab.navigate_to(&link) {
        Ok(unloaded_page) => match unloaded_page.wait_until_navigated() {
            Ok(loaded_page) => Ok(loaded_page),
            Err(load_error) => {
                println!("couldn't waited {}, {load_error}", &link);
                Err(load_error.into())
            }
        },
        Err(open_error) => {
            println!("couldn't open {}, {open_error}", &link);
            Err(open_error.into())
        }
    };

    if active.is_ok() {
        let b = &active.unwrap().get_content().expect("sdfsdf");
        let html = Html::parse_document(b);
        let total_selector = Selector::parse("span.__total").unwrap();
        let total_count_str = html
            .select(&total_selector)
            .map(|el| el.inner_html())
            .collect::<String>();
        println!("{total_count_str}");
        let total_count: u16 = total_count_str
            .trim()
            .replace(",", "")
            .parse()
            .expect("please give me correct string number!");
        let pages: u16 = (total_count as f32 / 25 as f32).ceil() as u16;
        println!("Total count: {}, Pages: {} ", total_count, pages);
    }
}

fn convert_parallel(
    url_list: Vec<String>,
    tx: mpsc::Sender<ProcessorMessage>,
) -> Vec<thread::JoinHandle<()>> {
    url_list
        .into_iter()
        .enumerate()
        .map(|(index, url)| {
            let new_tx = tx.clone();
            thread::spawn(move || {
                fetch_car_list_page(&url);
                println!("{}", &url);
                new_tx
                    .send(ProcessorMessage::Success(format!(
                        "{index}. {url:?} is processed!"
                    )))
                    .expect("Failed to send message");
            })
        })
        .collect()
}

async fn get_proxy_data() -> Result<ProxyBuilder, reqwest::Error> {
    match reqwest::Client::new()
        .get("http://192.168.88.245:3333/api/v1/getproxy")
        .send()
        .await
    {
        /* -- successfull respose  --*/
        Ok(response) => match response.text().await {
            /* -- Successfull decode  --*/
            Ok(data) => Ok(serde_json::from_str(&data).unwrap()),
            /* -- Error decode  --*/
            Err(decode_error) => {
                eprintln!("Error decoding from proxy_builder: {decode_error}");
                Err(decode_error)
            }
        },
        /* -- Error respose  --*/
        Err(request_error) => {
            eprintln!("Error requesting from proxy_builder, {request_error}");
            Err(request_error)
        }
    }
}

async fn proxy_builder() -> Result<Proxy, reqwest::Error> {
    // proxy builder fn
    println!("Building proxy...");
    match get_proxy_data().await {
        Ok(proxy) => {
            let proxy_ip = String::from(proxy.data.proxy.ip);
            let proxy_port = String::from(proxy.data.proxy.port);
            let proxy_username = String::from(proxy.data.proxy.login);
            let proxy_password = String::from(proxy.data.proxy.pass);
            let proxy_delimeter = String::from(":");
            let proxy_uri = String::from("https://") + &proxy_ip + &proxy_delimeter + &proxy_port;
            println!("Proxy built: {}", &proxy_uri);
            Ok(reqwest::Proxy::http(&proxy_uri)?.basic_auth(&proxy_username, &proxy_password))
        }
        Err(e) => {
            eprintln!("Error building proxy {e}, must rebuild later...");
            Err(e)
        }
    }
}
