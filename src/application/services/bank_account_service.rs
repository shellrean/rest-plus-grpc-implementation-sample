use crate::application::dto::bank_account_dto::BankAccountResponse;
use crate::domain::repositories::BankAccountRepository;
use crate::error::AppError;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct BankAccountService {
    repo: Arc<dyn BankAccountRepository>,
}

impl BankAccountService {
    pub fn new(repo: Arc<dyn BankAccountRepository>) -> Self {
        Self { repo }
    }

    pub async fn find_all(&self) -> Result<Vec<BankAccountResponse>, AppError> {
        let results = self.repo.find_all().await?;
        Ok(results.into_iter().map(Into::into).collect())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<BankAccountResponse, AppError> {
        let account =
            self.repo.find_by_id(id).await?.ok_or_else(|| {
                AppError::NotFound(format!("bank account with id {} not found", id))
            })?;

        Ok(account.into())
    }
}
