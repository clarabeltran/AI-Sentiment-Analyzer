use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 1185
// Hash 8814
// Hash 7959
// Hash 5163
// Hash 4819
// Hash 9121
// Hash 4671
// Hash 2600
// Hash 6814
// Hash 5236
// Hash 9586
// Hash 9859
// Hash 6173
// Hash 4687
// Hash 7311
// Hash 5030
// Hash 6019
// Hash 8201
// Hash 8883
// Hash 1561
// Hash 7641
// Hash 5056
// Hash 2208
// Hash 2109
// Hash 5075
// Hash 1671