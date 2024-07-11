use crate::kbchachacha::maker::structs::Maker;
use reqwest;

pub async fn parse() {
    let maker_result = fetch_maker().await;
    if maker_result.is_ok() {
        let maker: Maker = maker_result.unwrap();
        let mut maker_list = maker.result.income;
        maker_list.extend(maker.result.domestical);
        println!("{maker_list:#?}");
    }
}

async fn fetch_maker() -> Result<Maker, reqwest::Error> {
    let marker_url = String::from("https://www.kbchachacha.com/public/search/carMaker.json");
    match reqwest::Client::new().get(&marker_url).send().await {
        /* -- successfull respose  --*/
        Ok(v) => match v.json().await {
            /* -- Successfull decode  --*/
            Ok(s) => Ok(s),
            /* -- Error decode  --*/
            Err(decode_error) => {
                eprintln!("Error decoding maker: {decode_error}");
                Err(decode_error)
            }
        },
        /* -- Error respose  --*/
        Err(request_error) => {
            eprintln!("Error requesting maker: {marker_url},\n {request_error}");
            Err(request_error)
        }
    }
}
