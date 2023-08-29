use crate::energy::EnergySiteId;
use crate::Values;
use crate::{get_args, join_query_pairs, rfc3339, Api};
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use strum::{Display, EnumString, IntoStaticStr};

#[rustfmt::skip]
impl Api {
    get_args!(energy_sites_calendar_history, CalendarHistory, "/energy_sites/{}/calendar_history", CalendarHistoryValues);
}

#[derive(Debug, Clone, Display, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum HistoryKind {
    Power,
    Energy,
}

#[derive(Debug, Clone, Display, EnumString, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum HistoryPeriod {
    Day,
    Month,
    Year,
    Lifetime,
}

pub struct CalendarHistoryValues {
    // Modify URL:
    pub site_id: EnergySiteId,

    // Query params:
    pub period: HistoryPeriod,
    pub kind: HistoryKind,
    pub start_date: Option<DateTime<FixedOffset>>,
    pub end_date: Option<DateTime<FixedOffset>>,
}

impl Values for CalendarHistoryValues {
    fn format(&self, url: &str) -> String {
        let url = url.replace("{}", &format!("{}", self.site_id.0));
        let mut pairs: Vec<(&str, String)> = vec![
            ("period", self.period.to_string()),
            ("kind", self.kind.to_string()),
        ];
        if let Some(start_date) = self.start_date {
            let start_date = rfc3339(&start_date);
            pairs.push(("start_date", start_date));
        }
        if let Some(end_date) = self.end_date {
            let end_date = rfc3339(&end_date);
            pairs.push(("end_date", end_date));
        }
        format!("{}?{}", url, join_query_pairs(&pairs))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalendarHistory {
    pub serial_number: String,
    /// Only appears in energy kind.
    pub period: Option<String>,
    pub installation_time_zone: String,
    /// Optional because if there are no `Series` fields, this field is omitted.
    pub time_series: Option<Vec<Series>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Series {
    Power(PowerSeries),
    Energy(EnergySeries),
}

#[derive(Debug, Clone, Deserialize)]
pub struct PowerSeries {
    pub timestamp: DateTime<FixedOffset>,
    pub solar_power: f64,
    pub battery_power: f64,
    pub grid_power: f64,
    pub grid_services_power: f64,
    pub generator_power: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnergySeries {
    pub timestamp: DateTime<FixedOffset>,
    pub solar_energy_exported: f64,
    pub generator_energy_exported: f64,
    pub grid_energy_imported: f64,
    pub grid_services_energy_imported: f64,
    pub grid_services_energy_exported: f64,
    pub grid_energy_exported_from_solar: f64,
    pub grid_energy_exported_from_generator: f64,
    pub grid_energy_exported_from_battery: f64,
    pub battery_energy_exported: f64,
    pub battery_energy_imported_from_grid: f64,
    pub battery_energy_imported_from_solar: f64,
    pub battery_energy_imported_from_generator: f64,
    pub consumer_energy_imported_from_grid: f64,
    pub consumer_energy_imported_from_solar: f64,
    pub consumer_energy_imported_from_battery: f64,
    pub consumer_energy_imported_from_generator: f64,
}
