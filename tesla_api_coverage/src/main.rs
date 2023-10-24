mod fleet;
mod nom_help;
mod teslatte;
mod timdorr;
mod vehicle_command;

use clap::{Parser, Subcommand};
use scraper::Element;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::info;

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

    // let timdorr_endpoints = timdorr::parse(&timdorr);
    // let fleet_endpoints = fleet::parse(&fleet_html);
    // let vehicle_command_endpoints = vehicle_command::parse(&command_golang);

    let mut teslatte_project_path = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    teslatte_project_path.push("..");

    let teslatte_endpoints = teslatte::parse(&teslatte_project_path);

    // // Make hashsets from all the keys and see what's different between timdorr and fleet
    // let timdorr_keys: std::collections::HashSet<&String> = timdorr_endpoints.keys().collect();
    // let fleet_keys: std::collections::HashSet<&String> = fleet_endpoints.keys().collect();
    //
    // let timdorr_only = timdorr_keys.difference(&fleet_keys);
    // let fleet_only = fleet_keys.difference(&timdorr_keys);
    // let both = timdorr_keys.intersection(&fleet_keys);
    //
    // info!("Timdorr only: {:?}", timdorr_only);
    // info!("Fleet only: {:?}", fleet_only);
    // info!("Both: {:?}", both);
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
