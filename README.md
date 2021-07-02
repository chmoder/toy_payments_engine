# Toy Payments Engine

Toy Payment Engine is a simple transaction processor written in Rust 🦀!  

It's function is to process transactions and build an account summary showing various balances and status.  This application runs on command line taking a filepath to a CSV file containing transactions.  The account summary output will be sent to STDOUT.

Example input file layout:
```csv
type, client, tx, amount
deposit,    1, 1, 1.1
```

Example output layout:
```csv
client,available,held,total,locked
1,1.6235,0.0000,1.6235,false
```

### usage:
`cargo run --transactions.csv > accounts.csv`
