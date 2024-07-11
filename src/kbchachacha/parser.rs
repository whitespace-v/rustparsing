#![warn(clippy::all, clippy::pedantic)]

use crate::kbchachacha::maker;

pub async fn parse() {
    let maker = maker::parser::parse_maker().await;
    println!("{maker:#?}")
}
