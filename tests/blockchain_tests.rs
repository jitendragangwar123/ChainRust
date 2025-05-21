mod blockchain {
    include!("../src/blockchain.rs");
}
mod block {
    include!("../src/block.rs");
}
mod transaction {
    include!("../src/transaction.rs");
}
mod wallet {
    include!("../src/wallet.rs");
}

use blockchain::Blockchain;
use block::Block;
use transaction::Transaction;
use wallet::Wallet;

#[test]
fn test_new_blockchain() {
    let blockchain = Blockchain::new();
    assert_eq!(blockchain.chain.len(), 1);
    assert_eq!(blockchain.chain[0].index, 0);
    assert_eq!(blockchain.balances.len(), 0);
}

#[test]
fn test_add_funds() {
    let mut blockchain = Blockchain::new();
    let address = "test_address";
    blockchain.add_funds(address, 100);
    assert_eq!(blockchain.balances.get(address), Some(&100));
    blockchain.add_funds(address, 50);
    assert_eq!(blockchain.balances.get(address), Some(&150));
}

#[test]
fn test_get_balance() {
    let mut blockchain = Blockchain::new();
    let address = "test_address";
    assert_eq!(blockchain.get_balance(address), 0);
    blockchain.add_funds(address, 200);
    assert_eq!(blockchain.get_balance(address), 200);
}

#[test]
fn test_is_chain_valid() {
    let mut blockchain = Blockchain::new();
    assert!(blockchain.is_chain_valid());

    let wallet = Wallet::new();
    let transaction = Transaction::new(
        &wallet.public_key,
        "receiver",
        50,
        &wallet.private_key,
    );
    let new_block = Block::new(
        1,
        vec![transaction],
        blockchain.chain[0].hash.clone(),
    );
    blockchain.chain.push(new_block);
    assert!(blockchain.is_chain_valid());

    // Invalidate the chain
    blockchain.chain[1].hash = "invalid_hash".to_string();
    assert!(!blockchain.is_chain_valid());
}

#[test]
fn test_save_and_load_from_file() {
    let mut blockchain = Blockchain::new();
    blockchain.add_funds("test_address", 100);
    let filename = "test_blockchain.json";
    blockchain.save_to_file(filename).unwrap();
    let loaded_blockchain = Blockchain::load_from_file(filename).unwrap();
    assert_eq!(loaded_blockchain.chain.len(), 1);
    assert_eq!(loaded_blockchain.get_balance("test_address"), 100);
    std::fs::remove_file(filename).unwrap();
}