use axum::{Router, routing::get};
use std::net::SocketAddr;

use handlers::users;
mod handlers;

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/users", get(users));
    let addr = SocketAddr::from(([0, 0, 0, 0], 4001));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!(
        "Server Listening on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
