use crate::TeslatteError;
use crate::TeslatteError::UnhandledReqwestError;
use rand::Rng;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};
use url::Url;

const AUTHORIZE_URL: &str = "https://auth.tesla.com/oauth2/v3/authorize";
const TOKEN_URL: &str = "https://auth.tesla.com/oauth2/v3/token";

pub struct Authentication {
    client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken(pub String);

impl Authentication {
    pub fn new() -> Result<Self, TeslatteError> {
        let client = Client::builder()
            .cookie_store(false)
            .build()
            .map_err(UnhandledReqwestError)?;
        Ok(Self { client })
    }

    /// Currently the only way to get an access token via this library.
    pub async fn interactive_get_access_token(
        &self,
    ) -> Result<(AccessToken, RefreshToken), TeslatteError> {
        let login_form = self.get_login_url_for_user().await;
        dbg!(&login_form);

        let callback_url =
            ask_input("Enter the URL of the 404 error page after you've logged in: ");
        let callback_code = Self::extract_callback_code_from_url(&callback_url)?;

        let bearer = self
            .exchange_auth_for_bearer(&login_form.code, &callback_code)
            .await?;
        let refresh_token = bearer.refresh_token.clone();

        Ok((
            AccessToken(bearer.access_token),
            RefreshToken(refresh_token),
        ))
    }

    pub async fn get_login_url_for_user(&self) -> LoginForm {
        let code = Code::new();
        let state = random_string(8);
        let url = Self::login_url(&code, &state);
        LoginForm { url, code, state }
    }

    async fn exchange_auth_for_bearer(
        &self,
        code: &Code,
        callback_code: &str,
    ) -> Result<BearerTokenResponse, TeslatteError> {
        let url = TOKEN_URL;
        let payload = BearerTokenRequest {
            grant_type: "authorization_code".into(),
            client_id: "ownerapi".into(),
            code: callback_code.into(),
            code_verifier: code.verifier.clone(),
            redirect_uri: "https://auth.tesla.com/void/callback".into(),
        };
        self.post(url, &payload).await
    }

    pub async fn refresh_access_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<RefreshTokenResponse, TeslatteError> {
        let url = "https://auth.tesla.com/oauth2/v3/token";
        let payload = RefreshTokenRequest {
            grant_type: "refresh_token".into(),
            client_id: "ownerapi".into(),
            refresh_token: refresh_token.0.clone(),
            scope: "openid email offline_access".into(),
        };
        self.post(url, &payload).await
    }

    async fn post<'a, S, D>(&self, url: &str, payload: &S) -> Result<D, TeslatteError>
    where
        S: Serialize,
        D: DeserializeOwned,
    {
        let response = self
            .client
            .post(url)
            .header("Accept", "application/json")
            .json(payload)
            .send()
            .await
            .map_err(|source| TeslatteError::FetchError {
                source,
                request: url.to_string(),
            })?;

        let body = response
            .text()
            .await
            .map_err(|source| TeslatteError::FetchError {
                source,
                request: url.to_string(),
            })?;

        let json =
            serde_json::from_str::<D>(&body).map_err(|source| TeslatteError::DecodeJsonError {
                source,
                body: body.to_string(),
                request: url.to_string(),
            })?;

        Ok(json)
    }

    pub fn login_url(code: &Code, state: &str) -> String {
        let mut url = Url::parse(AUTHORIZE_URL).unwrap();
        let mut query = url.query_pairs_mut();
        query.append_pair("client_id", "ownerapi");
        query.append_pair("code_challenge", &code.challenge);
        query.append_pair("code_challenge_method", "S256");
        query.append_pair("redirect_uri", "https://auth.tesla.com/void/callback");
        query.append_pair("response_type", "code");
        query.append_pair("scope", "openid email offline_access");
        query.append_pair("state", state);
        drop(query);
        url.to_string()
    }

    fn extract_callback_code_from_url(callback_url: &str) -> Result<String, TeslatteError> {
        Ok(Url::parse(callback_url)
            .map_err(TeslatteError::UserDidNotSupplyValidCallbackUrl)?
            .query_pairs()
            .find(|(k, _)| k == "code")
            .map(|kv| kv.1)
            .ok_or(TeslatteError::CouldNotFindCallbackCode)?
            .to_string())
    }
}

#[derive(Debug, Serialize)]
struct RefreshTokenRequest {
    grant_type: String,
    client_id: String,
    refresh_token: String,
    scope: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
    pub id_token: String,
    pub expires_in: u32,
    pub token_type: String,
}

#[derive(Debug, Default)]
pub struct LoginForm {
    url: String,
    code: Code,
    state: String,
}

// These can be probably &str.
#[derive(Debug, Serialize)]
struct BearerTokenRequest {
    grant_type: String,
    client_id: String,
    code: String,
    code_verifier: String,
    redirect_uri: String,
}

#[derive(Debug, Deserialize)]
struct BearerTokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u32,
    state: String,
    token_type: String,
    id_token: String,
}

#[derive(Debug, Default)]
pub struct Code {
    verifier: String,
    challenge: String,
}

impl Code {
    fn new() -> Self {
        let verifier = random_string(86);
        let hex_digest = sha256::digest_bytes(verifier.as_bytes());
        let challenge = base64::encode_config(&hex_digest, base64::URL_SAFE);

        Self {
            verifier,
            challenge,
        }
    }
}

fn random_string(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut s = String::with_capacity(len);
    for _ in 0..len {
        s.push(rng.gen_range(b'a'..=b'z') as char);
    }
    s
}

fn ask_input(prompt: &str) -> String {
    print!("{}", prompt);
    let mut s = String::new();
    stdout()
        .flush()
        .expect("Failed to flush while expecting user input.");
    stdin()
        .read_line(&mut s)
        .expect("Failed to read line of user input.");
    s.trim().to_string()
}
