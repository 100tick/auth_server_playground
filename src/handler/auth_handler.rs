use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension,
};
use axum_extra::{
    extract::{cookie::Cookie, SignedCookieJar},
    routing::TypedPath,
};
use serde::Deserialize;

use crate::{
    config::constants::AUTH_COOKIE_NAME,
    error::{AppResult, AuthError},
    lib::{
        jwt::JwtStore,
        oauth::google::{GoogleOAuthClient, GoogleUserInfo},
    },
    model::NewUser,
    repo::UserRepo,
    util::cookie::{create_auth_cookie, create_expired_auth_cookie},
};

pub async fn google_login(
    Extension(google_oauth_client): Extension<GoogleOAuthClient>,
) -> Redirect {
    let url = google_oauth_client.code_url_for_login();
    Redirect::permanent(&url)
}

pub async fn google_create_user(
    Extension(google_oauth_client): Extension<GoogleOAuthClient>,
) -> Redirect {
    let url = google_oauth_client.code_url_for_create_user();
    Redirect::permanent(&url)
}

pub async fn google_login_callback(
    Query(code): Query<String>,
    jar: SignedCookieJar,
    Extension(google_oauth_client): Extension<GoogleOAuthClient>,
    Extension(user_repo): Extension<UserRepo>,
    Extension(jwt_store): Extension<JwtStore>,
) -> AppResult<impl IntoResponse> {
    let token = google_oauth_client.get_token_for_login(&code).await?;
    let google_user_info = google_oauth_client
        .get_user_info(&token.access_token)
        .await?;

    let user = user_repo
        .find_user_by_google_id(&google_user_info.id)
        .await?;

    if user.is_none() {
        return Err(AuthError::UserNotFound.into());
    }

    let auth_token = jwt_store.create_auth_token(user.unwrap().id);
    let auth_cookie = create_auth_cookie(auth_token);
    let jar = jar.add(auth_cookie);

    Ok((jar, Redirect::permanent("/")))
}

pub async fn google_create_user_callback(
    Query(code): Query<String>,
    jar: SignedCookieJar,
    Extension(google_oauth_client): Extension<GoogleOAuthClient>,
    Extension(user_repo): Extension<UserRepo>,
    Extension(jwt_store): Extension<JwtStore>,
) -> AppResult<impl IntoResponse> {
    let token = google_oauth_client.get_token_for_login(&code).await?;
    let GoogleUserInfo {
        id,
        email,
        name,
        first_name,
        last_name,
        picture,
        locale,
    } = google_oauth_client
        .get_user_info(&token.access_token)
        .await?;

    let is_user_exists = user_repo.is_user_exists_by_google_id(&id).await?;
    if is_user_exists {
        return Err(AuthError::DuplicateUser.into());
    }

    let new_user = NewUser {
        google_id: id,
        email,
        name,
        first_name,
        last_name,
        avatar_url: picture,
        locale,
    };

    let user_id = user_repo.create_user_and_get_id(&new_user).await?;

    let auth_token = jwt_store.create_auth_token(user_id);
    let auth_cookie = create_auth_cookie(auth_token);
    let jar = jar.add(auth_cookie);

    Ok((jar, Redirect::permanent("/")))
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/auth/login/:auth_token")]
pub struct LoginPath {
    // path_secret: String,
    auth_token: String,
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/auth/login_test")]
pub struct LoginTestPath;

pub async fn login_test(_: LoginTestPath) -> Redirect {
    println!("okok");
    let url = format!("auth/login/{}", "x12".to_string());
    Redirect::permanent(&url)
}

#[derive(Deserialize)]
pub struct LoginQuery {
    pub user_id: i64,
}

pub async fn login(
    // Query(q): Query<LoginQuery>,
    Extension(jwt_store): Extension<JwtStore>,
    jar: SignedCookieJar,
    // ) -> Result<(SignedCookieJar, Redirect), StatusCode> {
    // ) -> Result<(SignedCookieJar, Redirect), StatusCode> {
) -> AppResult<impl IntoResponse> {
    println!("login");
    let auth_token = jwt_store.create_auth_token(1);
    let auth_cookie = create_auth_cookie(auth_token);
    let jar = jar.add(auth_cookie);

    // Ok((jar, Redirect::permanent("/")))
    Ok((jar, Redirect::permanent("/")))
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/auth/logout")]
pub struct LogoutPath;

pub async fn logout(_: LogoutPath, jar: SignedCookieJar) -> AppResult<impl IntoResponse> {
    let expired_auth_cookie = create_expired_auth_cookie();
    let jar = jar.add(expired_auth_cookie);
    Ok((jar, Redirect::permanent("/")))
}
