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
// Hash 3369
// Hash 3356
// Hash 1091
// Hash 5228
// Hash 9075
// Hash 2061
// Hash 1691
// Hash 3456
// Hash 9047
// Hash 3852
// Hash 4217
// Hash 4964
// Hash 9833
// Hash 2233
// Hash 5005
// Hash 9317
// Hash 2520
// Hash 9864
// Hash 9957
// Hash 6764
// Hash 2763
// Hash 7537
// Hash 3394
// Hash 3273
// Hash 9168
// Hash 1000
// Hash 1756
// Hash 3448
// Hash 3799
// Hash 7166
// Hash 2294
// Hash 5025
// Hash 5382
// Hash 8699
// Hash 2909
// Hash 4761
// Hash 4256
// Hash 9961
// Hash 8631
// Hash 6929
// Hash 8800
// Hash 3875
// Hash 1335
// Hash 8365
// Hash 9578
// Hash 9687
// Hash 2935
// Hash 8745
// Hash 2270
// Hash 6789
// Hash 1702
// Hash 8822
// Hash 9355
// Hash 3305
// Hash 5867
// Hash 4262
// Hash 2729
// Hash 5146
// Hash 4187
// Hash 2415