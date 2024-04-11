use crate::energy_sites::WallConnector;
use crate::error::TeslatteError;
use crate::powerwall::PowerwallId;
use crate::vehicles::VehicleData;
use crate::{pub_get, OwnerApi};
use derive_more::Display;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::str::FromStr;

#[rustfmt::skip]
impl OwnerApi {
    pub_get!(products, Vec<Product>, "/products");
}

#[derive(Debug, Clone, Deserialize, Display)]
pub struct EnergySiteId(pub u64);

impl FromStr for EnergySiteId {
    type Err = TeslatteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EnergySiteId(s.parse().map_err(|_| {
            TeslatteError::DecodeEnergySiteIdError(s.to_string())
        })?))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayId(String);

#[derive(Debug, Clone)]
pub enum Product {
    Vehicle(Box<VehicleData>),
    Solar(Box<SolarData>),
    Powerwall(Box<PowerwallData>),
}

fn deserialize_product<'de, D>(deserializer: D) -> Result<Product, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Value::deserialize(deserializer)?;

    if v.get("vehicle_id").is_some() {
        let vehicle_data = VehicleData::deserialize(v).map_err(serde::de::Error::custom)?;
        Ok(Product::Vehicle(Box::new(vehicle_data)))
    } else if v.get("solar_type").is_some() {
        let solar_data = SolarData::deserialize(v).map_err(serde::de::Error::custom)?;
        Ok(Product::Solar(Box::new(solar_data)))
    } else if v.get("battery_type").is_some() {
        let powerwall_data = PowerwallData::deserialize(v).map_err(serde::de::Error::custom)?;
        Ok(Product::Powerwall(Box::new(powerwall_data)))
    } else {
        Err(serde::de::Error::custom(
            "No valid key found to determine the product type",
        ))
    }
}

impl<'de> Deserialize<'de> for Product {
    fn deserialize<D>(deserializer: D) -> Result<Product, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_product(deserializer)
    }
}

/// This is assumed from https://tesla-api.timdorr.com/api-basics/products
#[derive(Debug, Clone, Deserialize)]
pub struct SolarData {
    pub energy_site_id: EnergySiteId,
    pub solar_type: String,
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
    pub energy_site_id: EnergySiteId,
    pub battery_type: String,
    /// Should always be "battery".
    pub resource_type: String,
    pub site_name: String,
    pub id: PowerwallId,
    pub gateway_id: GatewayId,
    pub asset_site_id: String,
    pub percentage_charged: f64,
    pub backup_capable: bool,
    pub battery_power: i64,
    pub sync_grid_alert_enabled: bool,
    pub breaker_alert_enabled: bool,
    pub components: Components,
    // New fields as of 2024-01-20
    pub powerwall_onboarding_settings_set: bool,
    pub storm_mode_enabled: bool,
    pub features: PowerwallFeatures,
    pub warp_site_number: String,
    pub go_off_grid_test_banner_enabled: Option<bool>,
    pub powerwall_tesla_electric_interested_in: Option<bool>,
    pub vpp_tour_enabled: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PowerwallFeatures {
    pub rate_plan_manager_no_pricing_constraint: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Components {
    pub battery: bool,
    pub battery_type: Option<String>,
    pub solar: bool,
    pub solar_type: Option<String>,
    pub grid: bool,
    pub load_meter: bool,
    pub market_type: Option<String>,
    #[serde(default)]
    pub wall_connectors: Vec<WallConnector>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::energy_sites::{CalendarHistoryValues, HistoryKind, HistoryPeriod};
    use crate::{ApiValues, PrintResponses, RequestData};
    use chrono::DateTime;

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
        let energy_site: Product = serde_json::from_str(json).unwrap();
        if let Product::Vehicle(v) = energy_site {
            assert_eq!(v.id.0, 1111193485934);
            assert_eq!(v.user_id, 2222291283912);
            assert_eq!(v.vehicle_id.0, 333331238921);
            assert_eq!(v.vin, "T234567890123456789");
            assert_eq!(v.display_name.unwrap(), "My Vehicle");
            assert_eq!(v.option_codes.unwrap(), "ASDF,SDFG,DFGH");
            assert_eq!(v.color, None);
            assert_eq!(v.access_type, "OWNER");
            assert_eq!(v.tokens, vec!["asdf1234"]);
            assert_eq!(v.state, "online");
            assert!(!v.in_service);
            assert!(v.calendar_enabled);
            assert_eq!(v.api_version, 42);
            assert_eq!(v.backseat_token, None);
            assert_eq!(v.backseat_token_updated_at, None);
            assert_eq!(
                v.vehicle_config.unwrap().aux_park_lamps,
                Some("Eu".to_string())
            );
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

    #[test]
    fn calendar_history_values_dates() {
        let v = CalendarHistoryValues {
            site_id: EnergySiteId(123),
            period: HistoryPeriod::Month,
            kind: HistoryKind::Energy,
            start_date: Some(DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z").unwrap()),
            end_date: Some(DateTime::parse_from_rfc3339("2020-01-31T23:59:59Z").unwrap()),
        };
        let url = v.format("https://base.com/e/{}/history");
        assert_eq!(
            url,
            "https://base.com/e/123/history?period=month&kind=energy&start_date=2020-01-01T00:00:00Z&end_date=2020-01-31T23:59:59Z"
        );
    }

    #[test]
    fn json_products_gak_2024_01_20() {
        let s = include_str!("../testdata/products_gak_2024_01_20.json");
        let request_data = RequestData::Get { url: "" };
        OwnerApi::parse_json::<Vec<Product>>(&request_data, s.to_string(), PrintResponses::Pretty)
            .unwrap();
    }

    #[test]
    fn json_products_gak_2024_04_12() {
        let s = include_str!("../testdata/products_gak_2024_04_12.json");
        let request_data = RequestData::Get { url: "" };
        OwnerApi::parse_json::<Vec<Product>>(&request_data, s.to_string(), PrintResponses::Pretty)
            .unwrap();
    }
}
