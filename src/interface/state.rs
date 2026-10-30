use crate::application::services::BankAccountService;

#[derive(Clone)]
pub struct AppState {
    pub bank_account_service: BankAccountService,
}
