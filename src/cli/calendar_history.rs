use crate::calendar_history::{HistoryKind, HistoryPeriod};
use clap::Args;

#[derive(Debug, Args)]
pub struct CalendarHistoryArgs {
    pub kind: HistoryKind,

    #[clap(short, long, default_value = "day")]
    pub period: HistoryPeriod,

    #[clap(short, long)]
    pub start: Option<String>,

    #[clap(short, long)]
    pub end: Option<String>,
}
