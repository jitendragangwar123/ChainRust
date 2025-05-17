use actix_web::{web, App, Error as ActixError, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, write};
use std::sync::{Arc, Mutex};

// Transaction struct
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: u64,
}

// Block struct
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    timestamp: i64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(transactions: Vec<Transaction>, previous_hash: String) -> Self {
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

    fn calculate_hash(&self) -> String {
        debug!("Serializing block for hashing");
        let serialized = serde_json::to_string(&(
            self.timestamp,
            &self.transactions,
            &self.previous_hash,
            self.nonce,
        ))
        .expect("Failed to serialize block");
        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        let hash = format!("{:x}", hasher.finalize());
        debug!("Hash computed: {}", hash);
        hash
    }

    fn mine_block(&mut self, difficulty: usize) {
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

// Blockchain struct
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new() -> Self {
        info!("Creating new blockchain with genesis block");
        let mut genesis_block = Block::new(vec![], "0".to_string());
        genesis_block.mine_block(2);
        Blockchain {
            chain: vec![genesis_block],
            difficulty: 2,
        }
    }

    fn add_block(&mut self, transactions: Vec<Transaction>) {
        if transactions.is_empty() {
            warn!("Skipping empty transaction list");
            return;
        }
        info!("Adding block with {} transactions", transactions.len());
        let previous_hash = self
            .chain
            .last()
            .expect("Chain should have genesis block")
            .hash
            .clone();
        let mut new_block = Block::new(transactions, previous_hash);
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
        info!(
            "Block added to chain, new chain length: {}",
            self.chain.len()
        );
    }

    fn is_chain_valid(&self) -> bool {
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

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        info!("Saving blockchain to file: {}", filename);
        let data = serde_json::to_string_pretty(self)?;
        write(filename, &data)?;
        info!("Blockchain saved successfully");
        Ok(())
    }

    fn load_from_file(filename: &str) -> std::io::Result<Self> {
        info!("Loading blockchain from file: {}", filename);
        let data = fs::read_to_string(filename)?;
        let blockchain: Self = serde_json::from_str(&data)?;
        info!(
            "Blockchain loaded successfully, {} blocks",
            blockchain.chain.len()
        );
        Ok(blockchain)
    }
}

// AppState for shared state
#[derive(Clone)]
struct AppState {
    blockchain: Arc<Mutex<Blockchain>>,
}

// API endpoints
async fn get_chain(data: web::Data<AppState>) -> Result<impl Responder, ActixError> {
    info!("GET /chain requested");
    let blockchain = data.blockchain.lock().map_err(|e| {
        error!("Mutex poisoned: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Mutex poisoned: {}", e))
    })?;
    debug!(
        "Returning blockchain with {} blocks",
        blockchain.chain.len()
    );
    Ok(HttpResponse::Ok().json(&*blockchain))
}

async fn add_block(
    data: web::Data<AppState>,
    transactions: web::Json<Vec<Transaction>>,
) -> Result<impl Responder, ActixError> {
    info!(
        "POST /add_block requested with {} transactions",
        transactions.len()
    );
    let transactions = transactions.into_inner();
    if transactions.is_empty() {
        warn!("Empty transaction list received");
        return Ok(HttpResponse::BadRequest()
            .json(serde_json::json!({"error": "No transactions provided"})));
    }
    let mut blockchain = data.blockchain.lock().map_err(|e| {
        error!("Mutex poisoned: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Mutex poisoned: {}", e))
    })?;
    blockchain.add_block(transactions);
    info!("Block added successfully");
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Block added successfully!"})))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let filename = "blockchain.json";
    info!("Starting blockchain application!");
    let blockchain: Blockchain = match Blockchain::load_from_file(filename) {
        Ok(blockchain) => {
            info!(
                "Loaded blockchain from {} with {} blocks",
                filename,
                blockchain.chain.len()
            );
            blockchain
        }
        Err(e) => {
            error!(
                "Failed to load blockchain from {}: {}. Creating new blockchain.",
                filename, e
            );
            Blockchain::new()
        }
    };

    let app_state = web::Data::new(AppState {
        blockchain: Arc::new(Mutex::new(blockchain)),
    });
    let app_state_clone = Arc::clone(&app_state);

    info!("Blockchain server starting on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/chain", web::get().to(get_chain))
            .route("/add_block", web::post().to(add_block))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    info!("Server shutting down, validating and saving blockchain");
    let blockchain = app_state_clone.blockchain.lock().map_err(|e| {
        error!("Mutex poisoned: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, format!("Mutex poisoned: {}", e))
    })?;
    if blockchain.is_chain_valid() {
        info!("Blockchain is valid, saving to {}", filename);
        blockchain.save_to_file(filename)?;
    } else {
        error!("Blockchain is invalid, not saving");
    }
    Ok(())
}
