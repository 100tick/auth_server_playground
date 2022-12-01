// use crate::config::Config;
// use axum_extra::extract::cookie::{Cookie, Key, SignedCookieJar};
// use once_cell::sync::Lazy;

// const AUTH_COOKIE_NAME: &str = "auth_cookie";
// static COOKIE_STORE: Lazy<CookieStore> = Lazy::new(CookieStore::new);

// pub struct CookieStore {
//     // pub cookie_secret: Key,
//     pub key: Key,
// }

// impl CookieStore {
//     pub fn get() -> Lazy<Self> {
//         COOKIE_STORE
//     }

//     fn new() -> Self {
//         let key = Key::from(Config::get().auth.cookie_secret.as_bytes());

//         // let cookie_secret = Key::from(cookie_secret);

//         Self { key }
//     }

//     pub fn create_signed_cookie_key(secret: &str) -> Key {
//         Key::from(secret.as_bytes())
//     }

//     pub fn auth_cookie_name() -> &'static str {
//         AUTH_COOKIE_NAME
//     }

//     pub fn create_auth_cookie<'c>(&self, auth_token: &'c str) -> Cookie<'c> {
//         Cookie::build(AUTH_COOKIE_NAME, auth_token).finish()
//     }

//     // pub fn get_auth_named_cookie<'c>(&self) -> Cookie<'c> {
//     pub fn create_auth_named_cookie(&self) -> Cookie {
//         Cookie::named(AUTH_COOKIE_NAME)
//     }

//     // pub fn add_signed_auth_cookie(&self auth_token: String) {

//     // }

//     // pub fn remove_signed_auth_cookie(&'static self, cookie_manager: &'static Cookies) {
//     //     // let cookie = cookie_manager.get(AUTH_COOKIE_NAME);
//     //     let cookie = self.get_signed_auth_cookie(cookie_manager);
//     //     if cookie.is_some() {
//     //         cookie_manager.remove(cookie.unwrap());
//     //     }
//     // }
// }
