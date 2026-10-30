use crate::domain::entities::bank_account::BankAccount;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccountResponse {
    pub id: Uuid,
    pub account_number: String,
    pub owner_name: String,
    pub balance: i64,
    pub formatted_balance: String,
    pub currency: String,
}

impl From<BankAccount> for BankAccountResponse {
    fn from(acc: BankAccount) -> Self {
        Self {
            id: acc.id,
            account_number: acc.account_number,
            owner_name: acc.owner_name,
            balance: acc.balance,
            formatted_balance: format!("{} {}", acc.currency, acc.balance),
            currency: acc.currency,
        }
    }
}
