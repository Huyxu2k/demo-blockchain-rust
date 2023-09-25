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
    pub fn to_vec(&self)->Vec<Transaction>{
        self.to_vec()
    }
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum TransactionData {
    CreateAccount { user: String },
    TransferTokens { receiver: Address, token: u128 },
    //ChangeStoreValue { address: Address, store: Store },
    //CreateTokens { receiver: Address, amount: u128 },
}

