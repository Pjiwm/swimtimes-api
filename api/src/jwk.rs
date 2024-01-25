use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
use log::{error, info};
use reqwest::{header::HeaderValue, Response};
use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::Mutex;

const FALLBACK_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Debug, Deserialize)]
struct KeyResponse {
    keys: Vec<JwkKey>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct JwkKey {
    pub e: String,
    pub alg: String,
    pub kty: String,
    pub kid: String,
    pub n: String,
}

struct JwkKeys {
    pub keys: Vec<JwkKey>,
    pub validity: Duration,
}

enum JwkError {
    NoMaxAgeSpecified,
    NoCacheControlHeader,
    MaxAgeValueEmpty,
    NonNumericMaxAge,
    MissingKey,
    FailedToBuildKeys,
    FailedToFetchKeys,
}

#[derive(Debug, Clone)]
struct JwkConfiguration {
    jwk_url: String,
    audience: String,
    issuer: String,
}

impl JwkConfiguration {
    pub fn get_max_age(&self, response: &Response) -> Result<Duration, JwkError> {
        let headers = response.headers();
        let header = headers.get("Cache-Control");

        match header {
            Some(header_value) => self.parse_cache_control_header(header_value),
            None => Err(JwkError::NoCacheControlHeader),
        }
    }

    pub async fn fetch_keys_for_configuration(&self) -> Result<JwkKeys, JwkError> {
        let url = &self.jwk_url;

        match reqwest::get(url).await {
            Ok(response) => {
                let max_age = self.get_max_age(&response).unwrap_or(FALLBACK_TIMEOUT);
                response
                    .json::<KeyResponse>()
                    .await
                    .map(|res| JwkKeys {
                        keys: res.keys,
                        validity: max_age,
                    })
                    .map_err(|_| JwkError::FailedToBuildKeys)
            }
            Err(_) => Err(JwkError::FailedToFetchKeys),
        }
    }

    fn parse_cache_control_header(&self, header_value: &HeaderValue) -> Result<Duration, JwkError> {
        match header_value.to_str() {
            Ok(string_value) => self.parse_max_age_value(string_value),
            Err(_) => Err(JwkError::NoCacheControlHeader),
        }
    }

    fn parse_max_age_value(&self, cache_control_value: &str) -> Result<Duration, JwkError> {
        let tokens: Vec<&str> = cache_control_value.split(',').collect();
        for token in tokens {
            let key_value: Vec<&str> = token.split('=').map(|s| s.trim()).collect();
            let key = key_value.first().ok_or(JwkError::MissingKey)?;
            let val = key_value.get(1).ok_or(JwkError::MaxAgeValueEmpty)?;

            if String::from("max-age").eq(&key.to_lowercase()) {
                return Ok(Duration::from_secs(
                    val.parse().map_err(|_| JwkError::NonNumericMaxAge)?,
                ));
            }
        }
        Err(JwkError::NoMaxAgeSpecified)
    }
}

impl Default for JwkConfiguration {
    fn default() -> Self {
        Self {
            jwk_url: env::var("JWK_URL").expect("JWK_URL must be set"),
            audience: env::var("JWK_AUDIENCE").expect("JWK_AUDIENCE must be set"),
            issuer: env::var("JWK_ISSUER").expect("JWK_ISSUER must be set"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    // The audience the token was issued for
    pub aud: String,
    // The expiry date -- as epoch seconds
    pub exp: i64,
    // The token issuer
    pub iss: String,
    // The subject the token refers to
    pub sub: String,
    // Issued at -- as epoch seconds
    pub iat: i64,
}

enum VerificationError {
    InvalidSignature,
    UnkownKeyAlgorithm,
    FailedToDecodeKey,
}

#[derive(Debug, Clone)]
struct JwkVerifier {
    keys: Arc<HashMap<String, JwkKey>>,
    config: Arc<JwkConfiguration>,
}

fn keys_to_map(keys: Vec<JwkKey>) -> HashMap<String, JwkKey> {
    let mut keys_as_map = HashMap::new();
    for key in keys {
        keys_as_map.insert(String::clone(&key.kid), key);
    }
    keys_as_map
}

impl JwkVerifier {
    pub fn new(keys: Vec<JwkKey>, conf: JwkConfiguration) -> JwkVerifier {
        JwkVerifier {
            keys: Arc::new(keys_to_map(keys)),
            config: Arc::new(conf),
        }
    }

    pub fn verify(&self, token: &str) -> Option<TokenData<Claims>> {
        let token_kid = match decode_header(token).map(|header| header.kid) {
            Ok(Some(header)) => header,
            _ => return None,
        };

        let jwk_key = match self.get_key(token_kid) {
            Some(key) => key,
            _ => return None,
        };

        match self.decode_token_with_key(jwk_key, token) {
            Ok(token_data) => Some(token_data),
            _ => None,
        }
    }

    pub fn set_keys(&mut self, keys: Vec<JwkKey>) {
        let map = keys_to_map(keys);
        self.keys = Arc::new(map)
    }

    fn get_key(&self, key_id: String) -> Option<&JwkKey> {
        self.keys.get(&key_id)
    }

    fn decode_token_with_key(
        &self,
        key: &JwkKey,
        token: &str,
    ) -> Result<TokenData<Claims>, VerificationError> {
        let algorithm = match Algorithm::from_str(&key.alg) {
            Ok(alg) => alg,
            Err(_error) => return Err(VerificationError::UnkownKeyAlgorithm),
        };

        let mut validation = Validation::new(algorithm);
        validation.set_audience(&[&self.config.audience]);
        validation.set_issuer(&[self.config.issuer.clone()]);
        let key = DecodingKey::from_rsa_components(&key.n, &key.e)
            .map_err(|_| VerificationError::FailedToDecodeKey)?;

        decode::<Claims>(token, &key, &validation).map_err(|_| VerificationError::InvalidSignature)
    }
}

async fn fetch_keys(conf: &JwkConfiguration) -> Result<JwkKeys, JwkError> {
    conf.fetch_keys_for_configuration().await
}

#[derive(Clone)]
pub struct JwkAuth {
    verifier: Arc<Mutex<JwkVerifier>>,
    refresh: Arc<Mutex<(SystemTime, Duration)>>,
}

impl JwkAuth {
    pub async fn new() -> JwkAuth {
        let config = JwkConfiguration::default();
        let jwk_key_result = fetch_keys(&config).await;
        let jwk_keys: JwkKeys = match jwk_key_result {
            Ok(keys) => keys,
            Err(_) => {
                panic!("Unable to fetch jwk keys on startup! Cannot verify user tokens!")
            }
        };
        let verifier = Mutex::new(JwkVerifier::new(jwk_keys.keys, config));
        let curr_time = SystemTime::now();
        Self {
            verifier: Arc::new(verifier),
            refresh: Arc::new(Mutex::new((curr_time, jwk_keys.validity))),
        }
    }

    async fn fetch_and_refresh(&self) -> Option<JwkVerifier> {
        let mut verifier_guard = self.verifier.lock().await;
        let mut refresh_guard = self.refresh.lock().await;

        if let Ok(elapsed) = refresh_guard.0.elapsed() {
            if elapsed > refresh_guard.1 {
                let jwk_key_result = fetch_keys(&verifier_guard.config).await;
                let jwk_keys: JwkKeys = match jwk_key_result {
                    Ok(keys) => keys,
                    Err(_) => {
                        error!("Unable to fetch jwk keys! Cannot verify user tokens!");
                        return None;
                    }
                };

                verifier_guard.set_keys(jwk_keys.keys);
                refresh_guard.0 = SystemTime::now();
                refresh_guard.1 = jwk_keys.validity;
                info!("Refreshed jwk keys {}s", jwk_keys.validity.as_secs());
            }
        }
        Some(verifier_guard.clone())
    }

    pub async fn verify(&self, token: &str) -> Option<TokenData<Claims>> {
        match self.fetch_and_refresh().await {
            Some(verifier) => verifier.verify(token),
            None => None,
        }
    }
}
