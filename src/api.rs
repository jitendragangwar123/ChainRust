use actix_web::{web, HttpResponse, Responder, Error as ActixError};
use log::{info, debug, error, warn};
use std::sync::{Arc, Mutex};
use utoipa::OpenApi;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::models::{ErrorResponse, SuccessResponse};
use crate::block::Block;

#[derive(Clone)]
pub struct AppState {
    pub blockchain: Arc<Mutex<Blockchain>>,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_chain,
        add_block
    ),
    components(
        schemas(Transaction, Block, Blockchain, ErrorResponse, SuccessResponse)
    ),
    info(
        title = "Blockchain API",
        description = "A simple blockchain API with endpoints to retrieve the chain and add new blocks.",
        version = "1.0.0"
    )
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/chain",
    responses(
        (status = 200, description = "Retrieve the entire blockchain", body = Blockchain, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorResponse, content_type = "application/json")
    ),
    context_path = ""
)]
pub async fn get_chain(data: web::Data<AppState>) -> Result<impl Responder, ActixError> {
    info!("GET /chain requested");
    let blockchain = data.blockchain.lock().map_err(|e| {
        error!("Mutex poisoned: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Mutex poisoned: {}", e))
    })?;
    debug!("Returning blockchain with {} blocks", blockchain.chain.len());
    Ok(HttpResponse::Ok().json(&*blockchain))
}

#[utoipa::path(
    post,
    path = "/add_block",
    request_body(content = Vec<Transaction>, description = "List of transactions to add to a new block", content_type = "application/json"),
    responses(
        (status = 200, description = "Block added successfully", body = SuccessResponse, content_type = "application/json"),
        (status = 400, description = "No transactions provided", body = ErrorResponse, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorResponse, content_type = "application/json")
    ),
    context_path = ""
)]
pub async fn add_block(data: web::Data<AppState>, transactions: web::Json<Vec<Transaction>>) -> Result<impl Responder, ActixError> {
    info!("POST /add_block requested with {} transactions", transactions.len());
    let transactions = transactions.into_inner();
    if transactions.is_empty() {
        warn!("Empty transaction list received");
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            error: "No transactions provided".to_string(),
        }));
    }
    let mut blockchain = data.blockchain.lock().map_err(|e| {
        error!("Mutex poisoned: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Mutex poisoned: {}", e))
    })?;
    blockchain.add_block(transactions);
    info!("Block added successfully");
    Ok(HttpResponse::Ok().json(SuccessResponse {
        message: "Block added successfully!".to_string(),
    }))
}