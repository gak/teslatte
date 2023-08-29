#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum TeslatteError {
    #[error("{request} server error: {msg}: {description:?}")]
    #[diagnostic()]
    ServerError {
        request: String,
        msg: String,
        description: Option<String>,
    },

    #[error("{request} unhandled server response: {body}")]
    #[diagnostic()]
    UnhandledServerError { request: String, body: String },

    #[error("{request} fetch error")]
    #[diagnostic()]
    FetchError {
        source: reqwest::Error,
        request: String,
    },

    #[error("{request} json decode error: {body}")]
    #[diagnostic()]
    DecodeJsonError {
        source: serde_json::Error,
        request: String,
        body: String,
    },

    #[error("Unhandled reqwest error.")]
    UnhandledReqwestError(#[source] reqwest::Error),

    #[error("Did not supply a valid callback URL.")]
    UserDidNotSupplyValidCallbackUrl(#[source] url::ParseError),

    #[error("Callback URL did not contain a callback code.")]
    CouldNotFindCallbackCode,

    #[error("Callback URL did not contain the state.")]
    CouldNotFindState,

    #[error(
        "State in the callback URL did not match the state in the request: {request} != {callback}"
    )]
    StateMismatch { request: String, callback: String },

    #[error("Could not convert \"{0}\" to an EnergySiteId.")]
    DecodeEnergySiteIdError(String),

    #[error("No refresh token available.")]
    NoRefreshToken,
}
