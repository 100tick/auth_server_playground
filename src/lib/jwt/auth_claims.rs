use super::{
    ts::{exp_ts, now_ts},
    AuthTokenValidationError,
};
use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthClaims {
    pub user_id: i64,
    // sub: String,
    pub iss: i64,
    pub exp: i64,
}

impl AuthClaims {
    pub fn is_expired(&self) -> bool {
        now_ts() > self.exp
    }

    pub fn is_refreshable(&self) -> bool {
        (self.iss + Duration::days(30).num_seconds()) < now_ts()
    }

    pub fn refresh(&self) -> Result<AuthClaims, AuthTokenValidationError> {
        if !self.is_refreshable() {
            return Err(AuthTokenValidationError::NotRefreshableToken);
        }

        let claims = AuthClaims {
            user_id: self.user_id,
            iss: self.iss,
            exp: exp_ts(),
        };
        Ok(claims)
    }
}
