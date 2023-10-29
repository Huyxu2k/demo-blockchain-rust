use std::time::SystemTime;
use chrono::{Utc,NaiveDateTime};

use super::{account::{Store, Account}, address::Address,blockchain::{SyncedAccountVec,SyncedBlockVec,Blockchain}};
use serde::{Deserialize, Serialize};
use anyhow::Result;

///Transaction struct
///
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Transaction {
    pub tx_hash: Address,
    //pub tx_receipt_status:bool,
    pub block_height: u128,
    pub time_stamp: i64,
    pub from: Address,
    pub to:Address,
    pub value:i128,
    pub nonce:u128,
    pub input_data:Option<String>,
    pub signature: Option<String>,
}
impl Transaction {
    pub fn new(from:Address,to:Address,value:i128,input_data:Option<String>) -> Transaction {
        let time_now=Utc::now().naive_utc().timestamp_millis();
        let tx_hash=Address::new();
        Transaction {
            tx_hash: tx_hash,
            block_height: todo!(),
            time_stamp: time_now,
            from:from,
            to:to,
            value: value,
            nonce: 0,
            input_data: input_data,
            signature:None,
        }
    }
    pub fn sign_transaction(&mut self){
       let signature=Some(self.from.generate_signature());
       self.signature=signature;
    }
    pub fn is_valid_transaction(&self)->bool{
        match self.signature.clone() {
            Some(sign) =>{
                if self.from.clone().verify_address(sign).unwrap(){
                    true
                }
                else {
                    false
                }
            },
            None => false,
        }
    }
    // pub fn to_vec(&self)->Vec<Transaction>{
    //     self.to_vec()
    // }
}

//Not use
//
// #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
// pub enum TransactionData {
//     CreateAccount { user: String },
//     TransferTokens { receiver: Address, token: u128 },
//     //ChangeStoreValue { address: Address, store: Store },
//     //CreateTokens { receiver: Address, amount: u128 },
// }
// #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
// pub struct TransactionInfo{
//     pub sender: String,
//     pub tokens: u128,
//     pub data: TransactionDataInfo,
//     pub signature: Option<String>,
// }
// #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
// pub enum TransactionDataInfo{
//     CreateAccountInfo { user: String },
//     TransferTokensInfo { receiver: String, token: u128 },
// }
