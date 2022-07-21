use crate::auth::AccessToken;
use crate::error::TeslatteError;
use miette::IntoDiagnostic;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use tracing::{debug, instrument, trace};

pub mod auth;
pub mod error;
pub mod vehicle_state;

const API_URL: &str = "https://owner-api.teslamotors.com";

/// Vehicle ID used by the owner-api endpoint.
///
/// This data comes from [`Api::vehicles()`] `id` field.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Id(u64);

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Id {
    type Err = miette::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Id(s.parse().into_diagnostic()?))
    }
}

/// Vehicle ID used by other endpoints.
///
/// This data comes from [`Api::vehicles()`] `vehicle_id` field.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VehicleId(u64);

pub struct Api {
    access_token: AccessToken,
    client: Client,
}

impl Api {
    pub fn new(access_token: &AccessToken) -> Self {
        Api {
            access_token: access_token.clone(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(), // TODO: unwrap
        }
    }

    #[instrument(skip(self))]
    async fn get<D>(&self, path: &str) -> Result<D, TeslatteError>
    where
        // I don't understand but it works: https://stackoverflow.com/a/60131725/11125
        D: for<'de> Deserialize<'de> + Debug,
    {
        let url = format!("{}{}", API_URL, path);
        let request = || format!("GET {url}");
        trace!(?url, "Fetching");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.access_token.0))
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|source| TeslatteError::FetchError {
                source,
                request: request(),
            })?;

        let body = response
            .text()
            .await
            .map_err(|source| TeslatteError::FetchError {
                source,
                request: request(),
            })?;
        trace!(?body);

        let json = Self::parse_json::<D, _>(&body, request)?;
        trace!(?json);

        Ok(json)
    }

    #[instrument(skip(self))]
    async fn post<S>(&self, path: &str, body: S) -> Result<(), TeslatteError>
    where
        S: Serialize + Debug,
    {
        trace!("Fetching");
        let url = format!("{}{}", API_URL, path);
        let request = || {
            let payload =
                serde_json::to_string(&body).expect("Should not fail creating the request struct.");
            format!("POST {} {payload}", &url)
        };
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token.0))
            .header("Accept", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|source| TeslatteError::FetchError {
                source,
                request: request(),
            })?;
        let body = response
            .text()
            .await
            .map_err(|source| TeslatteError::FetchError {
                source,
                request: request(),
            })?;
        let json = Self::parse_json::<PostResponse, _>(&body, request)?;
        trace!(?json);

        if json.result {
            Ok(())
        } else {
            Err(TeslatteError::ServerError {
                request: request(),
                msg: json.reason,
                description: None,
            })
        }
    }

    // The `request` argument is for additional context in the error.
    fn parse_json<T, F>(body: &str, request: F) -> Result<T, TeslatteError>
    where
        T: for<'de> Deserialize<'de> + Debug,
        F: FnOnce() -> String + Copy,
    {
        trace!("{}", &body);
        let r: Response<T> = serde_json::from_str::<ResponseDeserializer<T>>(body)
            .map_err(|source| TeslatteError::DecodeJsonError {
                source,
                request: request(),
                body: body.to_string(),
            })?
            .into();

        match r {
            Response::Response(r) => Ok(r),
            Response::Error(e) => Err(TeslatteError::ServerError {
                request: request(),
                msg: e.error,
                description: e.error_description,
            }),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ResponseDeserializer<T> {
    error: Option<ResponseError>,
    response: Option<T>,
}

#[derive(Debug)]
enum Response<T> {
    Response(T),
    Error(ResponseError),
}

impl<T> From<ResponseDeserializer<T>> for Response<T> {
    fn from(response: ResponseDeserializer<T>) -> Self {
        match response.error {
            Some(error) => Response::Error(error),
            None => match response.response {
                Some(response) => Response::Response(response),
                None => panic!("ResponseDeserializer has no error or response."),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
struct PostResponse {
    reason: String,
    result: bool,
}

#[derive(Debug, Deserialize)]
struct ResponseError {
    error: String,
    error_description: Option<String>,
}

#[derive(Debug, Serialize)]
struct Empty {}

/// GET /api/1/[url]
macro_rules! get {
    ($name:ident, $return_type:ty, $url:expr) => {
        pub async fn $name(&self) -> Result<$return_type, TeslatteError> {
            let url = format!("/api/1{}", $url);
            self.get(&url).await
        }
    };
}

/// GET /api/1/[url] with an argument.
///
/// Pass in the URL as a format string with one arg, which has to impl Display.
macro_rules! get_arg {
    ($name:ident, $return_type:ty, $url:expr, $arg_type:ty) => {
        pub async fn $name(&self, arg: &$arg_type) -> miette::Result<$return_type, TeslatteError> {
            let url = format!(concat!("/api/1", $url), arg);
            self.get(&url).await
        }
    };
}

/// POST /api/1/[url] with an argument and data
macro_rules! post_arg {
    ($name:ident, $request_type:ty, $url:expr, $arg_type:ty) => {
        pub async fn $name(
            &self,
            arg: &$arg_type,
            data: &$request_type,
        ) -> miette::Result<(), TeslatteError> {
            let url_fmt = format!($url, arg);
            let url = format!(concat!("/api/1", $url), arg);
            self.post(&url, data).await
        }
    };
}

// /// POST /api/1/vehicles/[id]/... without data
// macro_rules! post_v {
//     ($name:ident, $url:expr) => {
//         pub async fn $name(&self, id: &Id) -> miette::Result<(), TeslatteError> {
//             let url = format!("/vehicles/{}{}", id.0, $url);
//             self.post(&url, &Empty {}).await
//         }
//     };
// }
//
// /// POST /api/1/vehicles/[id]/... with data
// macro_rules! post_vd {
//     ($name:ident, $struct:ty, $url:expr) => {
//         pub async fn $name(&self, id: &Id, data: &$struct) -> miette::Result<(), TeslatteError> {
//             let url = format!("/api/1/vehicles/{}{}", id.0, $url);
//             self.post(&url, &data).await
//         }
//     };
// }

use crate::vehicle_state::ChargeState;
use crate::vehicle_state::SetChargeLimit;
use crate::vehicle_state::Vehicle;
use crate::vehicle_state::VehicleData;

#[rustfmt::skip]
impl Api {
    get!(vehicles, Vec<Vehicle>, "/vehicles");
    get_arg!(vehicle_data, VehicleData, "/vehicles/{}/vehicle_data", Id);
    get_arg!(charge_state, ChargeState, "/vehicles/{}/data_request/charge_state", Id);
    post_arg!(set_charge_limit, SetChargeLimit, "/vehicles/{}/command/set_charge_limit", Id);
    // post_vd!(set_charging_amps, SetChargingAmps, "/command/set_charging_amps");
    // post_v!(charge_start, "/command/charge_start");
    // post_v!(charge_stop, "/command/charge_stop");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vehicle_state::ChargeState;
    use test_log::test;

    #[test]
    fn json() {
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
        Api::parse_json::<ChargeState, _>(s, || "req".to_string()).unwrap();
    }

    #[test]
    fn error() {
        let s = r#"{
            "response": null,
            "error":{"error": "timeout","error_description": "s"}
        }"#;
        let e = Api::parse_json::<ChargeState, _>(s, || "req".to_string());
        if let Err(e) = e {
            if let TeslatteError::ServerError {
                msg, description, ..
            } = e
            {
                assert_eq!(&msg, "timeout");
                assert_eq!(&description.unwrap(), "s");
            } else {
                panic!("unexpected error: {:?}", e);
            }
        } else {
            panic!("expected an error");
        }
    }
}
