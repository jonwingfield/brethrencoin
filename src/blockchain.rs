use std::time::SystemTime;
use std::fmt::{Display, Formatter, Result};
use sha2::{Sha256, Digest};
use base64ct::{Base64, Encoding};

pub struct Block {
    index: i64,
    timestamp: u128,
    transactions: Vec<Transaction>,
    previous_hash: Option<String>,
}

pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u128, // TODO: bigint?
}

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Block {
    pub fn new(prev: &Block, transactions: Vec<Transaction>) -> Block {
        Block {
            transactions,
            index: prev.index + 1,
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis(),
            previous_hash: Some(prev.hash()),
        }
    }

    fn initial() -> Block {
        Block {
            transactions: vec![],
            index: 0,
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis(),
            previous_hash: None,
        }
    }
    
    fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.to_string());
        let result = hasher.finalize();
        Base64::encode_string(&result)
    }
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u128) -> Transaction {
        Transaction {
            sender,
            recipient,
            amount,
        }
    }
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::initial()],
        }
    }

    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev = self.blocks.last().unwrap();
        let block = Block::new(&prev, transactions);
        self.blocks.push(block);
    }

}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} -> {}: {}", self.sender, self.recipient, self.amount)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,
            "Block - index: {}, timestamp: {}, previous hash: {}, transactions: \n",
            self.index, self.timestamp, self.previous_hash.as_ref().unwrap_or(&"Initial".to_string()),
        );
        for name in &self.transactions {
            write!(f, "| {}\n", name);
        }
        Ok(())
    }
}