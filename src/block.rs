use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;
use log::{debug, trace};
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
        debug!(
            "Created block {}: hash={}, previous_hash={}, transactions_len={}, nonce={}",
            block.index,
            block.hash,
            block.previous_hash,
            block.transactions.len(),
            block.nonce
        );
        block
    }

    pub fn genesis() -> Self {
        Block::new(0, vec![], "0".to_string())
    }

    pub fn calculate_hash(&self) -> String {
        // Serialize fields individually to ensure consistency
        let mut hasher = Sha256::new();
        
        // Update hasher with each field as bytes
        hasher.update(self.index.to_be_bytes());
        hasher.update(self.timestamp.to_be_bytes());
        hasher.update(
            serde_json::to_vec(&self.transactions)
                .expect("Failed to serialize transactions")
        );
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.nonce.to_be_bytes());

        let hash = format!("{:x}", hasher.finalize());
        trace!(
            "Hash input for block {}: index={}, timestamp={}, transactions={:?}, previous_hash={}, nonce={}",
            self.index,
            self.index,
            self.timestamp,
            self.transactions,
            self.previous_hash,
            self.nonce
        );
        debug!("Calculated hash for block {}: {}", self.index, hash);
        hash
    }
}