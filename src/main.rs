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
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // kbchachacha::parser::parse();
//     let url = "https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25495764".to_owned();
//     let client = reqwest::ClientBuilder::new().build()?;
//     let cookie = "WMONID=ZogN8o7iYci; Expires=Thu, 17-Jul-2025 13:48:11 GMT; Path=/, cha-cid=082c7130-63b4-46e1-b344-189875cc039b; Expires=Fri, 17-Jul-2026 04:48:11 GMT; Path=/; HttpOnly, cha-cid=ac89322d-febd-440c-a6d1-425120048db4; Expires=Fri, 17-Jul-2026 04:48:11 GMT; Path=/; HttpOnly, JSESSIONID=RD9HzT78Bj3n0mZ5rmBa0UCcCoPG1S52QhIN5TMHGprsZ9yGzsW0Gy2yO4IgITf7.cGNoYWFwbzFfZG9tYWluL0NBUjNBUF9zZXJ2ZXIyX2Nz; Path=/; Secure; HttpOnly";
//     let jar = Arc::new(Jar::default());
//     jar.add_cookie_str(
//         cookie,
//         &"https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25495764"
//             .to_owned()
//             .parse::<Url>()
//             .unwrap(),
//     );
//     // create the HTTP client
//     let client = Client::builder()
//         .cookie_provider(Arc::clone(&jar))
//         .cookie_store(true)
//         .build()
//         .unwrap();

//     match client.get(url).send().await {
//         Ok(response) => {
//             println!("{:?}", response.url());
//             println!("{:#?}", response.remote_addr());
//             println!("{:?}", response.status());
//             println!("{:?}", response.headers());
//             // println!("er: {:?}", response.text_with_charset())
//         }
//         Err(e) => {
//             println!("{e:#?}");
//             // Err(e)
//         }
//     }
//     Ok(())
// }

fn main() -> Result<(), ureq::Error> {
    //  let proxy = ureq::Proxy::new("user:password@cool.proxy:9090")?;
    match ureq::get("https://www.kbchachacha.com/public/car/detail.kbc?carSeq=25495764").call() {
        Ok(response) => {
            println!("{}", response.status())
        }
        Err(Error::Status(code, response)) => {
            /* the server returned an unexpected status
            code (such as 400, 500 etc) */
        }
        Err(_) => {}
    }
    // println!("{:#?}", body);
    Ok(())
}
