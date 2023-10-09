/// Please note that many of these structs are generated from my own API call responses.
///
/// Sometimes the API will return a null for a field where I've put in a non Option type, which
/// will cause the deserializer to fail. Please log an issue to fix these if you come across it.
use crate::{get, get_arg, post_arg, post_arg_empty, Api, Empty, ExternalVehicleId, VehicleId};
use serde::{Deserialize, Serialize};

#[rustfmt::skip]
impl Api {
    get!(vehicles, Vec<Vehicle>, "/vehicles");
    get_arg!(vehicle_data, VehicleData, "/vehicles/{}/vehicle_data", VehicleId);

    // Alerts
    post_arg_empty!(honk_horn, "/vehicles/{}/command/honk_horn", VehicleId);
    post_arg_empty!(flash_lights, "/vehicles/{}/command/flash_lights", VehicleId);

    // Charging
    post_arg_empty!(charge_port_door_open, "/vehicles/{}/command/charge_port_door_open", VehicleId);
    post_arg_empty!(charge_port_door_close, "/vehicles/{}/command/charge_port_door_close", VehicleId);
    post_arg!(set_charge_limit, SetChargeLimit, "/vehicles/{}/command/set_charge_limit", VehicleId);
    post_arg!(set_charging_amps, SetChargingAmps, "/vehicles/{}/command/set_charging_amps", VehicleId);
    post_arg_empty!(charge_standard, "/vehicles/{}/command/charge_standard", VehicleId);
    post_arg_empty!(charge_max_range, "/vehicles/{}/command/charge_max_range", VehicleId);
    post_arg_empty!(charge_start, "/vehicles/{}/command/charge_start", VehicleId);
    post_arg_empty!(charge_stop, "/vehicles/{}/command/charge_stop", VehicleId);
    post_arg!(set_scheduled_charging, SetScheduledCharging, "/vehicles/{}/command/set_scheduled_charging", VehicleId);
    post_arg!(set_scheduled_departure, SetScheduledDeparture, "/vehicles/{}/command/set_scheduled_departure", VehicleId);
}

#[derive(Debug, Clone, Deserialize)]
pub struct VehicleData {
    pub id: VehicleId,
    pub vehicle_id: ExternalVehicleId,
    pub user_id: i64,
    pub vin: String,
    pub display_name: Option<String>,
    pub option_codes: Option<String>,
    /// gak: This was null for me, assuming String.
    pub color: Option<String>,
    pub access_type: String,
    pub granular_access: Option<GranularAccess>,
    pub tokens: Vec<String>,
    pub state: String,
    pub in_service: bool,
    pub id_s: String,
    pub calendar_enabled: bool,
    pub api_version: i64,
    /// gak: This was null for me, assuming String.
    pub backseat_token: Option<String>,
    /// gak: This was null for me, assuming String.
    pub backseat_token_updated_at: Option<String>,
    pub ble_autopair_enrolled: Option<bool>,

    /// gak: Some of these have been null for me, so making them all Option.
    pub charge_state: Option<ChargeState>,
    pub climate_state: Option<ClimateState>,
    pub drive_state: Option<DriveState>,
    pub gui_settings: Option<GuiSettings>,
    pub vehicle_config: Option<VehicleConfig>,
    pub vehicle_state: Option<VehicleState>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChargeState {
    pub battery_heater_on: bool,
    pub battery_level: i64,
    pub battery_range: f64,
    pub charge_amps: i64,
    pub charge_current_request: i64,
    pub charge_current_request_max: i64,
    pub charge_enable_request: bool,
    pub charge_energy_added: f64,
    pub charge_limit_soc: i64,
    pub charge_limit_soc_max: i64,
    pub charge_limit_soc_min: i64,
    pub charge_limit_soc_std: i64,
    pub charge_miles_added_ideal: f64,
    pub charge_miles_added_rated: f64,
    pub charge_port_cold_weather_mode: Option<bool>,
    pub charge_port_color: String,
    pub charge_port_door_open: Option<bool>,
    pub charge_port_latch: String,
    pub charge_rate: f64,
    pub charge_to_max_range: Option<bool>,
    pub charger_actual_current: Option<i64>,
    pub charger_phases: Option<i64>,
    pub charger_pilot_current: Option<i64>,
    pub charger_power: Option<i64>,
    pub charger_voltage: Option<i64>,
    pub charging_state: String,
    pub conn_charge_cable: String,
    pub est_battery_range: f64,
    pub fast_charger_brand: String,
    pub fast_charger_present: Option<bool>,
    pub fast_charger_type: String,
    pub ideal_battery_range: f64,
    pub managed_charging_active: Option<bool>,
    pub managed_charging_start_time: Option<u64>,
    pub managed_charging_user_canceled: Option<bool>,
    pub max_range_charge_counter: i64,
    pub minutes_to_full_charge: i64,
    pub not_enough_power_to_heat: Option<bool>,
    pub off_peak_charging_enabled: bool,
    pub off_peak_charging_times: String,
    pub off_peak_hours_end_time: i64,
    pub preconditioning_enabled: bool,
    pub preconditioning_times: String,
    pub scheduled_charging_mode: String,
    pub scheduled_charging_pending: bool,
    pub scheduled_charging_start_time: Option<i64>,
    pub scheduled_charging_start_time_app: Option<i64>,
    pub scheduled_charging_start_time_minutes: Option<i64>,
    pub scheduled_departure_time: i64,
    pub scheduled_departure_time_minutes: i64,
    pub supercharger_session_trip_planner: bool,
    pub time_to_full_charge: f64,
    pub timestamp: u64,
    pub trip_charging: Option<bool>,
    pub usable_battery_level: i64,
    pub user_charge_enable_request: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateState {
    pub allow_cabin_overheat_protection: bool,
    pub auto_seat_climate_left: Option<bool>,
    pub auto_seat_climate_right: Option<bool>,
    pub battery_heater: bool,
    pub battery_heater_no_power: Option<bool>,
    pub cabin_overheat_protection: String,
    pub cabin_overheat_protection_actively_cooling: Option<bool>,
    pub climate_keeper_mode: String,
    pub defrost_mode: i64,
    pub driver_temp_setting: f64,
    pub fan_status: i64,
    pub hvac_auto_request: String,
    pub inside_temp: f64,
    pub is_auto_conditioning_on: Option<bool>,
    pub is_climate_on: bool,
    pub is_front_defroster_on: bool,
    pub is_preconditioning: bool,
    pub is_rear_defroster_on: bool,
    pub left_temp_direction: Option<i64>,
    pub max_avail_temp: f64,
    pub min_avail_temp: f64,
    pub outside_temp: Option<f64>,
    pub passenger_temp_setting: f64,
    pub remote_heater_control_enabled: bool,
    pub right_temp_direction: Option<i64>,
    pub seat_heater_left: i64,
    pub seat_heater_rear_center: Option<i64>,
    pub seat_heater_rear_left: Option<i64>,
    pub seat_heater_rear_right: Option<i64>,
    pub seat_heater_right: i64,
    pub side_mirror_heaters: bool,
    pub steering_wheel_heater: Option<bool>,
    pub supports_fan_only_cabin_overheat_protection: bool,
    pub timestamp: i64,
    pub wiper_blade_heater: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveState {
    pub gps_as_of: i64,
    pub heading: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub native_latitude: f64,
    pub native_location_supported: i64,
    pub native_longitude: f64,
    pub native_type: String,
    pub power: i64,
    pub shift_state: Option<String>,
    /// gak: I've assumed this to be String.
    pub speed: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiSettings {
    pub gui_24_hour_time: bool,
    pub gui_charge_rate_units: String,
    pub gui_distance_units: String,
    pub gui_range_display: String,
    pub gui_temperature_units: String,
    pub show_range_units: bool,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleConfig {
    pub aux_park_lamps: Option<String>,
    pub badge_version: Option<i64>,
    pub can_accept_navigation_requests: bool,
    pub can_actuate_trunks: bool,
    pub car_special_type: String,
    pub car_type: String,
    pub charge_port_type: String,
    pub dashcam_clip_save_supported: bool,
    pub default_charge_to_max: bool,
    pub driver_assist: String,
    pub ece_restrictions: bool,
    pub efficiency_package: String,
    pub eu_vehicle: bool,
    pub exterior_color: String,
    pub exterior_trim: Option<String>,
    pub exterior_trim_override: Option<String>,
    pub front_drive_unit: Option<String>,
    pub has_air_suspension: bool,
    pub has_ludicrous_mode: bool,
    pub has_seat_cooling: bool,
    pub headlamp_type: String,
    pub interior_trim_type: String,
    pub key_version: Option<i64>,
    pub motorized_charge_port: bool,
    pub paint_color_override: Option<String>,
    pub performance_package: Option<String>,
    pub plg: bool,
    pub pws: bool,
    pub rear_drive_unit: String,
    pub rear_seat_heaters: i64,
    pub rear_seat_type: i64,
    pub rhd: bool,
    pub roof_color: String,
    pub seat_type: Option<u32>,
    pub spoiler_type: String,
    pub sun_roof_installed: Option<u32>,
    pub supports_qr_pairing: Option<bool>,
    pub third_row_seats: String,
    pub timestamp: i64,
    pub trim_badging: String,
    pub use_range_badging: bool,
    pub utc_offset: i64,
    pub webcam_supported: Option<bool>,
    pub wheel_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleState {
    pub api_version: i64,
    pub autopark_state_v2: String,
    pub autopark_style: Option<String>,
    pub calendar_supported: bool,
    pub car_version: String,
    pub center_display_state: i64,
    pub dashcam_clip_save_available: bool,
    pub dashcam_state: String,
    pub df: i64,
    pub dr: i64,
    pub fd_window: i64,
    pub feature_bitmask: String,
    pub fp_window: i64,
    pub ft: i64,
    pub homelink_device_count: Option<i64>,
    pub homelink_nearby: Option<bool>,
    pub is_user_present: bool,
    pub last_autopark_error: Option<String>,
    pub locked: bool,
    pub media_state: MediaState,
    pub notifications_supported: bool,
    pub odometer: f64,
    pub parsed_calendar_supported: bool,
    pub pf: i64,
    pub pr: i64,
    pub rd_window: i64,
    pub remote_start: bool,
    pub remote_start_enabled: bool,
    pub remote_start_supported: bool,
    pub rp_window: i64,
    pub rt: i64,
    pub santa_mode: i64,
    pub sentry_mode: Option<bool>,
    pub sentry_mode_available: Option<bool>,
    pub service_mode: Option<bool>,
    pub service_mode_plus: Option<bool>,
    pub smart_summon_available: Option<bool>,
    pub software_update: SoftwareUpdate,
    pub speed_limit_mode: SpeedLimitMode,
    pub summon_standby_mode_enabled: Option<bool>,
    pub sun_roof_percent_open: Option<i64>,
    pub sun_roof_state: Option<String>,
    pub timestamp: i64,
    pub tpms_pressure_fl: Option<f64>,
    pub tpms_pressure_fr: Option<f64>,
    pub tpms_pressure_rl: Option<f64>,
    pub tpms_pressure_rr: Option<f64>,
    pub valet_mode: bool,
    pub valet_pin_needed: Option<bool>,
    pub vehicle_name: String,
    pub vehicle_self_test_progress: Option<i64>,
    pub vehicle_self_test_requested: Option<bool>,
    pub webcam_available: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaState {
    pub remote_control_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareUpdate {
    pub download_perc: i64,
    pub expected_duration_sec: i64,
    pub install_perc: i64,
    pub status: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedLimitMode {
    pub active: bool,
    pub current_limit_mph: f64,
    pub max_limit_mph: i64,
    pub min_limit_mph: f64,
    pub pin_code_set: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GranularAccess {
    pub hide_private: bool,
}

#[derive(Debug, Deserialize)]
pub struct Vehicles(Vec<Vehicle>);

#[derive(Debug, Deserialize)]
pub struct Vehicle {
    pub id: VehicleId,
    pub vehicle_id: ExternalVehicleId,
    pub vin: String,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct SetChargingAmps {
    pub charging_amps: i64,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct SetChargeLimit {
    // TODO: percent: Percentage,
    pub percent: u8,
}

/// set_scheduled_charging
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct SetScheduledCharging {
    /// Whether scheduled charging is enabled.
    #[cfg_attr(feature = "cli", clap(short, long))]
    pub enable: bool,

    /// Minutes after midnight (local time) to start charging. If omitted it will be midnight.
    ///
    /// NOTE: In the future this will be a time instead of minutes.
    #[cfg_attr(feature = "cli", clap(short, long))]
    pub time: Option<u64>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct SetScheduledDeparture {
    /// Whether scheduled departure is enabled.
    #[cfg_attr(feature = "cli", clap(short, long))]
    pub enable: bool,

    /// Minutes after midnight (local time) to depart.
    ///
    /// NOTE: In the future this will be a time instead of minutes.
    #[cfg_attr(feature = "cli", clap(short, long))]
    pub departure_time: Option<u64>,

    #[cfg_attr(feature = "cli", clap(short, long))]
    pub preconditioning_enabled: bool,

    #[cfg_attr(feature = "cli", clap(short = 'w', long))]
    pub preconditioning_weekdays_only: bool,

    #[cfg_attr(feature = "cli", clap(short, long))]
    pub off_peak_charging_enabled: bool,

    #[cfg_attr(feature = "cli", clap(short = 'y', long))]
    pub off_peak_charging_weekdays_only: bool,

    /// Minutes after midnight (local time) to end off peak charging.
    ///
    /// NOTE: In the future this will be a time instead of minutes.
    #[cfg_attr(feature = "cli", clap(short = 'n', long))]
    pub end_off_peak_time: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RequestData;

    #[test]
    fn json_charge_state() {
        let s = r#"
    {
      "response": {
        "battery_heater_on": false,
        "battery_level": 50,
        "battery_range": 176.08,
        "charge_amps": 5,
        "charge_current_request": 5,
        "charge_current_request_max": 16,
        "charge_enable_request": true,
        "charge_energy_added": 1.05,
        "charge_limit_soc": 75,
        "charge_limit_soc_max": 100,
        "charge_limit_soc_min": 50,
        "charge_limit_soc_std": 90,
        "charge_miles_added_ideal": 5,
        "charge_miles_added_rated": 5,
        "charge_port_cold_weather_mode": false,
        "charge_port_color": "<invalid>",
        "charge_port_door_open": true,
        "charge_port_latch": "Engaged",
        "charge_rate": 14.8,
        "charge_to_max_range": false,
        "charger_actual_current": 5,
        "charger_phases": 2,
        "charger_pilot_current": 16,
        "charger_power": 4,
        "charger_voltage": 241,
        "charging_state": "Charging",
        "conn_charge_cable": "IEC",
        "est_battery_range": 163.81,
        "fast_charger_brand": "<invalid>",
        "fast_charger_present": false,
        "fast_charger_type": "ACSingleWireCAN",
        "ideal_battery_range": 176.08,
        "managed_charging_active": false,
        "managed_charging_start_time": null,
        "managed_charging_user_canceled": false,
        "max_range_charge_counter": 0,
        "minutes_to_full_charge": 350,
        "not_enough_power_to_heat": null,
        "off_peak_charging_enabled": false,
        "off_peak_charging_times": "all_week",
        "off_peak_hours_end_time": 1140,
        "preconditioning_enabled": false,
        "preconditioning_times": "all_week",
        "scheduled_charging_mode": "StartAt",
        "scheduled_charging_pending": false,
        "scheduled_charging_start_time": 1647045000,
        "scheduled_charging_start_time_app": 690,
        "scheduled_charging_start_time_minutes": 690,
        "scheduled_departure_time": 1641337200,
        "scheduled_departure_time_minutes": 600,
        "supercharger_session_trip_planner": false,
        "time_to_full_charge": 5.83,
        "timestamp": 1646978638155,
        "trip_charging": false,
        "usable_battery_level": 50,
        "user_charge_enable_request": null
      }
    }
    "#;

        let request_data = RequestData::GET {
            url: "https://owner-api.teslamotors.com/api/1/vehicles/1234567890/data_request/charge_state",
        };
        Api::parse_json::<ChargeState>(&request_data, s.to_string()).unwrap();
    }

    #[test]
    fn json_climate_state() {
        let s = r#"
    {
      "response": {
          "allow_cabin_overheat_protection":false,
          "battery_heater":false,
          "battery_heater_no_power":false,
          "cabin_overheat_protection":"On",
          "climate_keeper_mode":"off",
          "defrost_mode":0,
          "driver_temp_setting":23.0,
          "fan_status":0,
          "hvac_auto_request":"On",
          "inside_temp":23.9,
          "is_auto_conditioning_on":false,
          "is_climate_on":false,
          "is_front_defroster_on":false,
          "is_preconditioning":false,
          "is_rear_defroster_on":false,
          "left_temp_direction":-103,
          "max_avail_temp":28.0,
          "min_avail_temp":15.0,
          "outside_temp":19.5,
          "passenger_temp_setting":23.0,
          "remote_heater_control_enabled":false,
          "right_temp_direction":-103,
          "seat_heater_left":0,
          "seat_heater_rear_center":0,
          "seat_heater_rear_left":0,
          "seat_heater_rear_right":0,
          "seat_heater_right":0,
          "side_mirror_heaters":false,
          "steering_wheel_heater":false,
          "supports_fan_only_cabin_overheat_protection":false,
          "timestamp":1695129645306,
          "wiper_blade_heater":false
      }
    }
    "#;

        let request_data = RequestData::GET {
            url: "https://owner-api.teslamotors.com/api/1/vehicles/1234567890/data_request/climate_state",
        };
        Api::parse_json::<ClimateState>(&request_data, s.to_string()).unwrap();
    }

    #[test]
    fn json_drive_state() {
        let s = r#"
    {
      "response": {
          "gps_as_of":1695129643,
          "heading":107,
          "latitude":47.004162,
          "longitude":8.603523,
          "native_latitude":47.004162,
          "native_location_supported":1,
          "native_longitude":8.603523,
          "native_type":"wgs",
          "power":0,
          "shift_state":null,
          "speed":null,
          "timestamp":1695129645307
      }
    }
    "#;

        let request_data = RequestData::GET {
            url: "https://owner-api.teslamotors.com/api/1/vehicles/1234567890/data_request/drive_state",
        };
        Api::parse_json::<DriveState>(&request_data, s.to_string()).unwrap();
    }

    #[test]
    fn json_gui_settings() {
        let s = r#"
    {
      "response": {
          "gui_24_hour_time":true,
          "gui_charge_rate_units":"km/hr",
          "gui_distance_units":"km/hr",
          "gui_range_display":"Ideal",
          "gui_temperature_units":"C",
          "show_range_units":true,
          "timestamp":1695129645307
      }
    }
    "#;

        let request_data = RequestData::GET {
            url: "https://owner-api.teslamotors.com/api/1/vehicles/1234567890/data_request/gui_settings",
        };
        Api::parse_json::<GuiSettings>(&request_data, s.to_string()).unwrap();
    }

    #[test]
    fn json_vehicle_config() {
        let s = r#"
    {
      "response": {
          "can_accept_navigation_requests": true,
          "can_actuate_trunks": true,
          "car_special_type": "base",
          "car_type": "models",
          "charge_port_type": "EU",
          "dashcam_clip_save_supported": false,
          "default_charge_to_max": false,
          "driver_assist": "MonoCam",
          "ece_restrictions": true,
          "efficiency_package": "Default",
          "eu_vehicle": true,
          "exterior_color": "MetallicBlack",
          "front_drive_unit": "NoneOrSmall",
          "has_air_suspension": true,
          "has_ludicrous_mode": false,
          "has_seat_cooling": false,
          "headlamp_type": "Hid",
          "interior_trim_type": "AllBlack",
          "motorized_charge_port": true,
          "plg": true,
          "pws": false,
          "rear_drive_unit": "Large",
          "rear_seat_heaters": 1,
          "rear_seat_type": 0,
          "rhd": false,
          "roof_color": "None",
          "seat_type": 1,
          "spoiler_type": "Passive",
          "sun_roof_installed": 1,
          "third_row_seats": "None",
          "timestamp": 1696881260450,
          "trim_badging": "p85d",
          "use_range_badging": false,
          "utc_offset": 7200,
          "wheel_type": "Charcoal21"
      }
    }
    "#;

        let request_data = RequestData::GET {
            url: "https://owner-api.teslamotors.com/api/1/vehicles/1234567890/data_request/vehicle_config",
        };
        Api::parse_json::<VehicleConfig>(&request_data, s.to_string()).unwrap();
    }

    #[test]
    fn json_vehicle_state() {
        let s = r#"
    {
      "response": {
          "api_version": 36,
          "autopark_state_v2": "standby",
          "autopark_style": "dead_man",
          "calendar_supported": true,
          "car_version": "2022.8.10.16 7477b4ff8e78",
          "center_display_state": 0,
          "dashcam_clip_save_available": false,
          "dashcam_state": "<invalid>",
          "df": 0,
          "dr": 0,
          "fd_window": 0,
          "feature_bitmask": "5,0",
          "fp_window": 0,
          "ft": 0,
          "homelink_device_count": 0,
          "homelink_nearby": false,
          "is_user_present": false,
          "last_autopark_error": "no_error",
          "locked": true,
          "media_state": {
            "remote_control_enabled": true
          },
          "notifications_supported": true,
          "odometer": 104885.252853,
          "parsed_calendar_supported": true,
          "pf": 0,
          "pr": 0,
          "rd_window": 0,
          "remote_start": false,
          "remote_start_enabled": true,
          "remote_start_supported": true,
          "rp_window": 0,
          "rt": 0,
          "santa_mode": 0,
          "smart_summon_available": false,
          "software_update": {
            "download_perc": 0,
            "expected_duration_sec": 2700,
            "install_perc": 1,
            "status": "",
            "version": "EU-2023.20-14615"
          },
          "speed_limit_mode": {
            "active": false,
            "current_limit_mph": 90,
            "max_limit_mph": 90,
            "min_limit_mph": 50,
            "pin_code_set": false
          },
          "summon_standby_mode_enabled": false,
          "sun_roof_percent_open": 0,
          "sun_roof_state": "closed",
          "timestamp": 1696881260448,
          "tpms_pressure_fl": null,
          "tpms_pressure_fr": null,
          "tpms_pressure_rl": null,
          "tpms_pressure_rr": null,
          "valet_mode": false,
          "valet_pin_needed": true,
          "vehicle_name": "HTLC"
        }
    }
    "#;

        let request_data = RequestData::GET {
            url: "https://owner-api.teslamotors.com/api/1/vehicles/1234567890/data_request/vehicle_state",
        };
        Api::parse_json::<VehicleState>(&request_data, s.to_string()).unwrap();
    }

    #[test]
    fn json_vehicle_data_htlc_2023_09_19() {
        let s = include_str!("../testdata/vehicle_data_HTLC_2023_09_19.json");

        let request_data = RequestData::GET {
            url: "https://owner-api.teslamotors.com/api/1/vehicles/1234567890/vehicle_data",
        };
        Api::parse_json::<VehicleData>(&request_data, s.to_string()).unwrap();
    }

    #[test]
    fn json_vehicle_data_htlc_2023_10_09() {
        let s = include_str!("../testdata/vehicle_data_HTLC_2023_10_09.json");

        let request_data = RequestData::GET {
            url: "https://owner-api.teslamotors.com/api/1/vehicles/1234567890/vehicle_data",
        };
        Api::parse_json::<VehicleData>(&request_data, s.to_string()).unwrap();
    }

    #[test]
    fn json_vehicle_data_bitcoin_lightning_2023_10_09() {
        let s = include_str!("../testdata/vehicle_data_BitcoinLightning_2023_10_09.json");

        let request_data = RequestData::GET {
            url: "https://owner-api.teslamotors.com/api/1/vehicles/1234567890/vehicle_data",
        };
        Api::parse_json::<VehicleData>(&request_data, s.to_string()).unwrap();
    }
}
