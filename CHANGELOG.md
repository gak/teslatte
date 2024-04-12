# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.15] - 2023-04-12

### Fixed

- Forgot a `total_pack_energy`. (#22)

## [0.1.14] - 2023-04-12

### Changed

- `energy_left` and `total_pack_energy` are gone from the API. (#22)
- Don't rely on serde untagged to determine the product type, as any missing
  fields will give an unspecified error. Instead directly check known fields
  that will probably not be removed. https://github.com/serde-rs/serde/pull/2376

## [0.1.13] - 2023-01-24

### Changed

- Vehicles list is gone from the API. Use `products` instead. (#13)

## [0.1.12] - 2023-01-20

### Changed

- PowerwallData new fields:
  - powerwall_onboarding_settings_set
  - storm_mode_enabled
  - features
  - warp_site_number
  - go_off_grid_test_banner_enabled
  - powerwall_tesla_electric_interested_in
  - vpp_tour_enabled
- Components market_type is now Option<String>
- LiveWallConnector wall_connector_power is now f32

## [0.1.11] - 2023-11-11

### Changed

- `vehicle_data` now accepts `GetVehicleData` instead of `VehicleId`. (See [6facc27](https://github.com/gak/teslatte/commit/6facc27d8b408d35b98b4c6c0ad3e5df82328d2c))

## [0.1.10] - 2023-11-11

### Changed

- API changes for "api_version 67"

  - VehicleData new fields:

    - cached_data
    - command_signing
    - release_notes_supported

  - ClimateState new fields:

    - auto_steering_wheel_heat
    - cop_activation_temperature,
    - steering_wheel_heat_level

  - DriveState now Optional:

    - gps_as_of
    - heading
    - latitude
    - longitude
    - native_latitude
    - native_location_supported
    - native_longitude
    - native_type

  - DriveState new fields:

    - active_route_traffic_minutes_delay

  - GuiSettings new field:

    - gui_tirepressure_units

  - VehicleConfig new fields:

    - cop_user_set_temp_supported
    - webcam_selfie_supported

  - VehicleState new fields:

    - media_info: MediaInfo
    - tpms_hard_warning_fl
    - tpms_hard_warning_fr
    - tpms_hard_warning_rl
    - tpms_hard_warning_rr
    - tpms_rcp_front_value
    - tpms_rcp_rear_value
    - tpms_last_seen_pressure_time_fl
    - tpms_last_seen_pressure_time_fr
    - tpms_last_seen_pressure_time_rl
    - tpms_last_seen_pressure_time_rr
    - tpms_soft_warning_fl
    - tpms_soft_warning_fr
    - tpms_soft_warning_rl
    - tpms_soft_warning_rr

  - MediaInfo new fields:
    - audio_volume
    - audio_volume_increment
    - audio_volume_max

## [0.1.9] - 2023-11-06

### Added

- Support for registered wall connectors.

## [0.1.8] - 2023-11-03

### Changed

- Internal doc generation for API coverage, and comparison between APIs. See [API.md](API.md) for the output.

### Fixed

- colored_json 3.3.0 -> 4.0.0 (was yanked) (#10)

## [0.1.7] - 2023-10-20

### Added

- API for HVAC: `auto_conditioning_start`, `auto_conditioning_stop`, `set_temps`.
- API for locking: `door_lock`, `door_unlock`.
- API `remote_start_drive`.
- API `wake_up`.

### Changed

- RequestData is now private because it's not used outside of the crate.
- RequestData variants fixed lints: GET -> Get.
- `ClimateState.inside_temp` is now `Optional`.

### Fixed

- Trim spaces around access token string.

## [0.1.6] - 2023-10-10

### Fixes

- More optional fields and parsing tests (#4).

## [0.1.5] - 2023-09-20

### Added

- This changelog.

### Fixed

- Additional response fields for Model S (#3).
