use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error as ActixError};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use sha2::{Sha256, Digest};
use std::sync::{Arc, Mutex};
use std::fs::{self, write};

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
        let mut block = Block {
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let serialized = serde_json::to_string(&(
            self.timestamp,
            &self.transactions,
            &self.previous_hash,
            self.nonce,
        )).expect("Failed to serialize block");
        let mut hasher = Sha256::new();
        hasher.update(serialized);
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        if difficulty > 64 { 
            panic!("Difficulty {} exceeds max hash length", difficulty);
        }
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
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
        let mut genesis_block = Block::new(vec![], "0".to_string());
        genesis_block.mine_block(2); 
        Blockchain {
            chain: vec![genesis_block],
            difficulty: 2,
        }
    }

    fn add_block(&mut self, transactions: Vec<Transaction>) {
        if transactions.is_empty() {
            return; 
        }
        let previous_hash = self.chain.last().expect("Chain should have genesis block").hash.clone();
        let mut new_block = Block::new(transactions, previous_hash);
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    fn is_chain_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            if i == 0 {
                continue; 
            }
            let previous_block = &self.chain[i - 1];
            if block.hash != block.calculate_hash() {
                println!("Invalid hash for block {}", i);
                return false;
            }
            if block.previous_hash != previous_block.hash {
                println!("Invalid previous hash for block {}", i);
                return false;
            }
        }
        true
    }

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        write(filename, data)?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let data = fs::read_to_string(filename)?;
        let blockchain = serde_json::from_str(&data)?;
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
    let blockchain = data.blockchain.lock().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Mutex poisoned: {}", e))
    })?;
    Ok(HttpResponse::Ok().json(&*blockchain))
}

async fn add_block(data: web::Data<AppState>, transactions: web::Json<Vec<Transaction>>) -> Result<impl Responder, ActixError> {
    let transactions = transactions.into_inner();
    if transactions.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "No transactions provided"})));
    }
    let mut blockchain = data.blockchain.lock().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Mutex poisoned: {}", e))
    })?;
    blockchain.add_block(transactions);
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Block added successfully!"})))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("Blockchain starting on http://127.0.0.1:8080");
    let filename = "blockchain.json";
    let blockchain = match Blockchain::load_from_file(filename) {
        Ok(blockchain) => blockchain,
        Err(e) => {
            println!("Failed to load blockchain from {}: {}. Creating new blockchain.", filename, e);
            Blockchain::new()
        }
    };

    let app_state = web::Data::new(AppState {
        blockchain: Arc::new(Mutex::new(blockchain)),
    });
    let app_state_clone=Arc::clone(&app_state);

    println!("Blockchain starting on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/chain", web::get().to(get_chain))
            .route("/add_block", web::post().to(add_block))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    // Save blockchain after server shutdown
    let blockchain = app_state_clone.blockchain.lock().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("Mutex poisoned: {}", e))
    })?;
    if blockchain.is_chain_valid() {
        println!("Blockchain is valid!");
        blockchain.save_to_file(filename)?;
    } else {
        println!("Blockchain is invalid! Not saving.");
    }
    Ok(())
}