use axum::{body::Body, http::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::info;

pub async fn log_status(req: Request<Body>, next: Next) -> Response<Body> {
    let now = Instant::now();

    let method = req.method().clone();
    let uri = req.uri().clone();

    let response = next.run(req).await;
    let status = response.status().as_u16();
    let elapsed = now.elapsed().as_micros();

    info!("{method} {uri} → {status} ({elapsed}µs)");

    response
}
