#![warn(clippy::all, clippy::pedantic)]
mod extend;
mod extractor;
mod http;
mod kbchachacha;
mod utils;

fn main() {
    kbchachacha::parser::parse();
}
