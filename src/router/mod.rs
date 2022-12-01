use axum::{routing::get, Router};

use crate::handler::test_handler;
pub mod auth_router;

pub fn test_router() -> Router {
    Router::new()
        .route("/add_cookie", get(test_handler::add_cookie))
        .route("/to_add_cookie", get(test_handler::to_add_cookies))
        // .route("/add_cookie2", get(test_handler::add_cookie2))
        .route("/remove_cookie", get(test_handler::remove_cookie))
    // .route("/remove_cookie", get(test_handler::remove_cookie))
    // .layer(Extension(cookies_test_store))
}
