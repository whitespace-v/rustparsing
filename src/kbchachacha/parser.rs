#![warn(clippy::all, clippy::pedantic)]
use crate::kbchachacha::{crawler, maker};

pub async fn parse() {
    // let url_list = maker::maker::generate_url_list()
    //     .await
    //     .expect("Couldn't parse maker");
    // crawler::crawler::collect_id_list(url_list).await;
    match crawler::crawler::main() {
        Ok(r) => println!("{:?}", r),
        Err(e) => eprintln!("{e}"),
    }
}
