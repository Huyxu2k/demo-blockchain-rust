use std::time::SystemTime;

use super::{account::{Store, Account}, address::Address,blockchain::{SyncedAccountVec,SyncedBlockVec,Blockchain}};
use serde::{Deserialize, Serialize};
use anyhow::Result;

///Transaction struct
///
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Transaction {
    pub sender: Address,
    pub tokens: u128,
    pub data: TransactionData,
    pub signature: Option<String>,
}
impl Transaction {
    pub fn new(tokens: u128,data:TransactionData) -> Transaction {
        let sender=Address::new();
        Transaction {
            sender,
            tokens,
            data,
            signature:None,
        }
    }
    pub fn sign_transaction(&mut self){
       let signature=Some(self.sender.generate_signature());
       self.signature=signature;
    }
    pub fn is_valid_transaction(&self)->bool{
        match self.signature.clone() {
            Some(sign) =>{
                if self.sender.clone().verify_address(sign).unwrap(){
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
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum TransactionData {
    CreateAccount { user: String },
    TransferTokens { receiver: Address, token: u128 },
    //ChangeStoreValue { address: Address, store: Store },
    //CreateTokens { receiver: Address, amount: u128 },
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct TransactionInfo{
    pub sender: String,
    pub tokens: u128,
    pub data: TransactionDataInfo,
    pub signature: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum TransactionDataInfo{
    CreateAccountInfo { user: String },
    TransferTokensInfo { receiver: String, token: u128 },
}
