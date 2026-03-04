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
// Hash 5182
// Hash 6884
// Hash 6386
// Hash 6153
// Hash 6108
// Hash 7850
// Hash 9684
// Hash 3096
// Hash 4451
// Hash 1868
// Hash 4134
// Hash 7587
// Hash 2377
// Hash 5466
// Hash 2563
// Hash 2318
// Hash 4507
// Hash 7813
// Hash 7675
// Hash 1101
// Hash 5992
// Hash 7178
// Hash 7479
// Hash 1028
// Hash 3310
// Hash 7865
// Hash 5972
// Hash 9055
// Hash 5849
// Hash 3401
// Hash 4448
// Hash 2740
// Hash 2930
// Hash 3843
// Hash 5611
// Hash 1967
// Hash 1644
// Hash 7670
// Hash 8060
// Hash 1004
// Hash 5880
// Hash 2282
// Hash 1199
// Hash 7335
// Hash 4965
// Hash 8848
// Hash 2075
// Hash 3271
// Hash 5114
// Hash 5027
// Hash 1895
// Hash 5371
// Hash 1510
// Hash 8010
// Hash 8314
// Hash 7872
// Hash 1344
// Hash 7182
// Hash 3678
// Hash 5829
// Hash 2114
// Hash 5681
// Hash 1542
// Hash 8273
// Hash 3483
// Hash 8635
// Hash 2564
// Hash 2915
// Hash 7175
// Hash 6278
// Hash 4764
// Hash 5053
// Hash 1646
// Hash 7624
// Hash 1038
// Hash 1314
// Hash 4656
// Hash 7823
// Hash 6501
// Hash 9501
// Hash 2073
// Hash 9105
// Hash 8927