use crate::calendar_history::{HistoryKind, HistoryPeriod};
use crate::cli::print_json_data;
use crate::powerwall::{PowerwallEnergyHistoryValues, PowerwallId};
use crate::Api;
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum PowerwallCommand {
    /// Show the status of the Powerwall.
    Status,

    History,
}

#[derive(Debug, Args)]
pub struct PowerwallArgs {
    pub id: PowerwallId,

    #[clap(subcommand)]
    pub command: PowerwallCommand,
}

impl PowerwallArgs {
    pub async fn run(&self, api: &Api) -> miette::Result<()> {
        match self.command {
            PowerwallCommand::Status => {
                print_json_data(api.powerwall_status(&self.id).await?);
            }
            PowerwallCommand::History => {
                print_json_data(
                    api.powerwall_energy_history(&PowerwallEnergyHistoryValues {
                        powerwall_id: self.id.clone(),
                        period: HistoryPeriod::Day,
                        kind: HistoryKind::Power,
                        start_date: None,
                        end_date: None,
                    })
                    .await?,
                );
            }
        }
        Ok(())
    }
}
