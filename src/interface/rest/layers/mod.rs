use axum::{middleware, Router};
use crate::interface::state::AppState;

pub mod rest_trace;

pub fn apply(router: Router<AppState>, state: AppState) -> Router {
    router
        .layer(middleware::from_fn(rest_trace::handle))
        .with_state(state)
}
