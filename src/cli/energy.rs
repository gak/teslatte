use crate::calendar_history::CalendarHistoryValues;
use crate::cli::calendar_history::CalendarHistoryArgs;
use crate::energy::EnergySiteId;
use crate::Api;
use chrono::DateTime;
use clap::{Args, Subcommand};
use miette::{IntoDiagnostic, WrapErr};

#[derive(Debug, Subcommand)]
pub enum EnergySiteCommand {
    CalendarHistory(CalendarHistoryArgs),
}

#[derive(Debug, Args)]
pub struct EnergySiteArgs {
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
                    .map(|s| DateTime::parse_from_rfc3339(s).into_diagnostic())
                    .transpose()
                    .wrap_err("start_date")?;
                let end_date = args
                    .end
                    .as_ref()
                    .map(|s| DateTime::parse_from_rfc3339(s).into_diagnostic())
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
