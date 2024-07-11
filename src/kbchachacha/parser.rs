#![warn(clippy::all, clippy::pedantic)]
use crate::kbchachacha::*;
use tokio;
#[tokio::main]
pub async fn parse() {
    // parse makers:
    let r = maker::parse_maker();
    println!("{r:?}")
}
