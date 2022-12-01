use axum_extra::extract::cookie::Cookie;

use crate::config::constants::AUTH_COOKIE_NAME;

pub fn create_auth_cookie<'c>(auth_token: String) -> Cookie<'c> {
    Cookie::build(AUTH_COOKIE_NAME, auth_token)
        .path("/")
        .http_only(true)
        .max_age(time::Duration::days(1))
        .finish()
}

pub fn create_expired_auth_cookie<'c>() -> Cookie<'c> {
    Cookie::build(AUTH_COOKIE_NAME, "")
        .path("/")
        .http_only(true)
        .max_age(time::Duration::days(-1))
        .finish()
}
