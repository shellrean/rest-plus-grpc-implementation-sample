use crate::domain::entities::bank_account::BankAccount;
use crate::error::AppError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait BankAccountRepository: Sync + Send {
    async fn find_all(&self) -> Result<Vec<BankAccount>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<BankAccount>, AppError>;
}
