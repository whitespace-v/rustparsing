#![warn(clippy::all, clippy::pedantic)]

use reqwest::{
    cookie::{CookieStore, Jar},
    header::HeaderValue,
    Client, Url,
};
use std::sync::Arc;
use ureq::Error;

mod http;
mod kbchachacha;
fn main() {
    kbchachacha::parser::parse();
}
