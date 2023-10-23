mod fleet;
mod vehicle_command;

use clap::Parser;
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

    #[clap(short = 'v', long)]
    only_vehicle_command: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();

    // let timorr = cache_fetch(TIMDORR_URL, TIMDORR_FILE, args.cache).await;
    //
    // let fleet_html = cache_fetch(
    //     FLEET_API_URL,
    //     FLEET_API_FILE,
    //     args.cache,
    // )
    // .await;
    //
    // let command_golang = cache_fetch(
    //     VEHICLE_COMMAND_URL,
    //     VEHICLE_COMMAND_FILE,
    //     args.cache,
    // ).await;

    let (timorr, fleet_html, command_golang) = tokio::join!(
        cache_fetch(TIMDORR_URL, TIMDORR_FILE, args.cached),
        cache_fetch(FLEET_API_URL, FLEET_API_FILE, args.cached),
        cache_fetch(VEHICLE_COMMAND_URL, VEHICLE_COMMAND_FILE, args.cached)
    );

    let mut vehicle_command = true;
    let mut fleet_api = true;
    let mut timdorr = true;

    if args.only_vehicle_command {
        fleet_api = false;
        timdorr = false;
    }

    if fleet_api {
        fleet::parse(&fleet_html);
    }

    if vehicle_command {
        vehicle_command::parse(&command_golang);
    }
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
