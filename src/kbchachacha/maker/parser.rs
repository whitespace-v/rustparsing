use crate::kbchachacha::maker::structs::Maker;
use reqwest;

pub async fn parse() {
    let model_list_result = fetch_models().await;
    if model_list_result.is_ok() {
        let maker: Maker = model_list_result.unwrap();
        for code in maker.result.code {
            println!("{code:#?}")
            // then go to every page, collect carSeq (=car id), and open
            // https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25919156
        }
    }
}

async fn fetch_models() -> Result<Maker, reqwest::Error> {
    let maker_url =
        String::from("https://www.kbchachacha.com/public/search/carClass.json?makerCode=");
    match reqwest::Client::new().get(&maker_url).send().await {
        /* -- successfull respose  --*/
        Ok(response) => match response.json().await {
            /* -- Successfull decode  --*/
            Ok(maker) => Ok(maker),
            /* -- Error decode  --*/
            Err(decode_error) => {
                eprintln!("Error decoding maker: {decode_error}");
                Err(decode_error)
            }
        },
        /* -- Error respose  --*/
        Err(request_error) => {
            eprintln!("Error requesting maker: {maker_url},\n {request_error}");
            Err(request_error)
        }
    }
}
