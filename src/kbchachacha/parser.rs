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

pub async fn parse() -> Vec<Todo> {
    let marker_url = String::from("https://jsonplaceholder.typicode.com/todos?userId=1");
    let mut todo: Vec<Todo> = vec![];
    match reqwest::Client::new().get(&marker_url).send().await {
        Ok(v) => match v.json().await {
            Ok(s) => todo = s,
            Err(decode_error) => eprintln!("Error decoding maker: {decode_error}"),
        },
        Err(request_error) => eprintln!("Error requesting maker: {},\n {}", { &marker_url }, {
            &request_error
        }),
    };
    todo
}
