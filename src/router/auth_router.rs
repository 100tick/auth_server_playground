use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_extra::extract::cookie::Key;

use crate::{
    handler::auth_handler::{
        google_create_user, google_create_user_callback, google_login, google_login_callback,
        login, logout,
    },
    lib::{jwt::JwtStore, oauth::google::GoogleOAuthClient},
    repo::UserRepo,
};

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/auth/token/:user_id")]
// pub struct AddAuthTokenPath {
//     user_id: i64,
// }

// #[derive(Typed)]
pub fn auth_router(
    jwt_store: JwtStore,
    google_oauth_client: GoogleOAuthClient,
    user_repo: UserRepo,
    jar_key: Key,
) -> Router {
    Router::new()
        .route("/auth/login", get(login))
        .route("/auth/google/login", get(google_login))
        .route("/auth/google/login/callback", post(google_login_callback))
        .route("/auth/google/create_user", get(google_create_user))
        .route(
            "/auth/google/create_user/callback",
            post(google_login_callback),
        )
        .route("/auth/logout", get(logout))
        .layer(Extension(jwt_store))
        .layer(Extension(google_oauth_client))
        .layer(Extension(user_repo))
        .layer(Extension(jar_key))
}
