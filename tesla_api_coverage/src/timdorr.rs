use heck::ToKebabCase;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct TimdorrEndpoint {
    #[serde(rename = "TYPE")]
    method: String,
    uri: String,
    auth: bool,
}

pub fn parse(json_str: &str) -> HashMap<String, TimdorrEndpoint> {
    let map: HashMap<String, TimdorrEndpoint> = serde_json::from_str(json_str).unwrap();

    // rename all the keys to kebab-case
    map.into_iter()
        .map(|(k, v)| (k.to_kebab_case(), v))
        .collect::<HashMap<String, TimdorrEndpoint>>()
}
