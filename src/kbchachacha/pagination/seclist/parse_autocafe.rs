use std::error::Error;
// use follow_redirects::Client;
use isahc::ReadResponseExt;
use reqwest::{redirect::Policy, Client, StatusCode};
use scraper::Html;
extern crate follow_redirects;
use crate::http;
// http://autocafe.co.kr/ASSO/CarCheck_Form.asp?OnCarNo=2023300215707
// https://ck.carmodoo.com/carCheck/carmodooPrint.do?print=0&checkNum=6623017076

pub fn parse(document: &Html) -> Result<(), Box<dyn Error>> {
    Ok(())
}
