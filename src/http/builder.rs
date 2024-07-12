use super::structs::ProxyBuilder;
use reqwest::Client;

pub async fn get_proxy() -> Result<ProxyBuilder, reqwest::Error> {
    match reqwest::Client::new()
        .get("http://192.168.88.245:3333/api/v1/getproxy")
        .send()
        .await
    {
        Ok(response) => match response.text().await {
            Ok(data) => Ok(serde_json::from_str(&data).unwrap()),
            Err(decode_error) => Err(decode_error),
        },

        Err(request_error) => Err(request_error),
    }
}

pub async fn build() -> Result<Client, reqwest::Error> {
    println!("Building proxy...");
    match get_proxy().await {
        Ok(proxy) => {
            let proxy_ip = String::from(proxy.data.proxy.ip);
            let proxy_port = String::from(proxy.data.proxy.port);
            let proxy_username = String::from(proxy.data.proxy.login);
            let proxy_password = String::from(proxy.data.proxy.pass);
            let proxy_delimeter = String::from(":");
            let proxy_uri = String::from("https://") + &proxy_ip + &proxy_delimeter + &proxy_port;
            println!("Proxy built: {}", &proxy_uri);
            let auth_client =
                reqwest::Proxy::http(&proxy_uri)?.basic_auth(&proxy_username, &proxy_password);
            Ok(reqwest::Client::builder().proxy(auth_client).build()?)
        }
        Err(e) => {
            eprintln!("Error building proxy {e}, must rebuild later...");
            Err(e)
        }
    }
}
