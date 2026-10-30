use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::application::services::BankAccountService;
use crate::error::AppError;
use crate::interface::grpc::pb::{
    BankAccountGrpcItem, FindAccountByIdRequest, FindAccountByIdResponse, FindAllAccountsRequest,
    FindAllAccountsResponse, bank_account_grpc_service_server::BankAccountGrpcService,
};

#[derive(Clone)]
pub struct BankAccountGrpcHandler {
    service: BankAccountService,
}

impl BankAccountGrpcHandler {
    pub fn new(service: BankAccountService) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl BankAccountGrpcService for BankAccountGrpcHandler {
    async fn find_all_accounts(
        &self,
        _request: Request<FindAllAccountsRequest>,
    ) -> Result<Response<FindAllAccountsResponse>, Status> {
        let accounts = self.service.find_all().await.map_err(Status::from)?;

        let items = accounts
            .into_iter()
            .map(|acc| BankAccountGrpcItem {
                id: acc.id.to_string(),
                account_number: acc.account_number,
                owner_name: acc.owner_name,
                balance: acc.balance,
                currency: acc.currency,
            })
            .collect();

        Ok(Response::new(FindAllAccountsResponse { accounts: items }))
    }

    async fn find_account_by_id(
        &self,
        request: Request<FindAccountByIdRequest>,
    ) -> Result<Response<FindAccountByIdResponse>, Status> {
        let raw_id = request.into_inner().id;
        let id = Uuid::parse_str(&raw_id)
            .map_err(|_| Status::from(AppError::InvalidInput("invalid UUID format".into())))?;

        let acc = self.service.find_by_id(id).await.map_err(Status::from)?;

        Ok(Response::new(FindAccountByIdResponse {
            account: Some(BankAccountGrpcItem {
                id: acc.id.to_string(),
                account_number: acc.account_number,
                owner_name: acc.owner_name,
                balance: acc.balance,
                currency: acc.currency,
            }),
        }))
    }
}
