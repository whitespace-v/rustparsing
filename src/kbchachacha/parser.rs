#![warn(clippy::all, clippy::pedantic)]

use crate::kbchachacha::maker;

pub async fn parse() {
    maker::parser::parse().await;
}
