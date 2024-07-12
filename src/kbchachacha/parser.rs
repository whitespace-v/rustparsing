#![warn(clippy::all, clippy::pedantic)]
use crate::kbchachacha::{crawler, maker};

pub async fn parse() {
    // let url_list = maker::maker::generate_url_list()
    //     .await
    //     .expect("Couldn't parse maker");
    let url_list = vec![];
    let v = crawler::crawler::collect_id_list(url_list).expect("Couldn't collect_id_list");
    println!("{v:?}")
}
