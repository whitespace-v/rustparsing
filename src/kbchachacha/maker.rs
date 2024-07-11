#![warn(clippy::all, clippy::pedantic)]

use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MakerParams {
    debug_mode: bool,
    page_size: u8,
    page: u8,
    default_fields: [i8; 0],
    include_fields: [i8; 0],
    exclude_fields: [i8; 0],
    sort: [i8; 0],
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MakerResult {
    hits: [i8; 0],
    total: u8,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Maker {
    status: u8,
    message: String,
    message_detail: String,
    params: MakerParams,
    result: MakerResult,
}

#[tokio::main]
pub fn parse_maker() -> Result<(), reqwest::Error> {
    // Receive type-checked JSON

    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", todos);

    // Send and receive type-checked JSON

    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: "Subscribe to Let's Get Rusty".to_owned(),
        completed: false,
    };

    let new_todo: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_todo);

    // Send and receive arbitrary JSON

    let new_todo: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&serde_json::json!({
            "userId": 1,
            "title": "Subscribe to Let's Get Rusty".to_owned(),
            "completed": false
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_todo);

    Ok(())
}
