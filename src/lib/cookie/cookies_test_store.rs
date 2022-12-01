use crate::config::Config;
use once_cell::sync::Lazy;
use tower_cookies::{Cookie, Cookies, Key};

const AUTH_COOKIE_TEST_NAME: &str = "auth_test_cookie";
static COOKIES_TEST_STORE: Lazy<CookiesTestStore> = Lazy::new(CookiesTestStore::new);

#[derive(Clone)]
pub struct CookiesTestStore {
    pub cookie_secret: Key,
}

impl CookiesTestStore {
    pub fn get() -> &'static Self {
        &COOKIES_TEST_STORE
    }

    fn new() -> Self {
        let cookie_secret = Config::get().auth.cookie_secret.as_bytes();
        let cookie_secret = Key::from(cookie_secret);
        Self { cookie_secret }
    }

    pub fn get_signed_auth_cookie(&self) -> Option<Cookie> {
        let cookie_manager = Cookies::default();

        cookie_manager
            .signed(&self.cookie_secret)
            .get(AUTH_COOKIE_TEST_NAME)
    }

    pub fn add_signed_auth_cookie(&self, auth_token: String) {
        let cookie_manager = Cookies::default();

        let cookie = Cookie::build(AUTH_COOKIE_TEST_NAME, auth_token)
            .http_only(true)
            .finish();
        cookie_manager.signed(&self.cookie_secret).add(cookie);
    }

    pub fn remove_signed_auth_cookie(&self) {
        let cookie_manager = Cookies::default();

        let cookie = COOKIES_TEST_STORE.get_signed_auth_cookie();
        if cookie.is_some() {
            cookie_manager.remove(cookie.unwrap());
        }
    }
}
