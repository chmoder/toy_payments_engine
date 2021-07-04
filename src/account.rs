use rust_decimal::Decimal;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};


/// This struct is used to hold the accounts details.
/// It is the one that inevitably will represent
/// each row in the output CSV text.  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "client")]
    pub id: u16,
    available: Decimal,
    held: Decimal,
    total: Decimal,
    locked: bool,
    #[serde(skip_serializing)]
    transactions: Vec<Transaction>,
}

impl Account {
    /// Create a new account with default values.
    /// 
    /// Arguments:
    ///     * `id` - the id of the account
    /// return:
    ///     a new Account object
    /// 
    /// # example
    /// ```rust
    /// mod Account;
    /// Account::new(1);
    /// ```
    pub fn new(id: u16) -> Account {
        Account {
            id: id,
            available: Decimal::new(0, 4),
            held: Decimal::new(0, 4),
            total: Decimal::new(0, 4),
            locked: false,
            transactions: Vec::new(),
        }
    }

    /// Private method to get this accounts transaction by id.  Usefull
    /// when looking for a dispute, resolve, or chargeback transaction.
    /// 
    /// Arguments:
    ///     * `id` - the id of the transaction
    /// return:
    ///     Optional Transaction.  (it is possible to not find the transaction)
    /// 
    /// # example
    /// ```rust
    /// fn dispute(&mut self, dispute_transaction: &Transaction) {
    ///    if let Some(transaction) = self.get_transaction_by_id(dispute_transaction.id) {
    ///        self.available -= transaction.amount;
    ///        self.held += transaction.amount;
    ///    }
    /// }
    /// ```
    fn get_transaction_by_id(&self, id: u32) -> Option<Transaction> {
        for transaction in &self.transactions {
            if transaction.id == id {
                return Some(transaction.clone());
            }
        }
        return None;
    }

    /// Adds a transaction to the end of the `transactions` vector.
    ///
    /// Arguments:
    ///     * `transaction` - a transaction to add to this account
    /// return:
    ///     ()
    /// 
    /// # example
    /// ```rust
    /// mod Account;
    /// mod Transaction;
    /// let account = Account::new(1);
    /// account.add_transaction(Transaction::new());
    /// ```
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    /// Sets the number of decimal places to at most 4.
    ///
    /// Arguments:
    ///     
    /// return:
    ///     ()
    pub fn round(&mut self) {
        self.available = self.available.round_dp(4);
        self.held = self.held.round_dp(4);
        self.total = self.total.round_dp(4);
    }

    pub fn process_transactions(&mut self) {
        let transactions = &self.transactions.clone();

        for transaction in transactions {
            match transaction.type_.as_str() {
                "deposit" => self.deposit(&transaction),
                "withdrawal" => self.withdrawal(&transaction),
                "dispute" => self.dispute(&transaction),
                "resolve" => self.resolve(&transaction),
                "chargeback" => self.chargeback(&transaction),
                _ => {
                    eprintln!(
                        "invalid transaction type handle: {}",
                        transaction.type_.as_str()
                    );
                }
            };
        }
        &self.round();
    }

    /// Calculates the affect of a deposit on this account
    /// for this transaction.
    ///
    /// Arguments:
    ///     * `transaction` - a transaction to deposit
    /// return:
    ///     ()
    /// 
    /// # example
    /// see `process_transactions`
    fn deposit(&mut self, transaction: &Transaction) {
        self.available += transaction.amount;
        self.total += transaction.amount;
    }

    /// Calculates the affect of a withdrawal on this account
    /// for this transaction.
    ///
    /// Arguments:
    ///     * `transaction` - a transaction to withdrawal
    /// return:
    ///     ()
    /// 
    /// # example
    /// see `process_transactions`
    fn withdrawal(&mut self, transaction: &Transaction) {
        if (self.available - transaction.amount) > Decimal::new(0, 4) {
            self.available -= transaction.amount;
            self.total -= transaction.amount;
        }
    }

    /// Calculates the affect of a dispute on this account
    /// for this transaction.
    ///
    /// Arguments:
    ///     * `transaction` - a transaction to dispute
    /// return:
    ///     ()
    /// 
    /// # example
    /// see `process_transactions`
    fn dispute(&mut self, dispute_transaction: &Transaction) {
        if let Some(transaction) = self.get_transaction_by_id(dispute_transaction.id) {
            self.available -= transaction.amount;
            self.held += transaction.amount;
        }
    }

    /// Calculates the affect of a resolve on this account
    /// for this transaction.
    ///
    /// Arguments:
    ///     * `transaction` - a transaction to resolve
    /// return:
    ///     ()
    /// 
    /// # example
    /// see `process_transactions`
    fn resolve(&mut self, ressolve_transaction: &Transaction) {
        if let Some(transaction) = self.get_transaction_by_id(ressolve_transaction.id) {
            self.available += transaction.amount;
            self.held -= transaction.amount;
        }
    }

    /// Calculates the affect of a chargeback on this account
    /// for this transaction.
    ///
    /// Arguments:
    ///     * `transaction` - a transaction to chargeback
    /// return:
    ///     ()
    /// 
    /// # example
    /// see `process_transactions`
    fn chargeback(&mut self, chargeback_transaction: &Transaction) {
        if let Some(transaction) = self.get_transaction_by_id(chargeback_transaction.id) {
            self.total -= transaction.amount;
            self.held -= transaction.amount;
            self.locked = true;
        }
    }
}

#[cfg(test)]
mod test {
    use rust_decimal::Decimal;
    use std::str::FromStr;
    use super::Account; 
    use super::Transaction;  

    #[test]
    fn test_new() {
        assert_eq!(Account::new(1).id, 1);
    }

    #[test]
    fn test_add_transaction() {
        let mut account = Account::new(1);
        let t1 = Transaction::new(1);
        account.add_transaction(t1);
        assert_eq!(account.transactions.len(), 1);
    }

    #[test]
    fn test_get_transaction_by_id() {
        let mut account = Account::new(1);
        let t1 = Transaction::new(1);
        let t2 = Transaction::new(2);
        account.add_transaction(t1);
        account.add_transaction(t2);
        let t3 = account.get_transaction_by_id(2).unwrap();
        assert_eq!(t3.id, 2);
    }

    #[test]
    fn test_round() {
        let mut account = Account::new(1);
        account.total += Decimal::from_str("0.123456789").unwrap();
        account.round();
        assert_eq!(account.total.to_string(), "0.1235");
    }

    #[test]
    fn test_deposit() {
        let mut account = Account::new(1);
        let mut t1 = Transaction::new(1);
        t1.type_ = String::from("deposit");
        t1.client_id = 1;
        t1.amount = Decimal::from_str("1.25").unwrap();
        account.add_transaction(t1);
        account.process_transactions();
        assert_eq!(account.available.to_string(), "1.25");
        assert_eq!(account.total.to_string(), "1.25");
    }

    #[test]
    fn test_withdrawal() {
        let mut account = Account::new(1);

        let mut t1 = Transaction::new(1);
        t1.type_ = String::from("deposit");
        t1.client_id = 1;
        t1.amount = Decimal::from_str("2.00").unwrap();
        account.add_transaction(t1);

        let mut t2 = Transaction::new(2);
        t2.type_ = String::from("withdrawal");
        t2.client_id = 1;
        t2.amount = Decimal::from_str("1.00").unwrap();
        account.add_transaction(t2);
        
        account.process_transactions();
        assert_eq!(account.available.to_string(), "1.00");
        assert_eq!(account.total.to_string(), "1.00");
    }

    #[test]
    fn test_dispute() {
        let mut account = Account::new(1);

        let mut t1 = Transaction::new(1);
        t1.type_ = String::from("deposit");
        t1.client_id = 1;
        t1.amount = Decimal::from_str("3.00").unwrap();
        account.add_transaction(t1);

        let mut t2 = Transaction::new(2);
        t2.type_ = String::from("deposit");
        t2.client_id = 1;
        t2.amount = Decimal::from_str("2.00").unwrap();
        account.add_transaction(t2);

        let mut t3 = Transaction::new(1);
        t3.type_ = String::from("dispute");
        t3.client_id = 1;
        t3.amount = Decimal::from_str("1.00").unwrap();
        account.add_transaction(t3);
        
        account.process_transactions();

        assert_eq!(account.available.to_string(), "2.00");
        assert_eq!(account.held.to_string(), "3.00");
        assert_eq!(account.total.to_string(), "5.00");
    }
    
    #[test]
    fn test_resolve() {
        let mut account = Account::new(1);

        let mut t1 = Transaction::new(1);
        t1.type_ = String::from("deposit");
        t1.client_id = 1;
        t1.amount = Decimal::from_str("3.00").unwrap();
        account.add_transaction(t1);

        let mut t2 = Transaction::new(2);
        t2.type_ = String::from("deposit");
        t2.client_id = 1;
        t2.amount = Decimal::from_str("2.00").unwrap();
        account.add_transaction(t2);

        let mut t3 = Transaction::new(1);
        t3.type_ = String::from("dispute");
        t3.client_id = 1;
        t3.amount = Decimal::from_str("0.00").unwrap();
        account.add_transaction(t3);

        let mut t4 = Transaction::new(1);
        t4.type_ = String::from("resolve");
        t4.client_id = 1;
        t4.amount = Decimal::from_str("0.00").unwrap();
        account.add_transaction(t4);
        
        account.process_transactions();

        assert_eq!(account.available.to_string(), "5.00");
        assert_eq!(account.held.to_string(), "0.0000");
        assert_eq!(account.total.to_string(), "5.00");
    }

    #[test]
    fn test_chargeback() {
        let mut account = Account::new(1);

        let mut t1 = Transaction::new(1);
        t1.type_ = String::from("deposit");
        t1.client_id = 1;
        t1.amount = Decimal::from_str("3.00").unwrap();
        account.add_transaction(t1);

        let mut t2 = Transaction::new(2);
        t2.type_ = String::from("deposit");
        t2.client_id = 1;
        t2.amount = Decimal::from_str("2.00").unwrap();
        account.add_transaction(t2);

        let mut t3 = Transaction::new(1);
        t3.type_ = String::from("dispute");
        t3.client_id = 1;
        t3.amount = Decimal::from_str("0.00").unwrap();
        account.add_transaction(t3);

        let mut t4 = Transaction::new(1);
        t4.type_ = String::from("chargeback");
        t4.client_id = 1;
        t4.amount = Decimal::from_str("0.00").unwrap();
        account.add_transaction(t4);
        
        account.process_transactions();

        println!("{:?}", account);

        assert_eq!(account.available.to_string(), "2.00");
        assert_eq!(account.held.to_string(), "0.0000");
        assert_eq!(account.total.to_string(), "2.00");
        assert_eq!(account.locked, true);
    }
}