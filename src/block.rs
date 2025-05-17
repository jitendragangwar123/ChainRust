use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sha2::{Sha256, Digest};
use chrono::Utc;
use log::{info, debug, error};
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Block {
    /// Unix timestamp of block creation
    pub timestamp: i64,
    /// List of transactions in the block
    pub transactions: Vec<Transaction>,
    /// Hash of the previous block
    pub previous_hash: String,
    /// Nonce used for mining
    pub nonce: u64,
    /// SHA-256 hash of the block
    pub hash: String,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        info!("Creating new block with timestamp: {}", timestamp);
        let mut block = Block {
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        debug!("Block hash calculated: {}", block.hash);
        block
    }

    pub fn calculate_hash(&self) -> String {
        debug!("Serializing block for hashing");
        let serialized = serde_json::to_string(&(
            self.timestamp,
            &self.transactions,
            &self.previous_hash,
            self.nonce,
        )).expect("Failed to serialize block");
        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        let hash = format!("{:x}", hasher.finalize());
        debug!("Hash computed: {}", hash);
        hash
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        if difficulty > 64 {
            error!("Difficulty {} exceeds max hash length (64)", difficulty);
            panic!("Difficulty {} exceeds max hash length", difficulty);
        }
        info!("Mining block with difficulty: {}", difficulty);
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
            debug!("Mining attempt, nonce: {}, hash: {}", self.nonce, self.hash);
        }
        info!("Block mined successfully with nonce: {}", self.nonce);
    }
}