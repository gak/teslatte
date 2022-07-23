use crate::error::TeslatteError;
use crate::powerwall::PowerwallId;
use crate::vehicles::VehicleData;
use crate::{
    get, get_arg, get_args, post_arg, post_arg_empty, Api, Empty, ExternalVehicleId, VehicleId,
};
use serde::{Deserialize, Serialize};
use url::Url;

#[rustfmt::skip]
impl Api {
    get!(energy_sites, Vec<EnergySite>, "/products");
    // https://owner-api.teslamotors.com/api/1/energy_sites/1370797147/calendar_history?period=day&kind=power
    get_args!(energy_sites_calendar_history, CalendarHistory, "/energy_sites/{}/calendar_history", CalendarHistoryValues);
}

#[derive(Debug, Clone, Deserialize)]
pub struct CalendarHistory {}

trait Values {
    fn format(&self, url: &str) -> String;
}

#[derive(Debug, Clone, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum HistoryKind {
    Power,
    Energy,
}

#[derive(Debug, Clone, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum HistoryPeriod {
    Day,
    Week,
    Month,
    Year,
}

pub struct CalendarHistoryValues {
    site_id: EnergySiteId,
    period: HistoryPeriod,
    kind: HistoryKind,
    start_date: Option<chrono::DateTime<chrono::Utc>>,
    end_date: Option<chrono::DateTime<chrono::Utc>>,
}

impl Values for CalendarHistoryValues {
    fn format(&self, url: &str) -> String {
        let url = url.replace("{}", &format!("{}", self.site_id.0));
        let mut url = Url::parse(&url).unwrap();
        let mut pairs = url.query_pairs_mut();
        pairs.append_pair("period", &self.period.to_string());
        pairs.append_pair("kind", &self.kind.to_string());
        if let Some(start_date) = self.start_date {
            pairs.append_pair("start_date", &start_date.to_rfc3339());
        }
        if let Some(end_date) = self.end_date {
            pairs.append_pair("end_date", &end_date.to_rfc3339());
        }
        drop(pairs);
        url.to_string()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnergySiteId(u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayId(String);

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum EnergySite {
    Vehicle(VehicleData),
    Solar(SolarData),
    Powerwall(PowerwallData),
}

/// This is assumed from https://tesla-api.timdorr.com/api-basics/products
#[derive(Debug, Clone, Deserialize)]
pub struct SolarData {
    // `solar_type` must be first in the struct so serde can properly decode.
    pub solar_type: String,
    pub energy_site_id: EnergySiteId,
    /// Should always be "solar".
    pub resource_type: String,
    pub id: String,
    pub asset_site_id: String,
    pub solar_power: i64,
    pub sync_grid_alert_enabled: bool,
    pub breaker_alert_enabled: bool,
    pub components: Components,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PowerwallData {
    // `battery_type` must be first in the struct so serde can properly decode.
    pub battery_type: String,
    pub energy_site_id: i64,
    /// Should always be "battery".
    pub resource_type: String,
    pub site_name: String,
    pub id: PowerwallId,
    pub gateway_id: GatewayId,
    pub asset_site_id: String,
    pub energy_left: f64,
    pub total_pack_energy: i64,
    pub percentage_charged: f64,
    pub backup_capable: bool,
    pub battery_power: i64,
    pub sync_grid_alert_enabled: bool,
    pub breaker_alert_enabled: bool,
    pub components: Components,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Components {
    pub battery: bool,
    pub battery_type: Option<String>,
    pub solar: bool,
    pub solar_type: Option<String>,
    pub grid: bool,
    pub load_meter: bool,
    pub market_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn energy_match_powerwall() {
        let json = r#"
        {
          "energy_site_id": 1032748243,
          "resource_type": "battery",
          "site_name": "1 Railway Pde",
          "id": "ABC2010-1234",
          "gateway_id": "3287423824-QWE",
          "asset_site_id": "123ecd-123ecd-12345-12345",
          "energy_left": 4394.000000000001,
          "total_pack_energy": 13494,
          "percentage_charged": 32.562620423892106,
          "battery_type": "ac_powerwall",
          "backup_capable": true,
          "battery_power": -280,
          "sync_grid_alert_enabled": true,
          "breaker_alert_enabled": false,
          "components": {
            "battery": true,
            "battery_type": "ac_powerwall",
            "solar": true,
            "solar_type": "pv_panel",
            "grid": true,
            "load_meter": true,
            "market_type": "residential"
          }
        }
        "#;

        if let EnergySite::Powerwall(data) = serde_json::from_str(json).unwrap() {
            assert_eq!(data.battery_type, "ac_powerwall");
            assert_eq!(data.backup_capable, true);
            assert_eq!(data.battery_power, -280);
            assert_eq!(data.sync_grid_alert_enabled, true);
            assert_eq!(data.breaker_alert_enabled, false);
            assert_eq!(data.components.battery, true);
            assert_eq!(
                data.components.battery_type,
                Some("ac_powerwall".to_string())
            );
            assert_eq!(data.components.solar, true);
            assert_eq!(data.components.solar_type, Some("pv_panel".to_string()));
            assert_eq!(data.components.grid, true);
            assert_eq!(data.components.load_meter, true);
            assert_eq!(data.components.market_type, "residential");
        } else {
            panic!("Expected PowerwallData");
        }
    }

    #[test]
    fn energy_match_vehicle() {
        let json = r#"
          {
            "id": 1111193485934,
            "user_id": 2222291283912,
            "vehicle_id": 333331238921,
            "vin": "T234567890123456789",
            "display_name": "My Vehicle",
            "option_codes": "ASDF,SDFG,DFGH",
            "color": null,
            "access_type": "OWNER",
            "tokens": [
              "asdf1234"
            ],
            "state": "online",
            "in_service": false,
            "id_s": "932423",
            "calendar_enabled": true,
            "api_version": 42,
            "backseat_token": null,
            "backseat_token_updated_at": null,
            "vehicle_config": {
              "aux_park_lamps": "Eu",
              "badge_version": 0,
              "can_accept_navigation_requests": true,
              "can_actuate_trunks": true,
              "car_special_type": "base",
              "car_type": "model3",
              "charge_port_type": "CCS",
              "dashcam_clip_save_supported": true,
              "default_charge_to_max": false,
              "driver_assist": "TeslaAP3",
              "ece_restrictions": false,
              "efficiency_package": "M32026",
              "eu_vehicle": true,
              "exterior_color": "MidnightSilver",
              "exterior_trim": "Black",
              "exterior_trim_override": "",
              "has_air_suspension": false,
              "has_ludicrous_mode": false,
              "has_seat_cooling": false,
              "headlamp_type": "Global",
              "interior_trim_type": "Black2",
              "key_version": 2,
              "motorized_charge_port": true,
              "paint_color_override": "255,200,253,0.9,0.3",
              "performance_package": "Base",
              "plg": true,
              "pws": false,
              "rear_drive_unit": "T15232Z",
              "rear_seat_heaters": 1,
              "rear_seat_type": 0,
              "rhd": true,
              "roof_color": "RoofColorGlass",
              "seat_type": null,
              "spoiler_type": "None",
              "sun_roof_installed": null,
              "supports_qr_pairing": false,
              "third_row_seats": "None",
              "timestamp": 1658390117642,
              "trim_badging": "9",
              "use_range_badging": true,
              "utc_offset": 0,
              "webcam_supported": false,
              "wheel_type": "StilettoRefresh19"
            },
            "command_signing": "allowed"
          }
        "#;
        let energy_site: EnergySite = serde_json::from_str(json).unwrap();
        if let EnergySite::Vehicle(v) = energy_site {
            assert_eq!(v.id.0, 1111193485934);
            assert_eq!(v.user_id, 2222291283912);
            assert_eq!(v.vehicle_id.0, 333331238921);
            assert_eq!(v.vin, "T234567890123456789");
            assert_eq!(v.display_name, "My Vehicle");
            assert_eq!(v.option_codes, "ASDF,SDFG,DFGH");
            assert_eq!(v.color, None);
            assert_eq!(v.access_type, "OWNER");
            assert_eq!(v.tokens, vec!["asdf1234"]);
            assert_eq!(v.state, "online");
            assert_eq!(v.in_service, false);
            assert_eq!(v.calendar_enabled, true);
            assert_eq!(v.api_version, 42);
            assert_eq!(v.backseat_token, None);
            assert_eq!(v.backseat_token_updated_at, None);
            assert_eq!(v.vehicle_config.unwrap().aux_park_lamps, "Eu");
        } else {
            panic!("Wrong EnergySite");
        }
    }

    #[test]
    fn calendar_history_values() {
        let v = CalendarHistoryValues {
            site_id: EnergySiteId(123),
            period: HistoryPeriod::Month,
            kind: HistoryKind::Energy,
            start_date: None,
            end_date: None,
        };
        let url = v.format("https://base.com/e/{}/history");
        assert_eq!(
            url,
            "https://base.com/e/123/history?period=month&kind=energy"
        );
    }
}
