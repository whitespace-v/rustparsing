#![warn(clippy::all, clippy::pedantic)]
mod kbchachacha;

#[tokio::main]
async fn main() {
    kbchachacha::parser::parse().await;
}
