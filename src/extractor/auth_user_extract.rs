use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};
use axum_extra::extract::cookie::SignedCookieJar;

use crate::lib::jwt::JwtStore;
use crate::{
    config::constants::AUTH_COOKIE_NAME, error::AuthError, util::cookie::create_auth_cookie,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AuthUser {
    pub user_id: Option<i64>,
}

#[async_trait]
impl<B> FromRequest<B> for AuthUser
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<AuthUser, Self::Rejection> {
        let jar = req.extensions().get::<SignedCookieJar>().unwrap();
        let mut auth_user = AuthUser { user_id: None };

        //
        // if auth_cookie exists
        //
        if let Some(auth_cookie) = jar.get(AUTH_COOKIE_NAME) {
            let auth_token = auth_cookie.value();
            let jwt_store = JwtStore::get();
            let auth_claims = jwt_store.verify_auth_token(auth_token)?;
            //
            // if auth_token is dead, remove auth_cookie
            //
            if auth_claims.is_expired() && !auth_claims.is_refreshable() {
                // jar.remove(Cookie::named(AUTH_COOKIE_NAME));
                return Ok(auth_user);
            }

            //
            // refresh `auth_token` and `auth_cookie`
            //
            if auth_claims.is_expired() && auth_claims.is_refreshable() {
                let new_auth_token = jwt_store.create_auth_token(auth_claims.user_id);
                let auth_cookie = create_auth_cookie(new_auth_token);
                // jar.borrow().clone().add(auth_cookie);
            }
            //
            // input data into auth_user
            //
            auth_user.user_id = Some(auth_claims.user_id);
        }

        Ok(auth_user)
    }
}
