use axum::{routing::get, Extension, Router};

use crate::config::Config;
use crate::db::connect_to_db;
use crate::repo::UserRepo;
use axum_extra::extract::cookie::Key;
// use crate::lib::cookies::cookies_test_store::{self, CookiesTestStore};
use crate::lib::jwt::JwtStore;
// use crate::handler::auth_handler::google_login;
use crate::lib::oauth::google::GoogleOAuthClient;
// use crate::router::auth_router::auth_router;
use crate::router::{auth_router::auth_router, test_router};
use std::net::SocketAddr;

pub async fn start(addr: &SocketAddr) {
    let router = setup_router().await;

    axum::Server::bind(addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

pub async fn setup_router() -> Router {
    let config = Config::get();

    let google_client_id = &config.google_oauth.client_id;
    let google_client_secret = &config.google_oauth.client_secret;
    let google_login_redirect_url = &config.google_oauth.login_redirect_url;
    let google_create_user_redirect_url = &config.google_oauth.create_user_redirect_url;

    let google_oauth_client = GoogleOAuthClient::new(
        google_client_id,
        google_client_secret,
        google_login_redirect_url,
        google_create_user_redirect_url,
    );

    let cookie_secret = Key::from(config.auth.cookie_secret.as_bytes());
    let jwt_store = JwtStore::get();
    let db_pool = connect_to_db(&config.db.db_url).await;
    let user_repo = UserRepo::new(db_pool);
    // let cookies_test_store = CookiesTestStore::get();

    Router::new()
        .route("/", get(home))
        .merge(auth_router(
            jwt_store.clone(),
            google_oauth_client.clone(),
            user_repo.clone(),
            cookie_secret.clone(),
        ))
        .merge(test_router())
        .layer(Extension(cookie_secret))
        .layer(Extension(google_oauth_client))
    // .layer(CookieManagerLayer::new())
}

async fn home() -> &'static str {
    "home"
}
// pub async fn connect_to_db() -> Result<DatabaseConnection, sea_orm::error::DbErr> {
//     let db_url = env::var("PORT").expect("`DATABASE_URL` must be set");

//     let mut opt = ConnectOptions::new(db_url);
//     opt.max_connections(50)
//         .min_connections(5)
//         .connect_timeout(Duration::from_secs(8))
//         .idle_timeout(Duration::from_secs(8))
//         .max_lifetime(Duration::from_secs(8))
//         .sqlx_logging(true);

//     let db = Database::connect(opt).await?;
//     Ok(db)
// }
