use axum::Router;
use crate::interface::state::AppState;

pub mod grpc;
pub mod rest;
pub mod shared;
pub mod state;

/// Builds the unified `axum::Router` combining REST and gRPC on the same port:
/// 1. `rest::routes::create_router` -> REST routes with REST-specific layers
/// 2. `grpc::routes::create_router` -> gRPC routes with gRPC-specific layers
/// 3. `shared::layers::apply`       -> Shared layers applied to both protocols
pub fn create_router(state: AppState) -> Router {
    let rest_router = rest::routes::create_router(state.clone());
    let grpc_router = grpc::routes::create_router(state);

    let merged_router = rest_router.merge(grpc_router);

    shared::layers::apply(merged_router)
}
