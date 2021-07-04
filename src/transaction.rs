use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// This calls ::Default for types that use the serde
/// deserialize_with attribute
fn decimal_default_if_empy<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + Default,
{
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_else(|| T::default()))
}

/// Holds information about a transaction
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    pub id: u32,
    #[serde(deserialize_with = "decimal_default_if_empy")]
    pub amount: Decimal,
}

impl Transaction {
    /// Create a new account with default values.
    ///
    /// Arguments:
    ///     * id - the id for the transaction
    /// return:
    ///     a new Transaction object
    ///
    /// # example
    /// ```rust
    /// mod transaction;
    /// Transaction::new(1);
    /// ```
    pub fn new(id: u32) -> Transaction {
        Transaction {
            type_: String::from(""),
            client_id: 0,
            id: id,
            amount: Decimal::new(0, 4),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Transaction;

    #[test]
    fn test_new() {
        assert_eq!(Transaction::new(1).id, 1);
    }
}
