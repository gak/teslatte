use crate::auth::{AccessToken, RefreshToken};
use crate::error::TeslatteError;
use chrono::{DateTime, SecondsFormat, TimeZone};
use derive_more::{Display, FromStr};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use tracing::debug;

pub mod auth;
pub mod calendar_history;
pub mod energy;
pub mod error;
pub mod powerwall;
pub mod vehicles;

#[cfg(feature = "cli")]
pub mod cli;

const API_URL: &str = "https://owner-api.teslamotors.com/api/1";

trait Values {
    fn format(&self, url: &str) -> String;
}

/// Vehicle ID used by the owner-api endpoint.
///
/// This data comes from [`Api::vehicles()`] `id` field.
#[derive(Debug, Serialize, Deserialize, Clone, Display, FromStr)]
pub struct VehicleId(u64);

/// Vehicle ID used by other endpoints.
///
/// This data comes from [`Api::vehicles()`] `vehicle_id` field.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalVehicleId(u64);

pub enum RequestData<'a> {
    GET { url: &'a str },
    POST { url: &'a str, payload: &'a str },
}

impl Display for RequestData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestData::GET { url } => write!(f, "GET {}", url),
            RequestData::POST { url, payload } => write!(f, "POST {} {}", url, payload),
        }
    }
}

pub struct Api {
    pub access_token: AccessToken,
    // TODO: Why is this an Option?
    pub refresh_token: Option<RefreshToken>,
    client: Client,
}

impl Api {
    pub fn new(access_token: AccessToken, refresh_token: Option<RefreshToken>) -> Self {
        Api {
            access_token,
            refresh_token,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(), // TODO: unwrap
        }
    }

    async fn get<D>(&self, url: &str) -> Result<ResponseData<D>, TeslatteError>
    where
        D: for<'de> Deserialize<'de> + Debug,
    {
        self.request(&RequestData::GET { url }).await
    }

    async fn post<S>(&self, url: &str, body: S) -> Result<ResponseData<PostResponse>, TeslatteError>
    where
        S: Serialize + Debug,
    {
        let payload =
            &serde_json::to_string(&body).expect("Should not fail creating the request struct.");
        let request_data = RequestData::POST { url, payload };
        let data = self.request::<PostResponse>(&request_data).await?;

        if !data.data.result {
            return Err(TeslatteError::ServerError {
                request: format!("{request_data}"),
                msg: data.data.reason,
                description: None,
                body: Some(data.body),
            });
        }

        Ok(data)
    }

    async fn request<T>(
        &self,
        request_data: &RequestData<'_>,
    ) -> Result<ResponseData<T>, TeslatteError>
    where
        T: for<'de> Deserialize<'de> + Debug,
    {
        debug!("{request_data}");

        let request_builder = match request_data {
            RequestData::GET { url } => self.client.get(*url),
            RequestData::POST { url, payload } => self
                .client
                .post(*url)
                .header("Content-Type", "application/json")
                .body(payload.to_string()),
        };

        let response_body = request_builder
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.access_token.0))
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

        Self::parse_json(request_data, response_body)
    }

    fn parse_json<T>(
        request_data: &RequestData,
        response_body: String,
    ) -> Result<ResponseData<T>, TeslatteError>
    where
        T: for<'de> Deserialize<'de> + Debug,
    {
        let response: Response<T> = serde_json::from_str::<ResponseDeserializer<T>>(&response_body)
            .map_err(|source| TeslatteError::DecodeJsonError {
                source,
                request: format!("{request_data}"),
                body: response_body.to_string(),
            })?
            .into();

        match response {
            Response::Response(data) => Ok(ResponseData {
                data,
                body: response_body,
            }),
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

#[derive(Debug, Deserialize)]
pub struct PostResponse {
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

/// Data and body from a request. The body can be used for debugging. The CLI can optionally
/// print the raw JSON so the user can manipulate it.
///
/// This struct will automatically deref to the data type for better ergonomics.
#[derive(Debug)]
pub struct ResponseData<T> {
    data: T,
    body: String,
}

impl<T> ResponseData<T> {
    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}

impl<T> std::ops::Deref for ResponseData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// GET /api/1/[url]
macro_rules! get {
    ($name:ident, $return_type:ty, $url:expr) => {
        pub async fn $name(
            &self,
        ) -> Result<crate::ResponseData<$return_type>, crate::error::TeslatteError> {
            let url = format!("{}{}", crate::API_URL, $url);
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
        pub async fn $name(
            &self,
            arg: &$arg_type,
        ) -> miette::Result<crate::ResponseData<$return_type>, crate::error::TeslatteError> {
            let url = format!($url, arg);
            let url = format!("{}{}", crate::API_URL, url);
            self.get(&url).await
        }
    };
}
pub(crate) use get_arg;

/// GET /api/1/[url] with a struct.
macro_rules! get_args {
    ($name:ident, $return_type:ty, $url:expr, $args:ty) => {
        pub async fn $name(
            &self,
            values: &$args,
        ) -> miette::Result<crate::ResponseData<$return_type>, crate::error::TeslatteError> {
            let url = values.format($url);
            let url = format!("{}{}", crate::API_URL, url);
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
        ) -> miette::Result<crate::ResponseData<crate::PostResponse>, crate::error::TeslatteError> {
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
        pub async fn $name(
            &self,
            arg: &$arg_type,
        ) -> miette::Result<crate::ResponseData<crate::PostResponse>, crate::error::TeslatteError> {
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

        let request_data = RequestData::POST {
            url: "https://example.com",
            payload: "doesn't matter",
        };

        let e = Api::parse_json::<ChargeState>(&request_data, s.to_string());
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
