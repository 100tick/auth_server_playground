use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
// use tower_cookies::{Cookie, Cookies, Key, SignedCookies};
use crate::{
    config::{constants::AUTH_COOKIE_NAME, Config},
    lib::jwt::JwtStore,
};
use axum_extra::extract::cookie::{Cookie, Key, SignedCookieJar};

pub struct AuthCookies {}

impl AuthCookies {
    pub fn add(&self, auth_token: &str) {
        let auth_cookie = Cookie::build(AUTH_COOKIE_NAME, auth_token)
            .http_only(true)
            .max_age(time::Duration::days(3))
            .finish();

        self.signed_cookies.add(auth_cookie);
    }
}

#[async_trait]
impl<B> FromRequest<B> for AuthCookies
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let key = Key::generate();

        let cookies_secret = Key::from(Config::get().auth.cookie_secret.as_bytes());
        // let jwt_store = JwtStore::get();
        // let cookies_secret = Key::from(req.extensions().get::<Config>());
        // let jwt_store = req.extensions().get::<JwtStore>().unwrap();
        let signed_cookies = req
            .extensions()
            .get::<Cookies>()
            .cloned()
            .unwrap()
            .signed(&cookies_secret);

        // let auth_cookie = cookies.get(AUTH_COOKIE_NAME);

        Ok(Self { signed_cookies })
    }
}

// use axum::{
//     async_trait,
//     extract::{FromRequest, RequestParts},
//     http::StatusCode,
// };
// use tower_cookies::{Cookie, Cookies, Key, SignedCookies};

// use crate::{
//     config::{constants::AUTH_COOKIE_NAME, Config},
//     lib::jwt::JwtStore,
// };

// pub struct AuthCookies<'c> {
//     signed_cookies: SignedCookies<'c>,
// }

// impl<'c> AuthCookies<'c> {
//     pub fn add(&self, auth_token: &str) {
//         let auth_cookie = Cookie::build(AUTH_COOKIE_NAME, auth_token)
//             .http_only(true)
//             .max_age(time::Duration::days(3))
//             .finish();

//         self.signed_cookies.add(auth_cookie);
//     }
// }

// #[async_trait]
// impl<B> FromRequest<B> for AuthCookies<'c>
// where
//     B: Send,
// {
//     type Rejection = (StatusCode, &'static str);

//     async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
//         let cookies_secret = Key::from(Config::get().auth.cookie_secret.as_bytes());
//         // let jwt_store = JwtStore::get();
//         // let cookies_secret = Key::from(req.extensions().get::<Config>());
//         // let jwt_store = req.extensions().get::<JwtStore>().unwrap();
//         let signed_cookies = req
//             .extensions()
//             .get::<Cookies>()
//             .cloned()
//             .unwrap()
//             .signed(&cookies_secret);

//         // let auth_cookie = cookies.get(AUTH_COOKIE_NAME);

//         Ok(Self { signed_cookies })
//     }
// }
