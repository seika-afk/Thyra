use axum::Json;
use serde::Serialize;
pub async fn root() -> &'static str {
    "AXUM TEST SERVER"
}

pub async fn health() -> &'static str {
    "HEALTH IS 200"
}
#[derive(Serialize)]
pub struct User {
    id: i32,
    name: String,
    age: u8,
}
pub async fn users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            age: 22,
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            age: 27,
        },
        User {
            id: 3,
            name: "Charlie".to_string(),
            age: 31,
        },
    ];

    Json(users)
}
