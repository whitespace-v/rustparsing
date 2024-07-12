use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProxyBuilder {
    pub status: bool,
    pub data: ProxyBuilderData,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProxyBuilderData {
    pub proxy: ProxyBuilderDataProxy,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProxyBuilderDataProxy {
    pub ip: String,
    pub port: String,
    pub login: String,
    pub pass: String,
}
