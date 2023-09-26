use chrono::Utc;
use crypto::{sha2::Sha256, digest::Digest};
use ethereum_types::U256;
use  serde::{Deserialize,Serialize};
use  super::{transaction::Transaction};
use anyhow::{Result, Ok};

///Block Struct
/// 
/// Summary
#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Block{
   pub id:u64,
   pub timestamp:i64,
   pub transactions:Vec<Transaction>,
   pub hash:U256,
   pub previous_hash:U256,
   pub nonce:u64,
}
impl Block {
    pub fn new(id:u64,nonce:u64,previous_hash:U256,transactions:Vec<Transaction>)->Block{
        let mut block=Block{ 
                   id, 
                   timestamp:Utc::now().timestamp_millis(), 
                   transactions, 
                   hash: U256::default(), 
                   previous_hash, 
                   nonce 
                };
        block.hash=block.calculate_hash();

        block
    }
    pub fn calculate_hash(&self)->U256{
      let mut hashable_data=self.clone();
      hashable_data.hash=U256::default();
      let serialized=serde_json::to_string(&hashable_data).unwrap();

      let mut byte=<[u8;32]>::default();
      let mut hasher=Sha256::new();
      hasher.input_str(&serialized);
      hasher.result(&mut byte);

      U256::from(byte)
    }
    pub fn get_transaction_count(&self)->usize{
      self.transactions.len()
    }
    pub fn add_transaction(&mut self,transactions:&mut Vec<Transaction>)->Result<()>{
      self.transactions.append(transactions);
      Ok(())
    }
    pub fn verify_own_hash(&mut self)->bool{
        todo!()
    }
}