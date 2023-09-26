use log::info;
use serde::de::value;

use  super::transaction::Transaction;
use  serde::{Deserialize,Serialize};

use std::sync::{Arc,Mutex};
pub type SyncedTransactionVec=Arc<Mutex<Vec<Transaction>>>;

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
        Self { transactions: SyncedTransactionVec::default() }
    }
    pub fn add(&self,transaction:Transaction){
        //lock TransactionVec when excute command
        let mut transactions=self.transactions.lock().unwrap();
        transactions.push(transaction);
        info!("add transaction");
    }
    pub fn add_vec(&mut self,mut transactions_data:Vec<Transaction>){
        let mut transactions=self.transactions.lock().unwrap();
        transactions.append(&mut transactions_data);
        info!("add vec transaction");
    }
    pub fn count(&self)->usize{
        let mut transactions=self.transactions.lock().unwrap();
        transactions.len()
    }
    pub fn pop(&self)->Vec<Transaction>{

        //lock TransactionVec when excute command
        let mut transactions=self.transactions.lock().unwrap();
        transactions.pop();

        transactions.to_vec()
    }
    pub fn remove(&self,value:Transaction)->Vec<Transaction>{

        //lock TransactionVec when excute command
        let mut transactions=self.transactions.lock().unwrap();
        transactions.retain(|a| !a.eq(&value));

        transactions.to_vec()
    }
    pub fn get_transactions(&self)->Vec<Transaction>{
        let mut transactions=self.transactions.lock().unwrap();

        transactions.to_vec()
    }
}
