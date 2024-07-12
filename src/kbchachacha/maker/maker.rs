use super::structs::Maker;
use crate::http;

pub async fn generate_url_list() -> Result<Vec<String>, reqwest::Error> {
    let client = http::builder::build().await.expect("Couldn't built client");
    match client
        .get("https://www.kbchachacha.com/public/search/carClass.json?makerCode=")
        .send()
        .await
    {
        Ok(response) => match response.json::<Maker>().await {
            Ok(maker_data) => {
                let maker: Maker = maker_data;
                let mut url_list: Vec<String> = vec![];
                for code in maker.result.code {
                    url_list.push(car_list_link_generator(&code.maker_code, &code.class_code));
                }
                Ok(url_list)
            }
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
