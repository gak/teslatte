use crate::energy::GatewayId;
use crate::error::TeslatteError;
use crate::vehicles::VehicleData;
use crate::{get, get_arg, post_arg, post_arg_empty, Api, Empty, ExternalVehicleId, VehicleId};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[rustfmt::skip]
impl Api {
    get_arg!(powerwall_status, PowerwallStatus, "/powerwalls/{}/status", PowerwallId);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerwallId(pub String);

impl Display for PowerwallId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PowerwallId {
    type Err = TeslatteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PowerwallId(s.to_string()))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PowerwallStatus {
    pub site_name: String,
    pub id: GatewayId,
    pub energy_left: f64,
    pub total_pack_energy: i64,
    pub percentage_charged: f64,
    pub battery_power: i64,
}
