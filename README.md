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
  - `POST /wallet`: Create a new wallet with key pair.
  - `POST /faucet`: Add funds to an address for testing.
  - `GET /check_balance`: Retrieve an address’s balance.
  - `POST /transaction`: Add a transaction to the mempool.
  - `POST /add_block`: Add a new block with transactions.
  - `GET /chain`: Retrieve the entire blockchain.

  
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
    actix-web="4.9.0"
    chrono="0.4.38" #handling dates and times
    serde={version = "1.0.210",features = ["derive"]}
    serde_json="1.0.128"
    sha2="0.10.8"
    tokio={version = "1.40.0",features = ["full"]}
    log = "0.4.22"
    env_logger = "0.11.5"
    utoipa = { version = "4.2", features = ["actix_extras"] }
    utoipa-swagger-ui = { version = "4.0", features = ["actix-web"] }
    secp256k1 = { version = "0.29", features = ["rand-std"] }
    rand = "0.8"
    hex = "0.4"

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

### POST /wallet
**Description**: Create a new wallet with key pair.
```bash
curl -X POST http://127.0.0.1:8080/wallet -H "Content-Type: application/json"
```

**Response**:
- `200 OK`: Returns the wallet as JSON.
- `500 Internal Server Error`: If the server encounters an issue (e.g., key generation failure).

### POST /faucet
**Description**: Add funds to an address for testing.

```bash
curl -X POST http://127.0.0.1:8080/faucet -H "Content-Type: application/json" -d '{"address": "<public_key>"}'
```


**Response**:
- `200 OK`: Returns a JSON string confirming funds added.
- `400 Bad Request`: If the request body is invalid (e.g., missing address).
- `500 Internal Server Error`: If the server encounters an issue (e.g., mutex poisoning).


### GET /check_balance
**Description**: Retrieve an address’s balance.
```bash
curl -X GET "http://127.0.0.1:8080/check_balance?address=<public_key>"
```

**Response**:
- `200 OK`: Returns the balance as a JSON integer.
- `400 Bad Request`: If the request body is invalid (e.g., missing address).
- `500 Internal Server Error`: If the server encounters an issue (e.g., mutex poisoning).

### POST /transaction
**Description**: Add a transaction to the mempool.

```bash
curl -X POST http://127.0.0.1:8080/transaction -H "Content-Type: application/json" -d '{"sender": "<public_key>", "receiver": "<public_key>", "amount": 50, "private_key": "<sender_private_key>"}'
```

**Response**:
- `200 OK`: Returns a JSON string confirming transaction addition.
- `400 Bad Request`: If the signature is invalid or funds are insufficient.
- `500 Internal Server Error`: If the server encounters an issue (e.g., mutex poisoning).

### POST /add_block
**Description**: Add a new block with mempool transactions.
```bash
curl -X POST http://127.0.0.1:8080/add_block
```

**Response**:
- `200 OK`: Returns a JSON string confirming block addition.
- `400 Bad Request`: If the signature is invalid or funds are insufficient.
- `500 Internal Server Error`: If the server encounters an issue (e.g., mutex poisoning).


### GET /chain
**Description**: Retrieve the entire blockchain.

```bash
curl http://127.0.0.1:8080/chain
```

**Response**:
- `200 OK`: Returns the blockchain as JSON.
- `400 Bad Request`: If the signature is invalid or funds are insufficient.
- `500 Internal Server Error`: If the server encounters an issue (e.g., mutex poisoning).


## Testing
### Running Tests
**Run all tests**:
  ```bash
  cargo test
  ```
**Run specific tests (e.g., block-related tests)**:
  ```bash
  cargo test --test block_tests
  ```
**Enable detailed logging for debugging**:
  ```bash
  RUST_LOG=trace cargo test --test block_tests -- --nocapture
  ```
