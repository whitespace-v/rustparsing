#![warn(clippy::all, clippy::pedantic)]

use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
