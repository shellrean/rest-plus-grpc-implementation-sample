use axum::extract::Request;
use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::Response;

pub async fn handle(req: Request, next: Next) -> Response {
    println!(
        "[gRPC Layer] Processing Tonic gRPC request: {}",
        req.uri().path()
    );
    let mut response = next.run(req).await;
    response
        .headers_mut()
        .insert("x-protocol", HeaderValue::from_static("grpc-tonic"));
    response
}
