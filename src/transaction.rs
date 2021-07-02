use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

/// This struct holds information about a transaction
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    pub id: u32,
    pub amount: Decimal,
}

impl Transaction {
    /// Create a new account with default values.
    /// 
    /// Arguments:
    ///     
    /// return:
    ///     a new Transaction object
    /// 
    /// # example
    /// ```rust
    /// mod transaction;
    /// Transaction::new(1);
    /// ```
    pub fn new(self) -> Transaction {
        Transaction {
            type_: String::from(""),
            client_id: 0,
            id: 0,
            amount: Decimal::new(0, 4),
        }
    }
}