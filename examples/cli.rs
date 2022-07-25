mod cli_vehicle;

use chrono::DateTime;
use clap::{Args, Parser, Subcommand};
use cli_vehicle::VehicleArgs;
use miette::{miette, IntoDiagnostic, WrapErr};
use serde::{Deserialize, Serialize};
use teslatte::auth::{AccessToken, Authentication, RefreshToken};
use teslatte::calendar_history::{CalendarHistoryValues, HistoryKind, HistoryPeriod};
use teslatte::energy::EnergySiteId;
use teslatte::powerwall::{PowerwallEnergyHistoryValues, PowerwallId};
use teslatte::vehicles::{SetChargeLimit, SetChargingAmps};
use teslatte::{Api, VehicleId};
use tracing_subscriber::util::SubscriberInitExt;

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
    /// List of vehicles.
    Vehicles,

    /// Specific Vehicle.
    Vehicle(VehicleArgs),

    /// List of energy sites.
    EnergySites,

    /// Specific energy site.
    EnergySite(EnergySiteArgs),

    /// Powerwall queries.
    Powerwall(PowerwallArgs),
}

#[derive(Debug, Args)]
struct EnergySiteArgs {
    pub id: EnergySiteId,

    #[clap(subcommand)]
    pub command: EnergySiteCommand,
}

impl EnergySiteArgs {
    pub async fn run(&self, api: &Api) -> miette::Result<()> {
        match &self.command {
            EnergySiteCommand::CalendarHistory(args) => {
                let start_date = args
                    .start
                    .as_ref()
                    .map(|s| DateTime::parse_from_rfc3339(&s).into_diagnostic())
                    .transpose()
                    .wrap_err("start_date")?;
                let end_date = args
                    .end
                    .as_ref()
                    .map(|s| DateTime::parse_from_rfc3339(&s).into_diagnostic())
                    .transpose()
                    .wrap_err("end_date")?;
                let values = CalendarHistoryValues {
                    site_id: self.id.clone(),
                    kind: args.kind.clone(),
                    period: args.period.clone(),
                    start_date,
                    end_date,
                };
                let history = api.energy_sites_calendar_history(&values).await?;
                println!("{:#?}", history);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Subcommand)]
enum EnergySiteCommand {
    CalendarHistory(CalendarHistoryArgs),
}

#[derive(Debug, Args)]
struct CalendarHistoryArgs {
    pub kind: HistoryKind,

    #[clap(short, long, default_value = "day")]
    pub period: HistoryPeriod,

    #[clap(short, long)]
    start: Option<String>,

    #[clap(short, long)]
    end: Option<String>,
}

#[derive(Debug, Args)]
struct PowerwallArgs {
    pub id: PowerwallId,

    #[clap(subcommand)]
    pub command: PowerwallCommand,
}

impl PowerwallArgs {
    pub async fn run(&self, api: &Api) -> miette::Result<()> {
        match self.command {
            PowerwallCommand::Status => {
                dbg!(api.powerwall_status(&self.id).await?);
            }
            PowerwallCommand::History => {
                dbg!(
                    api.powerwall_energy_history(&PowerwallEnergyHistoryValues {
                        powerwall_id: self.id.clone(),
                        period: HistoryPeriod::Day,
                        kind: HistoryKind::Power,
                        start_date: None,
                        end_date: None
                    })
                    .await?
                );
            }
        }
        Ok(())
    }
}

#[derive(Debug, Subcommand)]
enum PowerwallCommand {
    /// Show the status of the Powerwall.
    Status,

    History,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();

    match args.command {
        Command::Auth { save } => {
            let auth = Authentication::new()?;
            let (access_token, refresh_token) = auth.interactive_get_access_token().await?;
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

            let auth = Authentication::new()?;
            let response = auth.refresh_access_token(&refresh_token).await?;
            updated_tokens(save, response.access_token, refresh_token);
        }
        Command::Api(api_args) => {
            let access_token = match &api_args.access_token {
                Some(a) => a.clone(),
                None => {
                    let config = Config::load();
                    config.access_token.clone()
                }
            };

            let api = Api::new(&access_token);
            match api_args.command {
                ApiCommand::Vehicles => {
                    dbg!(api.vehicles().await?);
                }
                ApiCommand::Vehicle(v) => {
                    v.run(&api).await?;
                }
                ApiCommand::EnergySites => {
                    dbg!(api.energy_sites().await?);
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
