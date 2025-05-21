mod wallet {
    include!("../src/wallet.rs");
}

use wallet::Wallet;
use secp256k1::{Secp256k1, PublicKey, SecretKey};
use hex;

#[test]
fn test_wallet_new() {
    let wallet = Wallet::new();
    let public_key_bytes = hex::decode(&wallet.public_key).expect("Invalid public key hex");
    let private_key_bytes = hex::decode(&wallet.private_key).expect("Invalid private key hex");

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&private_key_bytes).expect("Invalid private key");
    let public_key = PublicKey::from_slice(&public_key_bytes).expect("Invalid public key");

    let derived_public_key = PublicKey::from_secret_key(&secp, &secret_key);
    assert_eq!(public_key, derived_public_key, "Public key should match derived key");
    assert_eq!(wallet.private_key.len(), 64, "Private key should be 32 bytes (64 hex chars)");
    assert_eq!(wallet.public_key.len(), 66, "Public key should be 33 bytes (66 hex chars)");
}