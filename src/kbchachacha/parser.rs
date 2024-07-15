#![warn(clippy::all, clippy::pedantic)]
use crate::kbchachacha::{crawler, maker};

pub async fn parse() {
    // get makers
    let url_list = maker::maker::generate_makers_list()
        .await
        .expect("Couldn't parse maker");
    // let url_list = vec![];
    let v = crawler::crawler::collect_parm_list(url_list);
    println!("{v:?}")
}
