use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::fs::{self, write};
use log::{info, warn, error};
use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Blockchain {
    /// List of blocks in the blockchain
    pub chain: Vec<Block>,
    /// Mining difficulty (number of leading zeros required in hash)
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        info!("Creating new blockchain with genesis block");
        let mut genesis_block = Block::new(vec![], "0".to_string());
        genesis_block.mine_block(2);
        Blockchain {
            chain: vec![genesis_block],
            difficulty: 2,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        if transactions.is_empty() {
            warn!("Skipping empty transaction list");
            return;
        }
        info!("Adding block with {} transactions", transactions.len());
        let previous_hash = self.chain.last().expect("Chain should have genesis block").hash.clone();
        let mut new_block = Block::new(transactions, previous_hash);
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
        info!("Block added to chain, new chain length: {}", self.chain.len());
    }

    pub fn is_chain_valid(&self) -> bool {
        info!("Validating blockchain with {} blocks", self.chain.len());
        for (i, block) in self.chain.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let previous_block = &self.chain[i - 1];
            if block.hash != block.calculate_hash() {
                error!("Invalid hash for block {}", i);
                return false;
            }
            if block.previous_hash != previous_block.hash {
                error!("Invalid previous hash for block {}", i);
                return false;
            }
        }
        info!("Blockchain is valid");
        true
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        info!("Saving blockchain to file: {}", filename);
        let data = serde_json::to_string_pretty(self)?;
        write(filename, &data)?;
        info!("Blockchain saved successfully");
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> std::io::Result<Self> {
        info!("Loading blockchain from file: {}", filename);
        let data = fs::read_to_string(filename)?;
        let blockchain: Self = serde_json::from_str(&data)?;
        info!("Blockchain loaded successfully, {} blocks", blockchain.chain.len());
        Ok(blockchain)
    }
}