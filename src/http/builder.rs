use super::structs::ProxyBuilder;
use reqwest::blocking::Client;
use serde::de::Error;
use ureq::Agent;

pub fn build_reqwest_client() -> Result<Client, reqwest::Error> {
    let proxy = get_proxy()?;
    let proxy_uri = "https://".to_owned() + &proxy.data.proxy.ip + ":" + &proxy.data.proxy.port;

    let auth_client = reqwest::Proxy::http(&proxy_uri)?
        .basic_auth(&proxy.data.proxy.login, &proxy.data.proxy.pass);

    println!("Building proxy...");
    let client = reqwest::blocking::Client::builder()
        .proxy(auth_client)
        .build()?;

    Ok(client) // or if error -> retry till Ok
}

pub fn build_ureq_client() -> Result<Agent, Box<dyn std::error::Error>> {
    let proxy = get_proxy()?;
    let proxy_uri = proxy.data.proxy.login
        + ":"
        + &proxy.data.proxy.pass
        + "@"
        + &proxy.data.proxy.ip
        + ":"
        + &proxy.data.proxy.port;
    let proxy_auth = ureq::Proxy::new(proxy_uri)?;
    let agent = ureq::AgentBuilder::new().proxy(proxy_auth).build();
    println!("Building proxy...");
    Ok(agent) // or if error -> retry till Ok
}

pub fn get_proxy() -> Result<ProxyBuilder, reqwest::Error> {
    println!("Requesting proxy...");
    let proxy = reqwest::blocking::Client::new()
        .get("http://192.168.88.245:3333/api/v1/getproxy")
        .send()?
        .text()?;
    let proxy_data = serde_json::from_str(&proxy).unwrap();
    Ok(proxy_data)
}
