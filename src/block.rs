use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Clone, utoipa::ToSchema)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp: Utc::now().timestamp(),
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn genesis() -> Self {
        Block::new(0, vec![], "0".to_string())
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}",
            self.index,
            self.timestamp,
            serde_json::to_string(&self.transactions).unwrap(),
            self.previous_hash
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
}