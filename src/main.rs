mod blockchain;

use blockchain::{Blockchain, Transaction};

fn main() {
    let mut blockchain = Blockchain::new();
    let mut transactions = vec![];
    transactions.push(Transaction::new("Alice".to_string(), "Bob".to_string(), 100));
    transactions.push(Transaction::new("Bob".to_string(), "James".to_string(), 50));
    blockchain.add_block(transactions);
    for block in blockchain.blocks() {
        println!("{}", block);
    }
}
