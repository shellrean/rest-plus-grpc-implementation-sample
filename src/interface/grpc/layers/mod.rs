use axum::{middleware, Router};
use tonic::service::Routes as TonicRoutes;

pub mod grpc_trace;

/// Converts `TonicRoutes` into `axum::Router` and applies gRPC-only layers
pub fn apply(grpc_routes: TonicRoutes) -> Router {
    grpc_routes
        .into_axum_router()
        .layer(middleware::from_fn(grpc_trace::handle))
}
