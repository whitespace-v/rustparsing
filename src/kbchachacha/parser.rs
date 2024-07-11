#![warn(clippy::all, clippy::pedantic)]

pub async fn parse() {
    let maker = super::maker::parse_maker().await;
    println!("{maker:#?}")
}
