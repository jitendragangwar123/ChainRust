use actix_web::{HttpServer, App};
use std::io;
use std::sync::{Arc, Mutex};
use log::{info, error};
use models::AppState;
use api::{get_chain, add_block, faucet, add_transaction, create_wallet, check_balance, ApiDoc};
use blockchain::Blockchain;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

mod api;
mod blockchain;
mod block;
mod transaction;
mod wallet;
mod models;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    let filename = "blockchain.json";
    info!("Starting blockchain application!");
    let blockchain: Blockchain = match Blockchain::load_from_file(filename) {
        Ok(blockchain) => {
            info!("Loaded blockchain from {} with {} blocks", filename, blockchain.chain.len());
            blockchain
        }
        Err(e) => {
            error!("Failed to load blockchain from {}: {}. Creating new blockchain.", filename, e);
            Blockchain::new()
        }
    };

    let app_state = actix_web::web::Data::new(AppState {
        blockchain: Arc::new(Mutex::new(blockchain)),
        mempool: Arc::new(Mutex::new(Vec::new())),
    });
    let app_state_clone = Arc::clone(&app_state);

    info!("Blockchain server starting on http://127.0.0.1:8080");
    info!("Swagger UI available at http://127.0.0.1:8080/swagger-ui/");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/chain", actix_web::web::get().to(get_chain))
            .route("/add_block", actix_web::web::post().to(add_block))
            .route("/faucet", actix_web::web::post().to(faucet))
            .route("/transaction", actix_web::web::post().to(add_transaction))
            .route("/wallet", actix_web::web::post().to(create_wallet))
            .route("/check_balance", actix_web::web::post().to(check_balance))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    info!("Server shutting down, validating and saving blockchain");
    let blockchain = app_state_clone.blockchain.lock().map_err(|e| {
        error!("Mutex poisoned: {}", e);
        io::Error::new(io::ErrorKind::Other, format!("Mutex poisoned: {}", e))
    })?;
    if blockchain.is_chain_valid() {
        info!("Blockchain is valid, saving to {}", filename);
        blockchain.save_to_file(filename)?;
    } else {
        error!("Blockchain is invalid, not saving");
    }
    Ok(())
}