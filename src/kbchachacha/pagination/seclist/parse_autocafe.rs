use std::error::Error;
// use follow_redirects::Client;
use isahc::ReadResponseExt;
use reqwest::{redirect::Policy, Client, StatusCode};
extern crate follow_redirects;
use crate::http;
// http://autocafe.co.kr/ASSO/CarCheck_Form.asp?OnCarNo=2023300215707
// https://ck.carmodoo.com/carCheck/carmodooPrint.do?print=0&checkNum=6623017076

pub fn parse(url: &String) -> Result<(), Box<dyn Error>> {
    // let agent = http::builder::build_ureq_client()?;
    let client = http::builder::build_reqwest_client()?;
    // let mut response = isahc::get(url)?;
    // let url = "http://docs.rs/hyper".parse().unwrap();
    // let future = client.follow_redirects().get(url);
    // println!("isach {:?}",res );

    match client.get(url).send(){
        Ok(response) => {
            let html = response.text().expect("couldn't parse string");
            // let document = &scraper::Html::parse_document(&html);
            // let s = response.get_url();
            // let b = response.headers_names();
            // let g = &response.status();

            println!("sss {html:#?}")
        }
        Err(e) => {
            eprintln!("sss {e:#?}")
        }
    }

    Ok(())
}
