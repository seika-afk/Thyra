use crate::models::{ApiResponse, LoginRequest, SignupRequest};
use axum::{
    extract::Json,
    response::{IntoResponse, Redirect},
};

pub async fn home() -> impl IntoResponse {
    "HOME"
}

pub async fn signup(Json(payload): Json<SignupRequest>) -> impl IntoResponse {
    println!("Creating User {}", payload.username);
    Json(ApiResponse {
        message: "Signup Successful".to_string(),
    })
}

pub async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    if payload.username == "user" && payload.password == "123" {
        Redirect::to("/").into_response()
    } else {
        (
            axum::http::StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                message: "Invalid username or password".to_string(),
            }),
        )
            .into_response()
    }
}
