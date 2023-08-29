use crate::calendar_history::{HistoryKind, HistoryPeriod};
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
