mod block {
    include!("../src/block.rs");
}
mod transaction {
    include!("../src/transaction.rs");
}

use block::Block;
use log::debug;

#[test]
fn test_genesis_block() {
    let block = Block::genesis();
    assert_eq!(block.index, 0);
    assert_eq!(block.previous_hash, "0");
    assert_eq!(block.transactions.len(), 0);
    assert_eq!(block.nonce, 0);
    assert_eq!(block.hash, block.calculate_hash());
}

#[test]
fn test_new_block() {
    let transactions = vec![];
    let previous_hash = "previous_hash".to_string();
    let block = Block::new(1, transactions.clone(), previous_hash.clone());
    assert_eq!(block.index, 1);
    assert_eq!(block.previous_hash, previous_hash);
    assert_eq!(block.transactions, transactions);
    assert_eq!(block.nonce, 0);
    assert_eq!(block.hash, block.calculate_hash());
}

#[test]
fn test_calculate_hash() {
    let block = Block::genesis();
    let expected_hash = block.calculate_hash();
    debug!("Genesis block hash: stored={}, calculated={}", block.hash, expected_hash);
    assert_eq!(block.hash, expected_hash, "Stored hash should match calculated hash");

    // Ensure hash changes with different input
    let mut modified_block = block.clone();
    modified_block.nonce = 1;
    let modified_hash = modified_block.calculate_hash();
    debug!("Modified block (nonce=1) hash: {}", modified_hash);
    assert_ne!(modified_hash, expected_hash, "Hash should change with different nonce");
}