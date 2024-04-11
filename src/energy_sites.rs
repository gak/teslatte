use crate::products::EnergySiteId;
use crate::{join_query_pairs, pub_get_arg, pub_get_args, rfc3339, ApiValues, OwnerApi};
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use strum::{Display, EnumString, IntoStaticStr};

#[rustfmt::skip]
impl OwnerApi {
    pub_get_arg!(energy_sites_site_status, SiteStatus, "/energy_sites/{}/site_status", EnergySiteId);
    pub_get_arg!(energy_sites_live_status, LiveStatus, "/energy_sites/{}/live_status", EnergySiteId);
    pub_get_arg!(energy_sites_site_info, SiteInfo, "/energy_sites/{}/site_info", EnergySiteId);
    pub_get_args!(energy_sites_calendar_history, CalendarHistory, "/energy_sites/{}/calendar_history", CalendarHistoryValues);
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SiteStatus {
    pub backup_capable: bool,
    pub battery_power: i64,
    pub battery_type: String,
    pub breaker_alert_enabled: bool,
    pub gateway_id: String,
    pub percentage_charged: f64,
    pub powerwall_onboarding_settings_set: bool,
    // TODO: Unknown type. Was null.
    pub powerwall_tesla_electric_interested_in: Option<()>,
    // battery
    pub resource_type: String,
    pub site_name: String,
    pub storm_mode_enabled: bool,
    pub sync_grid_alert_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct LiveStatus {
    pub backup_capable: bool,
    pub battery_power: i64,
    pub generator_power: i64,
    pub grid_power: i64,
    pub grid_services_active: bool,
    pub grid_services_power: i64,
    pub grid_status: String,
    pub island_status: String,
    pub load_power: i64,
    pub percentage_charged: f64,
    pub solar_power: i64,
    pub storm_mode_active: bool,
    pub timestamp: String,
    pub total_pack_energy: i64,
    pub wall_connectors: Vec<LiveWallConnector>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct WallConnector {
    pub device_id: String,
    pub din: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct LiveWallConnector {
    /// VIN of the car when connected to the wall connector.
    pub vin: Option<String>,
    pub din: String,

    /// Observed:
    /// 2: not plugged in
    /// 4: plugged in (not charging)
    pub wall_connector_state: i64,
    pub wall_connector_power: f32,
    pub wall_connector_fault_state: i64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct UserSettings {
    pub breaker_alert_enabled: bool,
    pub powerwall_onboarding_settings_set: bool,
    pub powerwall_tesla_electric_interested_in: bool,
    pub storm_mode_enabled: bool,
    pub sync_grid_alert_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Schedule {
    pub end_seconds: i64,
    pub start_seconds: i64,
    pub target: String,
    pub week_days: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TouSettings {
    pub optimization_strategy: String,
    pub schedule: Vec<Schedule>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Geolocation {
    pub latitude: f64,
    pub longitude: f64,
    pub source: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Components {
    pub backup: bool,
    pub backup_time_remaining_enabled: bool,
    pub battery: bool,
    pub battery_solar_offset_view_enabled: bool,
    pub battery_type: String,
    pub car_charging_data_supported: bool,
    pub configurable: bool,
    pub edit_setting_energy_exports: bool,
    pub edit_setting_grid_charging: bool,
    pub edit_setting_permission_to_export: bool,
    pub energy_service_self_scheduling_enabled: bool,
    pub energy_value_header: String,
    pub energy_value_subheader: String,
    pub flex_energy_request_capable: bool,
    pub gateway: String,
    pub grid: bool,
    pub grid_services_enabled: bool,
    pub load_meter: bool,
    pub off_grid_vehicle_charging_reserve_supported: bool,
    pub set_islanding_mode_enabled: bool,
    pub show_grid_import_battery_source_cards: bool,
    pub solar: bool,
    pub solar_type: String,
    pub solar_value_enabled: bool,
    pub storm_mode_capable: bool,
    pub tou_capable: bool,
    pub vehicle_charging_performance_view_enabled: bool,
    pub vehicle_charging_solar_offset_view_enabled: bool,
    pub wifi_commissioning_enabled: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Address {
    pub address_line1: String,
    pub city: String,
    pub country: String,
    pub state: String,
    pub zip: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SiteInfo {
    pub address: Address,
    pub backup_reserve_percent: i64,
    pub battery_count: i64,
    pub components: Components,
    pub default_real_mode: String,
    pub geolocation: Geolocation,
    pub id: String,
    pub installation_date: String,
    pub installation_time_zone: String,
    pub max_site_meter_power_ac: i64,
    pub min_site_meter_power_ac: i64,
    pub nameplate_energy: i64,
    pub nameplate_power: i64,
    pub site_name: String,
    pub tou_settings: TouSettings,
    pub user_settings: UserSettings,
    pub version: String,
    pub vpp_backup_reserve_percent: i64,
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

impl ApiValues for CalendarHistoryValues {
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

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CalendarHistory {
    pub serial_number: String,
    /// Only appears in energy kind.
    pub period: Option<String>,
    pub installation_time_zone: String,
    /// Optional because if there are no `Series` fields, this field is omitted.
    pub time_series: Option<Vec<Series>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Series {
    Power(PowerSeries),
    Energy(EnergySeries),
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PowerSeries {
    pub timestamp: DateTime<FixedOffset>,
    pub solar_power: f64,
    pub battery_power: f64,
    pub grid_power: f64,
    pub grid_services_power: f64,
    pub generator_power: f64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::energy_sites::{LiveStatus, LiveWallConnector, WallConnector};
    use crate::products::Product;
    use crate::{OwnerApi, PrintResponses, RequestData};

    #[test]
    fn json_energy_sites_gak_2023_11_06() {
        let s = include_str!("../testdata/energy_sites_gak_2023_11_06.json");

        let request_data = RequestData::Get { url: "" };
        let products = OwnerApi::parse_json::<Vec<Product>>(
            &request_data,
            s.to_string(),
            PrintResponses::Pretty,
        )
        .unwrap();

        let Product::Powerwall(powerwall) = &products[1] else {
            panic!("Expected Powerwall product");
        };
        assert_eq!(
            powerwall.components.wall_connectors[0],
            WallConnector {
                device_id: "uuid".to_string(),
                din: "12345".to_string(),
                is_active: true,
            }
        )
    }

    #[test]
    fn json_energy_sites_live_power_gak_2023_11_06() {
        let s = include_str!("../testdata/energy_sites_live_status_gak_plugged_in_2023_11_06.json");

        let request_data = RequestData::Get { url: "" };
        let live_status = OwnerApi::parse_json::<LiveStatus>(
            &request_data,
            s.to_string(),
            PrintResponses::Pretty,
        )
        .unwrap();

        let wall_connector = live_status.wall_connectors.first().unwrap();
        assert_eq!(
            wall_connector,
            &LiveWallConnector {
                vin: Some("1234".to_string()),
                din: "5432".to_string(),
                wall_connector_state: 4,
                wall_connector_power: 0.,
                wall_connector_fault_state: 2,
            }
        );
    }

    #[test]
    fn json_energy_sites_live_power_unplugged_charger_gak_2023_11_06() {
        let s = include_str!("../testdata/energy_sites_live_status_gak_unplugged_2023_11_06.json");

        let request_data = RequestData::Get { url: "" };
        let live_status = OwnerApi::parse_json::<LiveStatus>(
            &request_data,
            s.to_string(),
            PrintResponses::Pretty,
        )
        .unwrap();

        let wall_connector = live_status.wall_connectors.first().unwrap();
        assert_eq!(
            wall_connector,
            &LiveWallConnector {
                vin: None,
                din: "1234".to_string(),
                wall_connector_state: 2,
                wall_connector_power: 0.,
                wall_connector_fault_state: 2,
            }
        );
    }
}
