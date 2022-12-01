use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use axum_extra::extract::cookie::{Cookie, SignedCookieJar};

use crate::{lib::oauth::google::GoogleOAuthClient, util::cookie::create_auth_cookie};
// use tower_cookies::{Cookie, Cookies};

pub fn add_cookie_fn<'c>() -> Cookie<'c> {
    Cookie::build("a", "a").finish()
}
pub async fn add_cookie(
    jar: SignedCookieJar,
    Extension(o): Extension<GoogleOAuthClient>,
) -> impl IntoResponse {
    // let url = &format!("/auth/add_token/{}", 1);
    // Redirect::temporary(url);
    // let cookie = add_cookie_fn();
    let cookie = create_auth_cookie("x".to_string());
    let j = jar.add(cookie);

    // (j, o.code_url_for_login())
    (j, Redirect::permanent("/"))
    // Html("<h1>ADDED COOKIE</h1>")
    // (StatusCode::OK, "okok").into_response()
}

pub async fn to_add_cookies() -> Html<&'static str> {
    let url = &format!("/add_cookie");
    Redirect::to(url);
    Html("<h1>TO ADD COOKIE2</h1>")
}

// pub async fn add_cookie2(cookies: Cookies) -> Html<&'static str> {
//     let c = Cookie::build("a", "a").finish();
//     cookies.add(c);
//     // let url = &format!("/auth/add_token/{}", 1);
//     // Redirect::temporary(url);
//     println!("add_cookies2");
//     Html("<h1>ADD COOKIE2</h1>")
//     // (StatusCode::OK, "okok").into_response()
// }

pub async fn remove_cookie() -> Html<&'static str> {
    let url = &format!("/auth/remove_token");
    Redirect::temporary(url);
    Html("<h1>REMOVED COOKIE</h1>")
}
