use axum::Router;
use tonic::service::Routes as TonicRoutes;

use crate::interface::grpc::layers;
use crate::interface::grpc::pb::bank_account_grpc_service_server::BankAccountGrpcServiceServer;
use crate::interface::grpc::services::bank_account_grpc_service::BankAccountGrpcHandler;
use crate::interface::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let handler = BankAccountGrpcHandler::new(state.bank_account_service);
    let grpc_routes = TonicRoutes::new(BankAccountGrpcServiceServer::new(handler));

    layers::apply(grpc_routes)
}
