use serde::{Serialize, Deserialize};
use secp256k1::Secp256k1;
use rand::rngs::OsRng;
use hex;

#[derive(Serialize, Deserialize, Clone, utoipa::ToSchema)]
pub struct Wallet {
    pub public_key: String,
    pub private_key: String,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        Wallet {
            public_key: hex::encode(public_key.serialize()),
            private_key: hex::encode(secret_key.secret_bytes()),
        }
    }
}