use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;

pub async fn handle(req: Request, next: Next) -> Response {
    println!(
        "[REST Layer] Processing HTTP REST request: {} {}",
        req.method(),
        req.uri().path()
    );
    let mut response = next.run(req).await;
    response
        .headers_mut()
        .insert("x-protocol", HeaderValue::from_static("rest-axum"));
    response
}
