use crate::Restful;
use heck::ToKebabCase;
use reqwest::Method;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct TimdorrEndpoint {
    #[serde(rename = "TYPE")]
    pub method: String,
    pub uri: String,
    pub auth: bool,
}

impl Restful for TimdorrEndpoint {
    fn method(&self) -> &Method {
        match self.method.as_str() {
            "GET" => &Method::GET,
            "POST" => &Method::POST,
            "PUT" => &Method::PUT,
            "DELETE" => &Method::DELETE,
            _ => panic!("Unknown method {}", self.method),
        }
    }

    fn uri(&self) -> &str {
        &self.uri
    }
}

pub fn parse(json_str: &str) -> HashMap<String, TimdorrEndpoint> {
    let map: HashMap<String, TimdorrEndpoint> = serde_json::from_str(json_str).unwrap();

    // Massage all URLs to have a / before "api".
    let map = map
        .into_iter()
        .map(|(k, mut v)| {
            v.uri = format!("/{}", v.uri);
            (k, v)
        })
        .collect::<HashMap<String, TimdorrEndpoint>>();

    // Rename all the keys to kebab-case
    map.into_iter()
        .map(|(k, v)| (k.to_kebab_case(), v))
        .collect::<HashMap<String, TimdorrEndpoint>>()
}
