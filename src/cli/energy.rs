use crate::calendar_history::{CalendarHistoryValues, HistoryKind, HistoryPeriod};
use crate::cli::print_json;
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
                print_json(api.energy_sites_calendar_history(&values).await?);
            }
        }
        Ok(())
    }
}

/// Show the calendar history of an energy site. This is the same data that is shown in the Tesla app.
///
/// Use `energy_site_id` as the ID.
///
/// The `kind` argument must be `energy` or `power`.
///
/// Example:
///
/// teslatte api energy-site 1234567890 calendar-history power -s "2022-01-01T00:00:00Z" -e 2023-01-01T00:00:00Z -p month
#[derive(Debug, Args)]
pub struct CalendarHistoryArgs {
    /// `energy` or `power`
    pub kind: HistoryKind,

    #[clap(short, long, default_value = "day")]
    pub period: HistoryPeriod,

    /// ISO8601 date-time for the start of the period, e.g. 2000-01-01T00:00:00Z
    #[clap(short, long)]
    pub start: Option<String>,

    /// ISO8601 date-time for the end of the period, e.g. 2025-01-01T00:00:00Z
    #[clap(short, long)]
    pub end: Option<String>,
}
