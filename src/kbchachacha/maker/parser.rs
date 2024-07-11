use crate::kbchachacha::maker::structs::Maker;
use reqwest;

pub async fn parse() {
    let brand_list_result = fetch_brands().await;
    if brand_list_result.is_ok() {
        let maker: Maker = brand_list_result.unwrap();
        let mut brand_list = maker.result.income;
        brand_list.extend(maker.result.domestical);
        for brand in &brand_list {
            fetch_models(&brand.maker_code).await;
        }
    }
}

async fn fetch_brands() -> Result<Maker, reqwest::Error> {
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

async fn fetch_models(model: &String) {
    println!(" {:?}", model);
}
