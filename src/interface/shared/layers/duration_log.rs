use axum::extract::Request;
use axum::http::header::CONTENT_TYPE;
use axum::middleware::Next;
use axum::response::Response;
use tokio::time::Instant;

pub async fn handle(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let url = req.uri().clone();
    let content_type = req
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-")
        .to_string();

    let start = Instant::now();
    let response = next.run(req).await;
    let elapsed = start.elapsed();

    println!(
        "[Shared DurationLog] [{} {}] (Content-Type: {}) -> {:?}",
        method, url, content_type, elapsed
    );

    response
}
