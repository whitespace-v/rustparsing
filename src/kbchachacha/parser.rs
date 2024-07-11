#![warn(clippy::all, clippy::pedantic)]
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

pub async fn parse() -> Result<Vec<Todo>, reqwest::Error> {
    let marker_url = String::from("https://jsonplaceholder.typicode.com/todos?userId=1");
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
