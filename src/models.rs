use serde::Deserialize;
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct AppState {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct FaucetRequest {
    pub address: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct TransactionRequest {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub private_key: String,
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct BalanceRequest {
    pub address: String,
}