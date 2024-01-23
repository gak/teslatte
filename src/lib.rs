// Ignores: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
// TODO: Maybe use the suggestion of removing async and replacing it with Future<Output = Result<...>>
#![allow(async_fn_in_trait)]

use crate::auth::{AccessToken, RefreshToken};
use crate::error::TeslatteError;
use crate::vehicles::{
    GetVehicleData, SetChargeLimit, SetChargingAmps, SetScheduledCharging, SetScheduledDeparture,
    SetTemperatures, VehicleData,
};
use chrono::{DateTime, SecondsFormat, TimeZone};
use derive_more::{Deref, Display, From, FromStr};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use tracing::debug;

pub mod auth;
pub mod energy_sites;
pub mod error;
pub mod powerwall;
pub mod products;
pub mod vehicles;

#[cfg(feature = "cli")]
pub mod cli;

const API_URL: &str = "https://owner-api.teslamotors.com/api/1";

pub trait VehicleApi {
    async fn vehicle_data(
        &self,
        get_vehicle_data: &GetVehicleData,
    ) -> Result<VehicleData, TeslatteError>;
    async fn wake_up(&self, vehicle_id: &VehicleId) -> Result<PostResponse, TeslatteError>;

    // Alerts
    async fn honk_horn(&self, vehicle_id: &VehicleId) -> Result<PostResponse, TeslatteError>;
    async fn flash_lights(&self, vehicle_id: &VehicleId) -> Result<PostResponse, TeslatteError>;

    // Charging
    async fn charge_port_door_open(
        &self,
        vehicle_id: &VehicleId,
    ) -> Result<PostResponse, TeslatteError>;
    async fn charge_port_door_close(
        &self,
        vehicle_id: &VehicleId,
    ) -> Result<PostResponse, TeslatteError>;
    async fn set_charge_limit(
        &self,
        vehicle_id: &VehicleId,
        data: &SetChargeLimit,
    ) -> Result<PostResponse, TeslatteError>;
    async fn set_charging_amps(
        &self,
        vehicle_id: &VehicleId,
        data: &SetChargingAmps,
    ) -> Result<PostResponse, TeslatteError>;
    async fn charge_standard(&self, vehicle_id: &VehicleId) -> Result<PostResponse, TeslatteError>;
    async fn charge_max_range(&self, vehicle_id: &VehicleId)
        -> Result<PostResponse, TeslatteError>;
    async fn charge_start(&self, vehicle_id: &VehicleId) -> Result<PostResponse, TeslatteError>;
    async fn charge_stop(&self, vehicle_id: &VehicleId) -> Result<PostResponse, TeslatteError>;
    async fn set_scheduled_charging(
        &self,
        vehicle_id: &VehicleId,
        data: &SetScheduledCharging,
    ) -> Result<PostResponse, TeslatteError>;
    async fn set_scheduled_departure(
        &self,
        vehicle_id: &VehicleId,
        data: &SetScheduledDeparture,
    ) -> Result<PostResponse, TeslatteError>;

    // HVAC
    async fn auto_conditioning_start(
        &self,
        vehicle_id: &VehicleId,
    ) -> Result<PostResponse, TeslatteError>;
    async fn auto_conditioning_stop(
        &self,
        vehicle_id: &VehicleId,
    ) -> Result<PostResponse, TeslatteError>;
    async fn set_temps(
        &self,
        vehicle_id: &VehicleId,
        data: &SetTemperatures,
    ) -> Result<PostResponse, TeslatteError>;

    // Doors
    async fn door_unlock(&self, vehicle_id: &VehicleId) -> Result<PostResponse, TeslatteError>;
    async fn door_lock(&self, vehicle_id: &VehicleId) -> Result<PostResponse, TeslatteError>;
    async fn remote_start_drive(
        &self,
        vehicle_id: &VehicleId,
    ) -> Result<PostResponse, TeslatteError>;
}

trait EnergySitesApi {}

trait ApiValues {
    fn format(&self, url: &str) -> String;
}

/// Vehicle ID used by the owner-api endpoint.
///
/// This data comes from [`OwnerApi::vehicles()`] `id` field.
#[derive(Debug, Serialize, Deserialize, Clone, Display, FromStr, From, Deref)]
pub struct VehicleId(u64);

impl VehicleId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Vehicle ID used by other endpoints.
///
/// This data comes from [`OwnerApi::vehicles()`] `vehicle_id` field.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalVehicleId(u64);

enum RequestData<'a> {
    Get { url: &'a str },
    Post { url: &'a str, payload: &'a str },
}

impl Display for RequestData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestData::Get { url } => write!(f, "GET {}", url),
            RequestData::Post { url, payload } => write!(f, "POST {} {}", url, payload),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PrintResponses {
    No,
    Plain,
    Pretty,
}

/// API client for the Tesla API.
///
/// Main entry point for the API. It contains the access token and refresh token, and can be used
/// to make requests to the API.
pub struct OwnerApi {
    pub access_token: AccessToken,
    pub refresh_token: Option<RefreshToken>,
    pub print_responses: PrintResponses,
    client: Client,
}

impl OwnerApi {
    pub fn new(access_token: AccessToken, refresh_token: Option<RefreshToken>) -> Self {
        Self {
            access_token,
            refresh_token,
            print_responses: PrintResponses::No,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(), // TODO: unwrap
        }
    }

    async fn get<D>(&self, url: &str) -> Result<D, TeslatteError>
    where
        D: for<'de> Deserialize<'de> + Debug,
    {
        self.request(&RequestData::Get { url }).await
    }

    async fn post<S>(&self, url: &str, body: S) -> Result<PostResponse, TeslatteError>
    where
        S: Serialize + Debug,
    {
        let payload =
            &serde_json::to_string(&body).expect("Should not fail creating the request struct.");
        let request_data = RequestData::Post { url, payload };
        let data = self.request::<PostResponse>(&request_data).await?;

        if !data.result {
            return Err(TeslatteError::ServerError {
                request: format!("{request_data}"),
                description: None,
                msg: data.reason,
                body: None,
            });
        }

        Ok(data)
    }

    async fn request<T>(&self, request_data: &RequestData<'_>) -> Result<T, TeslatteError>
    where
        T: for<'de> Deserialize<'de> + Debug,
    {
        debug!("{request_data}");

        let request_builder = match request_data {
            RequestData::Get { url } => self.client.get(*url),
            RequestData::Post { url, payload } => self
                .client
                .post(*url)
                .header("Content-Type", "application/json")
                .body(payload.to_string()),
        };

        let response_body = request_builder
            .header("Accept", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.access_token.0.trim()),
            )
            .send()
            .await
            .map_err(|source| TeslatteError::FetchError {
                source,
                request: format!("{request_data}"),
            })?
            .text()
            .await
            .map_err(|source| TeslatteError::FetchError {
                source,
                request: format!("{request_data}"),
            })?;

        debug!("Response: {response_body}");

        Self::parse_json(request_data, response_body, self.print_responses)
    }

    fn parse_json<T>(
        request_data: &RequestData,
        response_body: String,
        print_response: PrintResponses,
    ) -> Result<T, TeslatteError>
    where
        T: for<'de> Deserialize<'de> + Debug,
    {
        match print_response {
            PrintResponses::No => {}
            PrintResponses::Plain => {
                println!("{}", response_body);
            }
            PrintResponses::Pretty => {
                print_json_str(&response_body);
            }
        }

        let response: Response<T> = serde_json::from_str::<ResponseDeserializer<T>>(&response_body)
            .map_err(|source| TeslatteError::DecodeJsonError {
                source,
                request: format!("{request_data}"),
                body: response_body.to_string(),
            })?
            .into();

        match response {
            Response::Response(data) => Ok(data),
            Response::Error(e) => Err(TeslatteError::ServerError {
                request: format!("{request_data}"),
                msg: e.error,
                description: e.error_description,
                body: Some(response_body.to_owned()),
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

/// Standard response data from the API. Contains a reason string and a result bool.
#[derive(Debug, Deserialize)]
pub struct PostResponse {
    reason: String,
    result: bool,
}

/// Standard error response from the API.
#[derive(Debug, Deserialize)]
struct ResponseError {
    error: String,
    error_description: Option<String>,
}

#[derive(Debug, Serialize)]
struct Empty {}

/// GET /api/1/[url]
#[allow(unused_macros)]
macro_rules! get {
    ($name:ident, $return_type:ty, $url:expr) => {
        async fn $name(&self) -> Result<$return_type, crate::error::TeslatteError> {
            let url = format!("{}{}", crate::API_URL, $url);
            self.get(&url)
                .await
                .map_err(|e| crate::error::TeslatteError::from(e))
        }
    };
}
#[allow(unused_imports)]
pub(crate) use get;

/// Same as get, but public.
macro_rules! pub_get {
    ($name:ident, $return_type:ty, $url:expr) => {
        pub async fn $name(&self) -> Result<$return_type, crate::error::TeslatteError> {
            let url = format!("{}{}", crate::API_URL, $url);
            self.get(&url)
                .await
                .map_err(|e| crate::error::TeslatteError::from(e))
        }
    };
}
pub(crate) use pub_get;

/// GET /api/1/[url] with an argument.
///
/// Pass in the URL as a format string with one arg, which has to impl Display.
#[allow(unused_macros)]
macro_rules! get_arg {
    ($name:ident, $return_type:ty, $url:expr, $arg_type:ty) => {
        async fn $name(
            &self,
            arg: &$arg_type,
        ) -> miette::Result<$return_type, crate::error::TeslatteError> {
            let url = format!($url, arg);
            let url = format!("{}{}", crate::API_URL, url);
            self.get(&url).await
        }
    };
}
#[allow(unused_imports)]
pub(crate) use get_arg;

/// Public variant of get_arg.
macro_rules! pub_get_arg {
    ($name:ident, $return_type:ty, $url:expr, $arg_type:ty) => {
        pub async fn $name(
            &self,
            arg: &$arg_type,
        ) -> miette::Result<$return_type, crate::error::TeslatteError> {
            let url = format!($url, arg);
            let url = format!("{}{}", crate::API_URL, url);
            self.get(&url).await
        }
    };
}
pub(crate) use pub_get_arg;

/// GET /api/1/[url] with a struct to format the URL.
macro_rules! get_args {
    ($name:ident, $return_type:ty, $url:expr, $args:ty) => {
        async fn $name(
            &self,
            values: &$args,
        ) -> miette::Result<$return_type, crate::error::TeslatteError> {
            let url = values.format($url);
            let url = format!("{}{}", crate::API_URL, url);
            self.get(&url).await
        }
    };
}
pub(crate) use get_args;

/// Public variant of get_args.
macro_rules! pub_get_args {
    ($name:ident, $return_type:ty, $url:expr, $args:ty) => {
        pub async fn $name(
            &self,
            values: &$args,
        ) -> miette::Result<$return_type, crate::error::TeslatteError> {
            let url = values.format($url);
            let url = format!("{}{}", crate::API_URL, url);
            self.get(&url).await
        }
    };
}
pub(crate) use pub_get_args;

/// POST /api/1/[url] with an argument and data
macro_rules! post_arg {
    ($name:ident, $request_type:ty, $url:expr, $arg_type:ty) => {
        async fn $name(
            &self,
            arg: &$arg_type,
            data: &$request_type,
        ) -> miette::Result<crate::PostResponse, crate::error::TeslatteError> {
            let url = format!($url, arg);
            let url = format!("{}{}", crate::API_URL, url);
            self.post(&url, data).await
        }
    };
}
pub(crate) use post_arg;

/// Post like above but with an empty body using the Empty struct.
macro_rules! post_arg_empty {
    ($name:ident, $url:expr, $arg_type:ty) => {
        async fn $name(
            &self,
            arg: &$arg_type,
        ) -> miette::Result<crate::PostResponse, crate::error::TeslatteError> {
            let url = format!($url, arg);
            let url = format!("{}{}", crate::API_URL, url);
            self.post(&url, &Empty {}).await
        }
    };
}
pub(crate) use post_arg_empty;

pub(crate) fn rfc3339<Tz>(d: &DateTime<Tz>) -> String
where
    Tz: TimeZone,
    Tz::Offset: Display,
{
    d.to_rfc3339_opts(SecondsFormat::Secs, true)
}

pub(crate) fn join_query_pairs(pairs: &[(&str, String)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("{}={}", k, v.replace('+', "%2B")))
        .collect::<Vec<_>>()
        .join("&")
}

pub fn print_json_str(body: &str) {
    #[cfg(feature = "cli-pretty-json")]
    {
        use colored_json::prelude::*;
        println!("{}", body.to_colored_json_auto().unwrap());
    }

    #[cfg(not(feature = "cli-pretty-json"))]
    {
        println!("{}", body);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vehicles::ChargeState;

    #[test]
    fn error() {
        let s = r#"{
            "response": null,
            "error":{"error": "timeout","error_description": "s"}
        }"#;

        let request_data = RequestData::Post {
            url: "https://example.com",
            payload: "doesn't matter",
        };

        let e = OwnerApi::parse_json::<ChargeState>(
            &request_data,
            s.to_string(),
            PrintResponses::Pretty,
        );
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
