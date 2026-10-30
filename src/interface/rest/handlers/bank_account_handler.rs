use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

use crate::application::dto::bank_account_dto::BankAccountResponse;
use crate::error::AppError;
use crate::interface::state::AppState;

pub async fn find_all(
    State(state): State<AppState>,
) -> Result<Json<Vec<BankAccountResponse>>, AppError> {
    let results = state.bank_account_service.find_all().await?;

    Ok(Json(results))
}

pub async fn find_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BankAccountResponse>, AppError> {
    let result = state.bank_account_service.find_by_id(id).await?;

    Ok(Json(result))
}
