#![warn(clippy::all, clippy::pedantic)]
mod http;
mod kbchachacha;
// #[tokio::main]
fn main() {
    kbchachacha::parser::parse();
}
