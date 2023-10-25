mod api_md;
mod fleet;
mod nom_help;
mod teslatte;
mod timdorr;
mod vehicle_command;

use crate::fleet::FleetEndpoint;
use crate::teslatte::TeslatteEndpoint;
use crate::timdorr::TimdorrEndpoint;
use crate::vehicle_command::VehicleCommandEndpoint;
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{error, info};

const TIMDORR_URL: &str =
    "https://raw.githubusercontent.com/timdorr/tesla-api/master/ownerapi_endpoints.json";
const TIMDORR_FILE: &str = "timdorr.json";
const VEHICLE_COMMAND_URL: &str = "https://raw.githubusercontent.com/teslamotors/vehicle-command/main/cmd/tesla-control/commands.go";
const VEHICLE_COMMAND_FILE: &str = "vehicle_command.go";
const FLEET_API_URL: &str = "https://developer.tesla.com/docs/fleet-api";
const FLEET_API_FILE: &str = "fleet.html";

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Cli {
    /// Use the cached html if exists, to avoid making requests.
    #[clap(short, long)]
    cached: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();

    let (timdorr, fleet_html, command_golang) = tokio::join!(
        cache_fetch(TIMDORR_URL, TIMDORR_FILE, args.cached),
        cache_fetch(FLEET_API_URL, FLEET_API_FILE, args.cached),
        cache_fetch(VEHICLE_COMMAND_URL, VEHICLE_COMMAND_FILE, args.cached)
    );

    let mut teslatte_project_path = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    teslatte_project_path.push("..");
    let teslatte_endpoints = teslatte::parse(&teslatte_project_path).unwrap();
    let teslatte_endpoints: HashMap<String, TeslatteEndpoint> = teslatte_endpoints
        .into_iter()
        .map(|e| (e.name.clone(), e))
        .collect();

    let fleet_endpoints = fleet::parse(&fleet_html);
    let command_endpoints = vehicle_command::parse(&command_golang);
    let timdorr_endpoints = timdorr::parse(&timdorr);

    info!("{} endpoints in teslatte", teslatte_endpoints.len());
    info!("{} endpoints in fleet", fleet_endpoints.len());
    info!("{} endpoints in command", command_endpoints.len());
    info!("{} endpoints in timdorr", timdorr_endpoints.len());

    let mut merged = merge(
        teslatte_endpoints,
        fleet_endpoints,
        command_endpoints,
        timdorr_endpoints,
    )
    .unwrap();

    // Let's do a check to see if the fleet api matches the timdorr api.
    // If it doesn't, let's panic!
    let mut perfect = true;
    for (k, v) in &merged {
        if let Some(fleet) = &v.fleet {
            if let Some(timdorr) = &v.timdorr {
                if fleet.uri != timdorr.uri {
                    error!("{}: fleet: {}, timdorr: {}", k, fleet.uri, timdorr.uri);
                    perfect = false;
                }
            }
        }
    }

    if !perfect {
        panic!("Fleet and Timdorr don't match. See errors above.");
    }

    // filter_interesting_endpoints(&mut merged);
    todo!();

    dbg!(&merged);
}

/// Remove endpoints that we're not interested in (yet) in place.
// pub fn filter_interesting_endpoints(mut endpoints: &mut HashMap<String, Endpoint>) {
//     endpoints.retain(|_, e| {
//         !e. starts_with("/api/1/directives")
//             && !e.starts_with("/api/1/subscriptions")
//             && !e.starts_with("/api/1/dx/")
//             && !e.starts_with("/bff/v2/mobile-app")
//     });
// }

#[derive(Debug)]
pub struct Endpoint {
    pub name: String,
    pub teslatte: Option<TeslatteEndpoint>,
    pub fleet: Option<FleetEndpoint>,
    pub vehicle_command: Option<VehicleCommandEndpoint>,
    pub timdorr: Option<TimdorrEndpoint>,
}

pub fn merge(
    teslatte: HashMap<String, TeslatteEndpoint>,
    fleet: HashMap<String, FleetEndpoint>,
    vehicle_command: HashMap<String, VehicleCommandEndpoint>,
    timdorr: HashMap<String, TimdorrEndpoint>,
) -> anyhow::Result<HashMap<String, Endpoint>> {
    // Collate all the keys into a single set
    let mut keys = HashSet::with_capacity(100);
    keys.extend(teslatte.iter().map(|(k, _)| k.clone()));
    keys.extend(fleet.keys().map(|k| k.clone()));
    keys.extend(vehicle_command.keys().map(|k| k.clone()));
    keys.extend(timdorr.keys().map(|k| k.clone()));

    // Put the keys into a Vec and sort.
    let mut keys: Vec<String> = keys.into_iter().collect();
    keys.sort();

    let mut endpoints = Vec::with_capacity(keys.len());
    for name in keys {
        // for each of these maps, if the key exists, then we have an endpoint.
        let teslatte = teslatte.get(&name).cloned();
        let fleet = fleet.get(&name).cloned();
        let vehicle_command = vehicle_command.get(&name).cloned();
        let timdorr = timdorr.get(&name).cloned();

        let endpoint = Endpoint {
            name,
            teslatte,
            fleet,
            vehicle_command,
            timdorr,
        };

        endpoints.push(endpoint);
    }

    Ok(endpoints
        .into_iter()
        .map(|e| (e.name.clone(), e))
        .collect::<HashMap<String, Endpoint>>())
}

async fn cache_fetch(url: &str, filename: &str, cache: bool) -> String {
    // Write to where this project root is, not in the parent project.
    let mut path = PathBuf::new();
    path.push(env!("CARGO_MANIFEST_DIR"));
    path.push("cached");
    path.push(filename);

    if cache && path.exists() {
        info!("Using cache: {path:?}");
        return std::fs::read_to_string(path).unwrap();
    }

    info!("Fetching {url} -> {path:?}");
    let response = reqwest::get(url).await.unwrap();

    let html = response.text().await.unwrap();

    std::fs::write(path, &html).unwrap();

    html
}
