use crate::account::Account;
use csv;
use std::collections::BTreeMap;
use std::io;
use structopt::StructOpt;
use transaction::Transaction;

mod account;
mod transaction;

type AccountsType = BTreeMap<u16, Account>;

/// This is the struct we use to parse command line 
/// arguments and display usage / help to the user.
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

/// reads the CSV file and creates a mapping of account
/// id - Account objects.
///
/// If the reader fails to parse the CSV file for any reason
/// the application aborts.  Invalid data in the CSV file will
/// ignore those rows and print the error to STDERR.
fn populate_accounts(path: String, accounts: &mut AccountsType) {
    let mut reader = match csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(path)
    {
        Ok(reader) => reader,
        Err(err) => {
            panic!("{:?}", err);
        }
    };

    for result in reader.deserialize() {
        match result {
            Ok(result) => {
                let transaction: Transaction = result;
                let account = accounts
                    .entry(transaction.client_id)
                    .or_insert(Account::new(transaction.client_id));
                account.add_transaction(transaction);
            }
            Err(err) => {
                eprintln!("{:?}", err)
            }
        }
    }
}

/// calculates the accounts available, held, total, and locked 
/// status using the stored `account.transactions`. 
fn process_transactions(accounts: &mut AccountsType) {
    for account in accounts {
        let account = account.1;
        account.process_transactions();
    }
}

/// Writes the account statuses to STDOUT using 
/// the serde + csv crates.
fn write_account_summary(accounts: &AccountsType) {
    let mut writer = csv::Writer::from_writer(io::stdout());

    for account in (accounts).values() {
        match writer.serialize(account) {
            Ok(_item) => {}
            Err(err) => {
                eprintln!("{:?}", err);
            }
        }
    }

    writer.flush().unwrap();
}

#[tokio::main]
/// Toy Payment Engine
///
/// This CLI program processes a list of transactions
/// given a filepath to a CSV like:
/// ```csv
/// type, client, tx, amount
/// deposit, 1, 1, 1.0
/// ```
///
/// The ouput is a CSV of account statuses.
/// This is directed to STDOUT and looks like:
/// ```csv
/// client,available,held,total,locked
/// 1,1.5,0.0,1.5,false
/// ```
/// ```shell
/// usage: cargo run --transactions.csv > accounts.csv
/// ```
pub async fn main() {
    let opt = Cli::from_args();
    let filepath = opt.path.as_path().display().to_string();
    let mut accounts: BTreeMap<u16, Account> = BTreeMap::new();

    populate_accounts(filepath, &mut accounts);
    process_transactions(&mut accounts);
    write_account_summary(&accounts);
}
