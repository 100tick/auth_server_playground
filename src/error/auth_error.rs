use axum::{
    extract::rejection::TypedHeaderRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::json;
use thiserror::Error;

use crate::lib::jwt::AuthTokenValidationError;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("tried to login with user not found")]
    UserNotFound,
    #[error("tried to create user duplicate")]
    DuplicateUser,
    #[error("`Authorization` header is invalid")]
    InvalidAuthHeader,
    #[error(transparent)]
    InvalidAuthToken(#[from] AuthTokenValidationError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        use AuthError::*;
        let (status, err_msg) = match self {
            UserNotFound => (StatusCode::BAD_REQUEST, self.to_string()),
            DuplicateUser => (StatusCode::BAD_REQUEST, self.to_string()),
            InvalidAuthHeader => (StatusCode::BAD_REQUEST, self.to_string()),
            InvalidAuthToken(err) => (StatusCode::BAD_REQUEST, err.to_string()),
        };

        let body = Json(json!({
            "error": err_msg,
        }));
        (status, body).into_response()
    }
}
