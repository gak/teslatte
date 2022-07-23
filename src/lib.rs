use crate::auth::AccessToken;
use crate::error::TeslatteError;
use miette::IntoDiagnostic;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use tracing::{debug, instrument, trace};

pub mod auth;
pub mod energy;
pub mod error;
pub mod powerwall;
pub mod vehicles;

const API_URL: &str = "https://owner-api.teslamotors.com";

/// Vehicle ID used by the owner-api endpoint.
///
/// This data comes from [`Api::vehicles()`] `id` field.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VehicleId(u64);

impl Display for VehicleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for VehicleId {
    type Err = miette::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(VehicleId(s.parse().into_diagnostic()?))
    }
}

/// Vehicle ID used by other endpoints.
///
/// This data comes from [`Api::vehicles()`] `vehicle_id` field.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalVehicleId(u64);

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
pub(crate) use get;

/// GET /api/1/[url] with an argument.
///
/// Pass in the URL as a format string with one arg, which has to impl Display.
macro_rules! get_arg {
    ($name:ident, $return_type:ty, $url:expr, $arg_type:ty) => {
        pub async fn $name(&self, arg: &$arg_type) -> miette::Result<$return_type, TeslatteError> {
            let url = format!($url, arg);
            let url = format!("/api/1{}", url);
            self.get(&url).await
        }
    };
}
pub(crate) use get_arg;

/// GET /api/1/[url] with a struct.
macro_rules! get_args {
    ($name:ident, $return_type:ty, $url:expr, $args:ty) => {
        pub async fn $name(&self, values: &$args) -> miette::Result<$return_type, TeslatteError> {
            let url = values.format($url);
            let url = format!("/api/1{}", url);
            self.get(&url).await
        }
    };
}
pub(crate) use get_args;

/// POST /api/1/[url] with an argument and data
macro_rules! post_arg {
    ($name:ident, $request_type:ty, $url:expr, $arg_type:ty) => {
        pub async fn $name(
            &self,
            arg: &$arg_type,
            data: &$request_type,
        ) -> miette::Result<(), TeslatteError> {
            let url = format!($url, arg);
            let url = format!("/api/1{}", url);
            self.post(&url, data).await
        }
    };
}
pub(crate) use post_arg;

/// Post like above but with an empty body using the Empty struct.
macro_rules! post_arg_empty {
    ($name:ident, $url:expr, $arg_type:ty) => {
        pub async fn $name(&self, arg: &$arg_type) -> miette::Result<(), TeslatteError> {
            let url = format!($url, arg);
            let url = format!("/api/1{}", url);
            self.post(&url, &Empty {}).await
        }
    };
}
pub(crate) use post_arg_empty;

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
