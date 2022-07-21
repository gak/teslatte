/// Please note that these structs are generated from my own responses.
///
/// Sometimes the API will return a null for a field where I've put in a non Option type, which
/// will cause the deserializer to fail. Please log an issue to fix these if you come across it.
use crate::{Id, VehicleId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct VehicleData {
    pub id: Id,
    pub user_id: i64,
    pub vehicle_id: VehicleId,
    pub vin: String,
    pub display_name: String,
    pub option_codes: String,
    /// gak: This was null for me, assuming String.
    pub color: Option<String>,
    pub access_type: String,
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
    pub charge_state: ChargeState,
    pub climate_state: ClimateState,
    pub drive_state: DriveState,
    pub gui_settings: GuiSettings,
    pub vehicle_config: VehicleConfig,
    pub vehicle_state: VehicleState,
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
    pub charge_port_cold_weather_mode: bool,
    pub charge_port_color: String,
    pub charge_port_door_open: bool,
    pub charge_port_latch: String,
    pub charge_rate: f64,
    pub charge_to_max_range: bool,
    pub charger_actual_current: i64,
    pub charger_phases: Option<i64>,
    pub charger_pilot_current: i64,
    pub charger_power: i64,
    pub charger_voltage: i64,
    pub charging_state: String,
    pub conn_charge_cable: String,
    pub est_battery_range: f64,
    pub fast_charger_brand: String,
    pub fast_charger_present: bool,
    pub fast_charger_type: String,
    pub ideal_battery_range: f64,
    pub managed_charging_active: bool,
    pub managed_charging_start_time: Option<u64>,
    pub managed_charging_user_canceled: bool,
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
    pub trip_charging: bool,
    pub usable_battery_level: i64,
    pub user_charge_enable_request: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateState {
    pub allow_cabin_overheat_protection: bool,
    pub auto_seat_climate_left: bool,
    pub auto_seat_climate_right: bool,
    pub battery_heater: bool,
    pub battery_heater_no_power: Option<bool>,
    pub cabin_overheat_protection: String,
    pub cabin_overheat_protection_actively_cooling: bool,
    pub climate_keeper_mode: String,
    pub defrost_mode: i64,
    pub driver_temp_setting: f64,
    pub fan_status: i64,
    pub hvac_auto_request: String,
    pub inside_temp: f64,
    pub is_auto_conditioning_on: bool,
    pub is_climate_on: bool,
    pub is_front_defroster_on: bool,
    pub is_preconditioning: bool,
    pub is_rear_defroster_on: bool,
    pub left_temp_direction: i64,
    pub max_avail_temp: f64,
    pub min_avail_temp: f64,
    pub outside_temp: f64,
    pub passenger_temp_setting: f64,
    pub remote_heater_control_enabled: bool,
    pub right_temp_direction: i64,
    pub seat_heater_left: i64,
    pub seat_heater_rear_center: i64,
    pub seat_heater_rear_left: i64,
    pub seat_heater_rear_right: i64,
    pub seat_heater_right: i64,
    pub side_mirror_heaters: bool,
    pub steering_wheel_heater: bool,
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
    pub aux_park_lamps: String,
    pub badge_version: i64,
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
    pub exterior_trim: String,
    pub exterior_trim_override: String,
    pub has_air_suspension: bool,
    pub has_ludicrous_mode: bool,
    pub has_seat_cooling: bool,
    pub headlamp_type: String,
    pub interior_trim_type: String,
    pub key_version: i64,
    pub motorized_charge_port: bool,
    pub paint_color_override: String,
    pub performance_package: String,
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
    pub supports_qr_pairing: bool,
    pub third_row_seats: String,
    pub timestamp: i64,
    pub trim_badging: String,
    pub use_range_badging: bool,
    pub utc_offset: i64,
    pub webcam_supported: bool,
    pub wheel_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleState {
    pub api_version: i64,
    pub autopark_state_v2: String,
    pub autopark_style: String,
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
    pub is_user_present: bool,
    pub last_autopark_error: String,
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
    pub sentry_mode: bool,
    pub sentry_mode_available: bool,
    pub service_mode: bool,
    pub service_mode_plus: bool,
    pub smart_summon_available: bool,
    pub software_update: SoftwareUpdate,
    pub speed_limit_mode: SpeedLimitMode,
    pub summon_standby_mode_enabled: bool,
    pub timestamp: i64,
    pub tpms_pressure_fl: f64,
    pub tpms_pressure_fr: f64,
    pub tpms_pressure_rl: f64,
    pub tpms_pressure_rr: f64,
    pub valet_mode: bool,
    pub vehicle_name: String,
    pub vehicle_self_test_progress: i64,
    pub vehicle_self_test_requested: bool,
    pub webcam_available: bool,
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

#[derive(Debug, Deserialize)]
pub struct Vehicles(Vec<Vehicle>);

#[derive(Debug, Deserialize)]
pub struct Vehicle {
    pub id: Id,
    pub vehicle_id: VehicleId,
    pub vin: String,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct SetChargingAmps {
    pub charging_amps: i64,
}

#[derive(Debug, Serialize)]
pub struct SetChargeLimit {
    // pub percent: Percentage,
    pub percent: u8,
}
