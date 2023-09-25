use log::info;
use serde::de::value;

use  super::transaction::Transaction;
use  serde::{Deserialize,Serialize};

use std::sync::{Arc,Mutex};

pub type TransactionVec=Vec<Transaction>;

pub type SyncedTransactionVec=Arc<Mutex<TransactionVec>>;

///TransactionPool struct
/// 
///Summary: 
/// 
///Function:
#[derive(Debug,Clone)]
pub struct  TransactionPool{
    transactions:SyncedTransactionVec,
}
impl TransactionPool {
    pub fn new()->TransactionPool{
        TransactionPool { transactions: SyncedTransactionVec::default(), }
    }
    pub fn add(&self,transaction:Transaction){
        //verify transaction
        //check exist sender & receiver 


        //lock TransactionVec when excute command
        let mut transactions=self.transactions.lock().unwrap();
        transactions.push(transaction);
        info!("transaction added");
    }
    pub fn add_vec(&self,mut transactions_data:Vec<Transaction>){
        let mut transactions=self.transactions.lock().unwrap();
        transactions.append(&mut transactions_data)
    }
    pub fn count(&self)->usize{
        let mut transactions=self.transactions.lock().unwrap();
        transactions.len()
    }
    pub fn pop(&self)->TransactionVec{

        //lock TransactionVec when excute command
        let mut transactions=self.transactions.lock().unwrap();
        transactions.pop();

        transactions.to_vec()
    }
    pub fn remove(&self,value:Transaction)->TransactionVec{

        //lock TransactionVec when excute command
        let mut transactions=self.transactions.lock().unwrap();
        transactions.retain(|a| !a.eq(&value));

        transactions.to_vec()
    }
    pub fn get_transactions(&self)->TransactionVec{
        let mut transactions=self.transactions.lock().unwrap();

        transactions.to_vec()
    }
}
