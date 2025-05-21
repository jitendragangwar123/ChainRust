mod transaction {
    include!("../src/transaction.rs");
}
mod wallet {
    include!("../src/wallet.rs");
}

use transaction::Transaction;
use wallet::Wallet;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use hex;

#[test]
fn test_create_message() {
    let sender = "sender";
    let receiver = "receiver";
    let amount = 50;
    let message = Transaction::create_message(sender, receiver, amount);
    assert_eq!(message, "senderreceiver50");
}

#[test]
fn test_transaction_new_and_verify() {
    let wallet = Wallet::new();
    let sender = wallet.public_key.clone();
    let receiver = "02d524421eb3d7d4c8d4e66f536aa00e3760282cd476373e0a7ca7cb73044ce934".to_string();
    let amount = 50;
    let private_key = wallet.private_key.clone();

    let transaction = Transaction::new(&sender, &receiver, amount, &private_key);
    assert_eq!(transaction.sender, sender);
    assert_eq!(transaction.receiver, receiver);
    assert_eq!(transaction.amount, amount);
    assert!(transaction.verify(), "Transaction verification failed");
}

#[test]
fn test_invalid_signature() {
    let wallet = Wallet::new();
    let sender = wallet.public_key.clone();
    let receiver = "02d524421eb3d7d4c8d4e66f536aa00e3760282cd476373e0a7ca7cb73044ce934".to_string();
    let amount = 50;
    let private_key = wallet.private_key.clone();

    let mut transaction = Transaction::new(&sender, &receiver, amount, &private_key);
    transaction.signature = "invalid_signature".to_string();
    assert!(!transaction.verify(), "Verification should fail with invalid signature");
}

#[test]
fn test_specific_key_pair() {
    let private_key = "df23b08e75015e778e1b6d6b63bcd64d324fd2547fd19471ce67dc51c5123a44";
    let secp = Secp256k1::new();
    let secret_key_bytes = hex::decode(private_key).expect("Invalid private key hex");
    let secret_key = SecretKey::from_slice(&secret_key_bytes).expect("Invalid private key");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let public_key_hex = hex::encode(public_key.serialize());
    let receiver = "02d524421eb3d7d4c8d4e66f536aa00e3760282cd476373e0a7ca7cb73044ce934".to_string();
    let amount = 50;

    let transaction = Transaction::new(&public_key_hex, &receiver, amount, private_key);
    assert!(transaction.verify(), "Transaction verification failed for specific key pair");
    println!("Derived Public Key: {}", public_key_hex);
}