#![warn(clippy::all, clippy::pedantic)]
mod kbchachacha;

#[tokio::main]
async fn main() {
    let maker = kbchachacha::parser::parse().await;
    println!("{maker:#?}")
}
