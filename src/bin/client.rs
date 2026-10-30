use tonic::Request;

pub mod pb {
    tonic::include_proto!("banking");
}
use pb::bank_account_grpc_service_client::BankAccountGrpcServiceClient;

use crate::pb::{FindAccountByIdRequest, FindAllAccountsRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://localhost:9090";
    let account_id = "11111111-1111-1111-1111-111111111111";

    let http_client = reqwest::Client::new();

    println!("=== 1. REST API: Find All Accounts (GET /bank-accounts) ===");
    let rest_all = http_client
        .get(format!("{}/bank-accounts", base_url))
        .send()
        .await?
        .text()
        .await?;
    println!("REST Output (JSON):\n{}\n", rest_all);

    println!(
        "=== 2. REST API: Find Account By ID (GET /bank-accounts/{}) ===",
        account_id
    );
    let rest_one = http_client
        .get(format!("{}/bank-accounts/{}", base_url, account_id))
        .send()
        .await?
        .text()
        .await?;
    println!("REST Single Output (JSON):\n{}\n", rest_one);

    let mut grpc_client = BankAccountGrpcServiceClient::connect(base_url).await?;

    println!("=== 3. gRPC: FindAllAccounts (Same Port 9090) ===");
    let grpc_all = grpc_client
        .find_all_accounts(Request::new(FindAllAccountsRequest {}))
        .await?;
    println!("gRPC Output (Protobuf):\n{:#?}\n", grpc_all.into_inner());

    println!("=== 4. gRPC: FindAccountById ('{}') ===", account_id);
    let grpc_one = grpc_client
        .find_account_by_id(Request::new(FindAccountByIdRequest {
            id: account_id.to_string(),
        }))
        .await?;
    println!(
        "gRPC Single Output (Protobuf):\n{:#?}\n",
        grpc_one.into_inner()
    );

    Ok(())
}
