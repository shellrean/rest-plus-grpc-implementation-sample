pub mod application;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod interface;

use std::error::Error;
use std::sync::Arc;
use tracing_subscriber::util::SubscriberInitExt;

use crate::application::services::BankAccountService;
use crate::infrastructure::database::{create_pool, run_migrations};
use crate::infrastructure::repositories::bank_account_repo::BankAccountPostgresRepository;
use crate::interface::create_router;
use crate::interface::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::registry().init();

    let pool = match create_pool().await {
        Ok(p) => {
            if let Err(e) = run_migrations(&p).await {
                tracing::warn!("failed to run migrations: {}", e);
            }
            p
        }
        Err(e) => {
            tracing::error!("failed to create database pool: {}", e);
            return Err(e.into());
        }
    };

    let bank_account_repository = BankAccountPostgresRepository::new(pool);
    let bank_account_service = BankAccountService::new(Arc::new(bank_account_repository));

    let state = AppState {
        bank_account_service,
    };

    let router = create_router(state);

    let addr = "localhost:9090";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!(
        "Server running on http://{} (Serving both REST & gRPC on the same port)",
        addr
    );

    axum::serve(listener, router).await?;

    Ok(())
}
