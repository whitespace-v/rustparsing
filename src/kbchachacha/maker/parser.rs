#![warn(clippy::all, clippy::pedantic)]
use crate::kbchachacha::maker::structs::Maker;
use reqwest;

pub async fn parse_maker() -> Result<Maker, reqwest::Error> {
    let marker_url = String::from("https://www.kbchachacha.com/public/search/carMaker.json");
    match reqwest::Client::new().get(&marker_url).send().await {
        /* -- successfull respose  --*/
        Ok(v) => match v.json().await {
            /* -- Successfull decode  --*/
            Ok(s) => return Ok(s),
            /* -- Error decode  --*/
            Err(decode_error) => {
                eprintln!("Error decoding maker: {decode_error}");
                return Err(decode_error);
            }
        },
        /* -- Error respose  --*/
        Err(request_error) => {
            eprintln!("Error requesting maker: {marker_url},\n {request_error}");
            return Err(request_error);
        }
    };
}
