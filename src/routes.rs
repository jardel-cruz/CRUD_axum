use crate::services::user_services::{find_user, register_user};
use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> Router {
    let user_routes = user_routes();
    Router::new().nest("/user", user_routes)
}

fn user_routes() -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/find/:id", get(find_user))
}
