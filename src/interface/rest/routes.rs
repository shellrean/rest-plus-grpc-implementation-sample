use axum::routing::get;
use axum::Router;

use crate::interface::rest::handlers::bank_account_handler;
use crate::interface::rest::layers;
use crate::interface::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let router = Router::new()
        .route("/bank-accounts", get(bank_account_handler::find_all))
        .route("/bank-accounts/:id", get(bank_account_handler::find_by_id));

    layers::apply(router, state)
}
