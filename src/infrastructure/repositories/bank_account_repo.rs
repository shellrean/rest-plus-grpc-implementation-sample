use crate::domain::entities::bank_account::BankAccount;
use crate::domain::repositories::BankAccountRepository;
use crate::error::AppError;
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct BankAccountPostgresRepository {
    pool: PgPool,
}

impl BankAccountPostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BankAccountRepository for BankAccountPostgresRepository {
    async fn find_all(&self) -> Result<Vec<BankAccount>, AppError> {
        let rows = sqlx::query(include_str!("sql/bank_account_repo_find_all.sql"))
            .fetch_all(&self.pool)
            .await?;

        let mut results = Vec::new();
        for r in rows {
            results.push(BankAccount {
                id: r.get("id"),
                account_number: r.get("account_number"),
                owner_name: r.get("owner_name"),
                balance: r.get("balance"),
                currency: r.get("currency"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            });
        }

        Ok(results)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<BankAccount>, AppError> {
        let row = sqlx::query(include_str!("sql/bank_account_repo_find_by_id.sql"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| BankAccount {
            id: r.get("id"),
            account_number: r.get("account_number"),
            owner_name: r.get("owner_name"),
            balance: r.get("balance"),
            currency: r.get("currency"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }
}
