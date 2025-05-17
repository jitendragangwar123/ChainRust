# ChainRust

A simple blockchain implementation in Rust using the Actix Web framework. The application supports creating and managing a blockchain with transactions, blocks, and Proof of Work (PoW) mining. It includes a REST API with endpoints to retrieve the blockchain and add new blocks, along with Swagger documentation for easy interaction via a web interface.

## Features
- **Blockchain Core**:
  - Transactions with sender, receiver, and amount.
  - Blocks containing transactions, linked via hashes.
  - Proof of Work mining with configurable difficulty.
  - Chain validation to ensure integrity.
  - Persistence to a JSON file (`blockchain.json`).
  
- **REST API**:
  - `GET /chain`: Retrieve the entire blockchain.
  - `POST /add_block`: Add a new block with transactions.
  
- **Swagger Documentation**:
  - Interactive Swagger UI at `/swagger-ui/` for API exploration and testing.
  - OpenAPI specification available at `/api-docs/openapi.json`.

- **Modular Design**:
  - Code organized into modules for transactions, blocks, blockchain, API, and models.
- **Logging**:
  - Detailed logging using the `log` crate for debugging and monitoring.

## Prerequisites
- **Rust**: Install Rust and Cargo (recommended version: 1.81 or later) via [rustup](https://rustup.rs/).
- **Git**: For cloning the repository.
- **curl** or a similar tool (optional): For testing API endpoints.

## Setup
**Clone the Repository**:
   ```bash
   git clone https://github.com/jitendragangwar123/ChainRust.git
   cd ChainRust
   ```
**Install Dependencies**:
  - ***Ensure the following are in your Cargo.toml***:
  
    ```bash
    [dependencies]
    actix-web = "4.8"
    serde = { version = "1.0", features = ["derive"] }
    chrono = "0.4"
    sha2 = "0.10"
    log = "0.4"
    env_logger = "0.10"
    utoipa = { version = "4.2", features = ["actix_extras"] }
    utoipa-swagger-ui = { version = "4.0", features = ["actix-web"] }
    ```
**Running the Blockahin Server**:
```bash
cargo build
RUST_LOG=info cargo run
```
**Access the Swagger UI**:
- Open http://127.0.0.1:8080/swagger-ui/ in a browser.
- Use the UI to explore and test the API endpoints interactively.



## API Endpoints

### GET /chain
**Description**: Retrieve the entire blockchain.

**Response**:
- `200 OK`: Returns the blockchain as JSON.
- `500 Internal Server Error`: If the server encounters an issue (e.g., mutex poisoning).

**Example**:
```bash
curl http://127.0.0.1:8080/chain
```

### POST /add_block
**Description**: Add a new block with a list of transactions.

**Request Body**: JSON array of transactions `[{sender, receiver, amount}, ...]`.

**Response**:
- `200 OK`: Block added successfully.
- `400 Bad Request`: If no transactions are provided.
- `500 Internal Server Error`: If the server encounters an issue.

**Example**:
```bash
curl -X POST http://127.0.0.1:8080/add_block -H "Content-Type: application/json" -d '[
  {"sender": "Alice", "receiver": "Bob", "amount": 50},
  {"sender": "Bob", "receiver": "Charlie", "amount": 30}
]'
```
