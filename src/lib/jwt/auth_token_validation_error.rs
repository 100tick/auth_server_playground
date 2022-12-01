use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthTokenValidationError {
    #[error("invalid token")]
    InvalidToken,
    #[error("token is expired")]
    ExpiredToken,
    #[error("not refreshable token")]
    NotRefreshableToken,
}
