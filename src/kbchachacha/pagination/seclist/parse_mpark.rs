use scraper::Html;
use std::error::Error;
use ureq::Agent;
use url::Url;

use crate::http;

pub fn parse(query: &str) -> Result<(), Box<dyn Error>> {
    let item = query
        .split('/')
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_owned();
    let agent = http::builder::build_ureq_client()?;
    let url = "https://api.m-park.co.kr/home/api/v1/wb/searchmycar/carcheckdetailinfo/get?checkNo="
        .to_owned()
        + item;
    match agent.get(&url).call() {
        Ok(r) => {
            println!("{:?}", r.into_string().unwrap())
        }
        Err(error) => println!("{error:#?}"),
    }
    Ok(())
}
