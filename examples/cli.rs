use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use teslatte::auth::{AccessToken, Authentication, RefreshToken};
use teslatte::vehicle_state::SetChargeLimit;
use teslatte::{Api, Id};

const TESLA_ACCESS_TOKEN: &str = "TESLA_ACCESS_TOKEN";
const TESLA_REFRESH_TOKEN: &str = "TESLA_REFRESH_TOKEN";

/// Teslatte
///
/// A command line interface for the Tesla API.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Authenticate with Tesla via URL, and receive an access token and refresh token.
    Auth {
        /// Save tokens to a cli.json file.
        ///
        /// Be careful with your access tokens!
        #[clap(short, long)]
        save: bool,
    },

    /// Refresh your tokens.
    Refresh {
        /// If not provided, will try to read the token from a cli.json file and automatically
        /// update the file.
        #[clap(short, long, env = "TESLA_REFRESH_TOKEN")]
        refresh_token: Option<RefreshToken>,
    },

    /// Run API commands.
    Api(ApiArgs),
}

#[derive(Debug, Args)]
struct ApiArgs {
    /// Access token. If not provided, will try to load from the cli.json file.
    #[clap(short, long, env = "TESLA_ACCESS_TOKEN")]
    access_token: Option<AccessToken>,

    #[clap(subcommand)]
    command: ApiCommand,
}

#[derive(Debug, Subcommand)]
enum ApiCommand {
    /// Get a list of vehicles.
    Vehicles,

    /// Get vehicle data.
    VehicleData { id: Id },

    /// Get charge state.
    ChargeState { id: Id },

    /// Set charge limit.
    SetChargeLimit { id: Id, percent: u8 },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Auth { save } => {
            let auth = Authentication::new().unwrap();
            let (access_token, refresh_token) = auth.interactive_get_access_token().await.unwrap();
            updated_tokens(save, access_token, refresh_token);
        }
        Command::Refresh { refresh_token } => {
            let (save, refresh_token) = match refresh_token {
                Some(refresh_token) => (false, refresh_token),
                None => {
                    let config = Config::load();
                    (true, config.refresh_token)
                }
            };

            let auth = Authentication::new().unwrap();
            let response = auth.refresh_access_token(&refresh_token).await.unwrap();
            updated_tokens(save, response.access_token, refresh_token);
        }
        Command::Api(api_args) => {
            let access_token = match api_args.access_token {
                Some(a) => a,
                None => {
                    let config = Config::load();
                    config.access_token
                }
            };

            let api = Api::new(&access_token);
            #[allow(unused_results)]
            match api_args.command {
                ApiCommand::Vehicles => {
                    dbg!(api.vehicles().await.unwrap());
                }
                ApiCommand::VehicleData { id } => {
                    dbg!(api.vehicle_data(&id).await.unwrap());
                }
                ApiCommand::ChargeState { id } => {
                    dbg!(api.charge_state(&id).await.unwrap());
                }
                ApiCommand::SetChargeLimit { id, percent } => {
                    dbg!(api
                        .set_charge_limit(&id, &SetChargeLimit { percent })
                        .await
                        .unwrap());
                }
            }
        }
    }
}

fn updated_tokens(save: bool, access_token: AccessToken, refresh_token: RefreshToken) {
    println!("Access token: {}", access_token.0);
    println!("Refresh token: {}", refresh_token.0);
    if save {
        Config {
            access_token,
            refresh_token,
        }
        .save();
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    access_token: AccessToken,
    refresh_token: RefreshToken,
}

impl Config {
    fn save(&self) {
        let json = serde_json::to_string(&self).unwrap();
        std::fs::write("cli.json", json).unwrap();
    }

    fn load() -> Self {
        let file = std::fs::File::open("cli.json").unwrap();
        let reader = std::io::BufReader::new(file);
        let json: serde_json::Value = serde_json::from_reader(reader).unwrap();
        let config: Config = serde_json::from_str(&json.to_string()).unwrap();
        config
    }
}
