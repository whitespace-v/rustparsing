#![warn(clippy::all, clippy::pedantic)]
use crate::kbchachacha::{crawler, maker};

pub async fn parse() {
    // get makers
    let url_list = maker::maker::generate_makers_list()
        .await
        .expect("Couldn't parse maker");
    // let url_list = vec![];
    // let url_list = vec![
    //     "https://stackoverflow.com/questions/71165876/rust-json-method-not-found-in-resultreqwestblockingresponse-reqwester".to_owned(),
    //     "https://www.google.com/search?q=error+handling+reqwest+blocking".to_owned()
    // ];
    let v = crawler::crawler::collect_parm_list(url_list);
    println!("{v:?}")
}
