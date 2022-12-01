use crate::config::Config;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;

use super::{
    ts::{exp_ts, now_ts},
    AuthClaims, AuthTokenValidationError,
};

static JWT_STORE: Lazy<JwtStore> = Lazy::new(JwtStore::new);

#[derive(Clone)]
pub struct JwtStore {
    jwt_secret: &'static [u8],
}

impl JwtStore {
    pub fn get() -> &'static Self {
        &JWT_STORE
    }

    fn new() -> Self {
        let jwt_secret = Config::get().auth.jwt_secret.as_bytes();

        Self { jwt_secret }
    }

    pub fn create_auth_token(&self, user_id: i64) -> String {
        self.create_auth_token_with_exp(user_id, exp_ts())
    }

    pub fn create_auth_token_with_exp(&self, user_id: i64, exp: i64) -> String {
        let claims = AuthClaims {
            user_id,
            iss: now_ts(),
            exp,
        };

        let header = Header {
            kid: Some("signing_key".to_owned()),
            alg: Algorithm::HS256,
            ..Default::default()
        };

        // TOKEN
        match encode(&header, &claims, &EncodingKey::from_secret(self.jwt_secret)) {
            Ok(token) => token,
            Err(err) => panic!("failed to create auth token {err}"),
        }
    }

    pub fn verify_auth_token(&self, token: &str) -> Result<AuthClaims, AuthTokenValidationError> {
        let auth_claims = match decode::<AuthClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => token_data.claims,
            Err(_) => return Err(AuthTokenValidationError::InvalidToken),
        };

        if auth_claims.is_expired() {
            return Err(AuthTokenValidationError::ExpiredToken);
        }

        Ok(auth_claims)
    }
}
