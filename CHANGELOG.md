# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
