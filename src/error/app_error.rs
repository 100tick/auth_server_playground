use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

use super::AuthError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("auth error")]
    AuthError(#[from] AuthError),
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    // ReqwestError(String),
    // DbError(St ring),
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),
    // SignUpError(String),
}

// impl From<SignUpError> for AppError {
//     fn from(err: SignUpError) -> Self {
//         AppError::SignUpError(err.to_string())
//     }
// }

// impl From<sea_orm::error::DbErr> for AppError {
//     fn from(err: sea_orm::error::DbErr) -> Self {
//         AppError::DbError(err.to_string())
//     }
// }

// impl From<reqwest::Error> for AppError {
//     fn from(err: reqwest::Error) -> Self {
//         AppError::ReqwestError(err.to_string())
//     }
// }

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use AppError::*;
        let (status_code, err_msg) = match self {
            AuthError(err) => {
                println!("auth error {:?}", err);
                let err_msg = err.to_string();
                (StatusCode::UNAUTHORIZED, err_msg.to_string())
            }
            DatabaseError(err) => {
                println!("database error {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "invalid query".to_string(),
                )
            }
            ReqwestError(err) => {
                println!("reqwest error {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "reqwest erro".to_string(),
                )
            }
        };
        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
