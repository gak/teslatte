use crate::calendar_history::{HistoryKind, HistoryPeriod};
use crate::energy::GatewayId;
use crate::error::TeslatteError;
use crate::vehicles::VehicleData;
use crate::{
    get, get_arg, get_args, join_query_pairs, post_arg, post_arg_empty, rfc3339, Api, Empty,
    ExternalVehicleId, Values, VehicleId,
};
use chrono::{DateTime, FixedOffset};
use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[rustfmt::skip]
impl Api {
    get_arg!(powerwall_status, PowerwallStatus, "/powerwalls/{}/status", PowerwallId);
    get_args!(powerwall_energy_history, PowerwallEnergyHistory, "/powerwalls/{}/energyhistory", PowerwallEnergyHistoryValues);
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, FromStr)]
pub struct PowerwallId(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct PowerwallStatus {
    pub site_name: String,
    pub id: GatewayId,
    pub energy_left: f64,
    pub total_pack_energy: i64,
    pub percentage_charged: f64,
    pub battery_power: i64,
}

#[derive(Debug, Clone)]
pub struct PowerwallEnergyHistoryValues {
    pub powerwall_id: PowerwallId,
    pub period: HistoryPeriod,
    pub kind: HistoryKind,
    pub start_date: Option<DateTime<FixedOffset>>,
    pub end_date: Option<DateTime<FixedOffset>>,
}

impl Values for PowerwallEnergyHistoryValues {
    fn format(&self, url: &str) -> String {
        let url = url.replace("{}", &format!("{}", self.powerwall_id.0));
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
pub struct PowerwallEnergyHistory {}
