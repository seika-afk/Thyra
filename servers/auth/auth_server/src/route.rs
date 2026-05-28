use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::{home, login, signup};

pub fn create_router() -> Router {
    Router::new()
        .route("/auth", get(home))
        .route("/auth/ignup", post(signup))
        .route("/auth/login", post(login))
}
