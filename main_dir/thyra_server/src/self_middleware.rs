use axum::{extract::Request, extract::State, middleware::Next, response::Response};

use std::{sync::atomic::Ordering, time::Instant};

use crate::AppState;

pub async fn timing_middleware(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed();
    println!("Request took {:?}", duration);
    response
}

pub async fn req_counter(State(state): State<AppState>, req: Request, next: Next) -> Response {
    state.counter.fetch_add(1, Ordering::Relaxed);
    let value = state.counter.load(Ordering::Relaxed);
    println!("Total requests : {}", value);
    next.run(req).await
}
