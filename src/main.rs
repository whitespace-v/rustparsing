#![warn(clippy::all, clippy::pedantic)]
mod http;
mod kbchachacha;
#[tokio::main]
async fn main() {
    kbchachacha::parser::parse().await;
}
