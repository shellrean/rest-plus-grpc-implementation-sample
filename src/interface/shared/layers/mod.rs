use axum::{Router, middleware};

pub mod duration_log;

pub fn apply(router: Router) -> Router {
    router.layer(middleware::from_fn(duration_log::handle))
}
