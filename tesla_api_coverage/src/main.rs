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
use reqwest::Method;
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

trait Restful {
    fn method(&self) -> &Method;
    fn uri(&self) -> &str;
}

trait EndpointGeneral {
    fn name(&self) -> &str;
}

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

    let mut fleet_endpoints = fleet::parse(&fleet_html);
    let mut command_endpoints = vehicle_command::parse(&command_golang);
    let mut timdorr_endpoints = timdorr::parse(&timdorr);

    info!("TOTALS: (before filtering and merging)");
    info!("{} endpoints in teslatte", teslatte_endpoints.len());
    info!("{} endpoints in fleet", fleet_endpoints.len());
    info!("{} endpoints in command", command_endpoints.len());
    info!("{} endpoints in timdorr", timdorr_endpoints.len());

    // Before we merge let's mangle the names so the URI's match.
    info!("-- rename timdorr based on teslatte");
    rename_keys_based_on_uri(&teslatte_endpoints, &mut timdorr_endpoints);
    info!("-- rename fleet based on teslatte");
    rename_keys_based_on_uri(&teslatte_endpoints, &mut fleet_endpoints);
    info!("-- rename timdorr based on fleet");
    rename_keys_based_on_uri(&fleet_endpoints, &mut timdorr_endpoints);
    info!("-- rename vehicle command");
    rename_vehicle_command(&mut command_endpoints);

    let mut merged = merge(
        teslatte_endpoints,
        fleet_endpoints,
        command_endpoints,
        timdorr_endpoints,
    )
    .unwrap();

    remove_unwanted_endpoints(&mut merged);
    ensure_timdorr_matches_fleet(&merged);

    dbg!(&merged);

    api_md::generate(&merged).unwrap();
}

fn rename_vehicle_command(endpoints: &mut HashMap<String, VehicleCommandEndpoint>) {
    let mut renames = vec![
        ("auto-seat-and-climate", "auto-conditioning-start"),
        ("charging-set-limit", "set-charge-limit"),
        ("charging-start", "charge-start"),
        ("charging-stop", "charge-stop"),
        ("charge-port-open", "charge-port-door-open"),
        ("charge-port-close", "charge-port-door-close"),
        ("honk", "honk-horn"),
        ("software-update-cancel", "cancel-software-update"),
        ("wake", "wake-up"),
    ];

    for (old_key, new_key) in renames {
        let endpoint = endpoints.remove(old_key).unwrap();
        endpoints.insert(new_key.to_string(), endpoint);
    }
}

fn rename_keys_based_on_uri(
    base: &HashMap<String, impl Restful>,
    mut to_rename: &mut HashMap<String, impl Restful>,
) {
    let mut renames = vec![];
    for (base_key, base_endpoint) in base.iter() {
        let mut seen = false;
        for (rename_key, rename_endpoint) in to_rename.iter() {
            if base_endpoint.uri() != rename_endpoint.uri() {
                continue;
            }
            if base_endpoint.method() != rename_endpoint.method() {
                continue;
            }
            if base_key == rename_key {
                continue;
            }

            info!(
                "Rename {rename_key} -> {base_key} for {}",
                base_endpoint.uri()
            );
            if seen {
                panic!("Duplicate rename for {base_key} -> {rename_key}");
            }

            renames.push((base_key.to_string(), rename_key.to_string()));
            seen = true;
        }
    }

    for (base_key, rename_key) in renames {
        let endpoint = to_rename.remove(&rename_key).unwrap();
        to_rename.insert(base_key.to_string(), endpoint);
    }
}

fn ensure_timdorr_matches_fleet(merged: &HashMap<String, Endpoint>) {
    // Let's do a check to see if the fleet api matches the timdorr api.
    // If it doesn't, let's panic!
    let mut perfect = true;
    for (k, v) in merged {
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
        panic!("Fleet and timdorr don't match. See errors above.");
    }
}

/// Remove endpoints that we're not interested in (yet) in place.
///
/// Base it on timdorr's list.
pub fn remove_unwanted_endpoints(mut endpoints: &mut HashMap<String, Endpoint>) {
    endpoints.retain(|_, e| {
        let Some(timdorr) = &e.timdorr else {
            return true;
        };
        let uri = &timdorr.uri;
        true // rustfmt hax :)
            && !uri.starts_with("/commerce-api")
            && !uri.starts_with("/api/1/directives")
            && !uri.starts_with("/api/1/subscriptions")
            && !uri.starts_with("/api/1/dx/")
            && !uri.starts_with("/mobile-app")
            && !uri.starts_with("/bff/mobile-app")
            && !uri.starts_with("/bff/v2/mobile-app")
    });
}

#[derive(Debug)]
pub struct Endpoint {
    pub name: String,
    pub teslatte_owners_api: Option<TeslatteEndpoint>,
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
            teslatte_owners_api: teslatte,
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
