use serde::{Serialize, Deserialize};
use secp256k1::{Secp256k1, Message, ecdsa::Signature, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use hex;

#[derive(Serialize, Deserialize, Clone, utoipa::ToSchema)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: String,
}

impl Transaction {
    pub fn new(sender: &str, receiver: &str, amount: u64, private_key: &str) -> Self {
        let secp = Secp256k1::new();
        let secret_key_bytes = hex::decode(private_key).expect("Invalid private key hex");
        let secret_key = SecretKey::from_slice(&secret_key_bytes).expect("Invalid private key");
        let message = Self::create_message(sender, receiver, amount);
        let msg_hash = Sha256::digest(message.as_bytes());
        let message = Message::from_digest_slice(&msg_hash).expect("Invalid message hash");
        let signature = secp.sign_ecdsa(&message, &secret_key);
        Transaction {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
            signature: hex::encode(signature.serialize_der()),
        }
    }

    pub fn create_message(sender: &str, receiver: &str, amount: u64) -> String {
        format!("{}{}{}", sender, receiver, amount)
    }

    pub fn verify(&self) -> bool {
        let secp = Secp256k1::new();
        let public_key_bytes = match hex::decode(&self.sender) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        let public_key = match PublicKey::from_slice(&public_key_bytes) {
            Ok(key) => key,
            Err(_) => return false,
        };
        let message = Self::create_message(&self.sender, &self.receiver, self.amount);
        let msg_hash = Sha256::digest(message.as_bytes());
        let message = match Message::from_digest_slice(&msg_hash) {
            Ok(msg) => msg,
            Err(_) => return false,
        };
        let signature_bytes = match hex::decode(&self.signature) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        let signature = match Signature::from_der(&signature_bytes) {
            Ok(sig) => sig,
            Err(_) => return false,
        };
        secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
    }
}