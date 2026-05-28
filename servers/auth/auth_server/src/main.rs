mod handlers;
mod models;
mod route;

use route::create_router;
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
    let app = create_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 4002));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Auth service running on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
