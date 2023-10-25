use crate::Restful;
use heck::ToKebabCase;
use reqwest::Method;
use scraper::{Element, ElementRef, Html, Selector};
use std::collections::HashMap;
use std::str::FromStr;
use tracing::debug;

struct FleetApiSpec {
    calls: HashMap<String, FleetEndpoint>,
}

// e.g. serialize to similar: vehicle-endpoints
#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
enum Category {
    ChargingEndpoints,
    PartnerEndpoints,
    UserEndpoints,
    VehicleCommands,
    VehicleEndpoints,
}

#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Scope {
    /// Profile Information
    ///
    /// Contact information, home address, profile picture, and referral information.
    UserData,

    /// Vehicle Information
    ///
    /// Vehicle live data, location, eligible upgrades, nearby superchargers, ownership, and service scheduling data.
    VehicleDeviceData,

    /// Vehicle Commands
    ///
    /// Commands like add/remove driver, access Live Camera, unlock, wake up, remote start, and schedule software updates.
    VehicleCmds,

    /// Vehicle Charging Management
    ///
    /// Vehicle charging history, billed amount, charging location, commands to schedule, and start/stop charging.
    VehicleChargingCmds,

    /// Energy Product Information
    ///
    /// Energy flow history, saving forecast, tariff rates, grid import, calendar, site status, time of use, and ownership.
    EnergyDeviceData,

    /// Energy Product Commands
    ///
    /// Commands like update storm mode.
    EnergyCmds,
}

enum InRequestData {
    Query,
    Body,
}

pub struct Parameter {
    name: String,
    request: InRequestData,
    var_type: String,
    required: bool,
    description: String,
}

#[derive(Debug, Clone)]
pub struct FleetEndpoint {
    pub name: String,
    pub method: Method,
    pub uri: String,
    // description: String,
    // category: Category,
    // scopes: Vec<Scope>,
    // parameters: Vec<Parameter>,
    // request_example: String,
    // response_example: String,
}

impl Restful for FleetEndpoint {
    fn method(&self) -> &Method {
        &self.method
    }

    fn uri(&self) -> &str {
        &self.uri
    }
}

pub fn parse(html: &str) -> HashMap<String, FleetEndpoint> {
    let document = Html::parse_document(html);
    let content_selector = selector(".content h1");
    let mut element = document.select(&content_selector).next().unwrap();
    let mut category = None;

    let mut map = HashMap::with_capacity(100);

    // Iterate over all the elements in the content section until we see a h1 or h2.
    loop {
        match element.value().name() {
            "h1" => {
                let category_name = element.value().id().unwrap();
                category = Category::from_str(&category_name).ok();
            }
            "h2" => {
                if category.is_some() {
                    let name = element.inner_html().to_kebab_case();
                    let call = parse_call(element);

                    if let Some(endpoint) = call {
                        debug!("{category:?} {endpoint:?}");
                        map.insert(name, endpoint);
                    }
                }
            }
            _ => {}
        }

        let Some(next_element) = element.next_sibling_element() else {
            break;
        };
        element = next_element;
    }

    // Replace all vehicles/{id}/ to use {vehicle_id}
    for (name, endpoint) in &mut map {
        endpoint.uri = endpoint
            .uri
            .replace("vehicles/{id}/", "vehicles/{vehicle_id}/");
    }

    return map;
}

/// Return None if this is not an endpoint.
///
/// Will panic if it looks like an endpoint and has trouble parsing.
fn parse_call(element: ElementRef) -> Option<FleetEndpoint> {
    let name = element.value().id().unwrap();

    // <p><span class="endpoint"><code>POST /api/1/vehicles/{id}/command/auto_conditioning_start</code></span></p>
    // This section determines if this is an endpoint or not.
    let (fragment, element) = next(element);
    let url = fragment.select(&selector("code")).next()?.inner_html();
    if !url.starts_with("GET ") && !url.starts_with("POST ") {
        return None;
    }

    let (method, url) = url.split_once(' ').unwrap();

    // <p>scopes: <em>vehicle_cmds</em></p>
    let (fragment, element) = next(element);
    let scopes = fragment
        .select(&selector("em"))
        .map(|e| e.inner_html())
        .map(|e| Scope::from_str(&e))
        .collect::<Vec<_>>();

    // 4 <div class="highlight"> nodes containing example requests in different languages.
    // TODO: Skip for now
    let mut count = 0;
    let mut element = element;
    loop {
        let (fragment, new_element) = next(element);
        element = new_element;
        if fragment
            .select(&selector(r#"div[class="highlight"]"#))
            .next()
            .is_none()
        {
            break;
        }

        count += 1;
        if count == 10 {
            panic!("Too many examples");
        }
    }
    if count == 0 && name != "api-status" {
        panic!("No examples for {}", name);
    }

    Some(FleetEndpoint {
        name: name.to_string(),
        method: reqwest::Method::from_bytes(method.as_bytes()).unwrap(),
        uri: url.to_string(),
    })
}

fn next(element: ElementRef) -> (Html, ElementRef) {
    let element = element.next_sibling_element().unwrap();
    let html = Html::parse_fragment(&element.html());
    (html, element)
}

fn selector(s: &str) -> Selector {
    Selector::parse(s).unwrap()
}
