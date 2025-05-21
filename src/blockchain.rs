use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use crate::block::Block;

#[derive(Serialize, Deserialize, Clone, utoipa::ToSchema)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub balances: HashMap<String, u64>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Block::genesis()],
            balances: HashMap::new(),
        }
    }

    pub fn add_funds(&mut self, address: &str, amount: u64) {
        *self.balances.entry(address.to_string()).or_insert(0) += amount;
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            if current.hash != current.calculate_hash() {
                return false;
            }
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }

    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let blockchain = serde_json::from_reader(file)?;
        Ok(blockchain)
    }
}