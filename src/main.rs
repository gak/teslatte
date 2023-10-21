use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use teslatte::auth::{AccessToken, RefreshToken};
use teslatte::cli::energy::EnergySiteArgs;
use teslatte::cli::powerwall::PowerwallArgs;
use teslatte::cli::vehicle::VehicleArgs;
use teslatte::{OwnerApi, PrintResponses, VehicleApi};

/// Teslatte
///
/// A command line interface for the Tesla API.
#[derive(Parser, Debug)]
#[clap(author, version)]
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
    /// List of vehicles.
    Vehicles,

    /// Specific Vehicle.
    Vehicle(VehicleArgs),

    /// List of energy sites.
    Products,

    /// Specific energy site.
    EnergySite(EnergySiteArgs),

    /// Powerwall queries.
    Powerwall(PowerwallArgs),
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();

    match args.command {
        Command::Auth { save } => {
            let api = OwnerApi::from_interactive_url().await?;
            print_or_save_tokens(save, &api);
        }
        Command::Refresh { refresh_token } => {
            let (save, refresh_token) = match refresh_token {
                Some(refresh_token) => (false, refresh_token),
                None => {
                    let config = Config::load();
                    (true, config.refresh_token)
                }
            };

            let api = OwnerApi::from_refresh_token(&refresh_token).await?;
            print_or_save_tokens(save, &api);
        }
        Command::Api(api_args) => {
            let (access_token, refresh_token) = match &api_args.access_token {
                Some(a) => (a.clone(), None),
                None => {
                    let config = Config::load();
                    (
                        config.access_token.clone(),
                        Some(config.refresh_token.clone()),
                    )
                }
            };

            let mut api = OwnerApi::new(access_token, refresh_token);
            api.print_responses = PrintResponses::Pretty;
            match api_args.command {
                ApiCommand::Vehicles => {
                    api.vehicles().await?;
                }
                ApiCommand::Vehicle(v) => {
                    v.run(&api).await?;
                }
                ApiCommand::Products => {
                    api.products().await?;
                }
                ApiCommand::EnergySite(e) => {
                    e.run(&api).await?;
                }
                ApiCommand::Powerwall(p) => {
                    p.run(&api).await?;
                }
            }
        }
    }
    Ok(())
}

fn print_or_save_tokens(save: bool, api: &OwnerApi) {
    let access_token = api.access_token.clone();
    let refresh_token = api.refresh_token.clone().unwrap();

    if save {
        Config {
            access_token,
            refresh_token,
        }
        .save();
        println!("Saved tokens to cli.json");
    } else {
        println!("Access token: {}", access_token);
        println!("Refresh token: {}", refresh_token);
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
