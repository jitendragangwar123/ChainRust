use actix_web::{web, HttpResponse};
use utoipa::OpenApi;
use crate::models::{AppState, FaucetRequest, TransactionRequest, BalanceRequest};
use crate::blockchain::Blockchain;
use crate::block::Block;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

#[derive(OpenApi)]
#[openapi(
    paths(get_chain, add_block, faucet, add_transaction, create_wallet, check_balance),
    components(schemas(Block, Transaction, Wallet, Blockchain, FaucetRequest, TransactionRequest, BalanceRequest))
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/chain",
    responses(
        (status = 200, description = "Get the entire blockchain", body = Blockchain)
    )
)]
pub async fn get_chain(state: web::Data<AppState>) -> impl actix_web::Responder {
    let blockchain = state.blockchain.lock().unwrap();
    HttpResponse::Ok().json(&*blockchain)
}

#[utoipa::path(
    post,
    path = "/add_block",
    responses(
        (status = 200, description = "Block added successfully", body = String)
    )
)]
pub async fn add_block(state: web::Data<AppState>) -> impl actix_web::Responder {
    let mut blockchain = state.blockchain.lock().unwrap();
    let mut mempool = state.mempool.lock().unwrap();
    let transactions = mempool.drain(..).collect::<Vec<_>>();
    
    for tx in &transactions {
        if tx.verify() {
            let sender_balance = blockchain.balances.get(&tx.sender).unwrap_or(&0);
            if *sender_balance >= tx.amount {
                *blockchain.balances.entry(tx.sender.clone()).or_insert(0) -= tx.amount;
                *blockchain.balances.entry(tx.receiver.clone()).or_insert(0) += tx.amount;
            }
        }
    }

    let previous_block = blockchain.chain.last().unwrap();
    let new_block = Block::new(
        previous_block.index + 1,
        transactions,
        previous_block.hash.clone(),
    );
    blockchain.chain.push(new_block);
    HttpResponse::Ok().json("Block added")
}

#[utoipa::path(
    post,
    path = "/faucet",
    request_body = FaucetRequest,
    responses(
        (status = 200, description = "Funds added to address", body = String)
    )
)]
pub async fn faucet(state: web::Data<AppState>, req: web::Json<FaucetRequest>) -> impl actix_web::Responder {
    let mut blockchain = state.blockchain.lock().unwrap();
    const FAUCET_AMOUNT: u64 = 100;
    blockchain.add_funds(&req.address, FAUCET_AMOUNT);
    HttpResponse::Ok().json(format!("Added {} funds to {}", FAUCET_AMOUNT, req.address))
}

#[utoipa::path(
    post,
    path = "/transaction",
    request_body = TransactionRequest,
    responses(
        (status = 200, description = "Transaction added to mempool", body = String),
        (status = 400, description = "Invalid transaction or insufficient funds")
    )
)]
pub async fn add_transaction(state: web::Data<AppState>, req: web::Json<TransactionRequest>) -> impl actix_web::Responder {
    let transaction = Transaction::new(&req.sender, &req.receiver, req.amount, &req.private_key);
    if !transaction.verify() {
        return HttpResponse::BadRequest().json("Invalid transaction signature");
    }
    let blockchain = state.blockchain.lock().unwrap();
    let balance = blockchain.balances.get(&req.sender).unwrap_or(&0);
    if *balance < req.amount {
        return HttpResponse::BadRequest().json("Insufficient funds");
    }
    let mut mempool = state.mempool.lock().unwrap();
    mempool.push(transaction);
    HttpResponse::Ok().json("Transaction added to mempool")
}

#[utoipa::path(
    post,
    path = "/wallet",
    responses(
        (status = 200, description = "Wallet created successfully", body = Wallet)
    )
)]
pub async fn create_wallet(_: web::Data<AppState>) -> impl actix_web::Responder {
    let wallet = Wallet::new();
    HttpResponse::Ok().json(wallet)
}

#[utoipa::path(
    post,
    path = "/check_balance",
    request_body = BalanceRequest,
    responses(
        (status = 200, description = "Balance retrieved successfully", body = u64)
    )
)]
pub async fn check_balance(state: web::Data<AppState>, req: web::Json<BalanceRequest>) -> impl actix_web::Responder {
    let blockchain = state.blockchain.lock().unwrap();
    let balance = blockchain.get_balance(&req.address);
    HttpResponse::Ok().json(balance)
}