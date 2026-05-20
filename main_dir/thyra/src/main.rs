use axum::middleware;
use axum::{Router, routing::get};
use reqwest;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod handlers;
use handlers::{health, root, users};

mod self_middleware;
use self_middleware::timing_middleware;

#[derive(Clone)]
struct AppState {
    my_name: String,
    client: reqwest::Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState {
        my_name: "seika".to_string(),
        client: reqwest::Client::new(),
    };

    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);
    let app: Router = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/users", get(users))
        .layer(cors)
        .layer(middleware::from_fn(timing_middleware))
        .layer(TraceLayer::new_for_http())
        .with_state(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!(
        "Server listening on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

//RUST_LOG=tower_http=debug cargo run
