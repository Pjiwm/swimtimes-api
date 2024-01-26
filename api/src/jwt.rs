use std::env;

use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub aud: String,
    pub role: String,
}

pub struct JwtVerifier {
    decoding_key: DecodingKey,
    validator: Validation,
}
impl Default for JwtVerifier {
    fn default() -> Self {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let mut validator = Validation::default();
        validator.set_audience(&["authenticated"]);
        Self {
            decoding_key: DecodingKey::from_secret(jwt_secret.as_ref()),
            validator,
        }
    }
}

impl JwtVerifier {
    pub fn check_claims(&self, token: String) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(&token, &self.decoding_key, &self.validator);
        token_data.map(|data| data.claims)
    }
}
