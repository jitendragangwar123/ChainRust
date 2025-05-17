use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Transaction {
    /// The sender's address or identifier
    pub sender: String,
    /// The receiver's address or identifier
    pub receiver: String,
    /// The amount of value transferred
    pub amount: u64,
}