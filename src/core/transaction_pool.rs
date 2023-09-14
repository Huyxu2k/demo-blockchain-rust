use log::info;
use serde::de::value;

use  super::transaction::Transaction;
use  serde::{Deserialize,Serialize};

use std::sync::{Arc,Mutex};

pub type TransactionVec=Vec<Transaction>;

type SyncedTransactionVec=Arc<Mutex<TransactionVec>>;

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
        

        //lock TransactionVec when excute command
        let mut transactions=self.transactions.lock().unwrap();
        transactions.push(transaction);
        info!("transaction added");
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

///Test
/// 
/// test TransactionPool
/// 
#[cfg(test)]
mod  tests{
    use crate::core::{transaction::Transaction, address::Address};

    use super::{TransactionVec, TransactionPool};
    fn init()->TransactionPool{
        let mut vec=TransactionPool::new();
        let transaction1=Transaction::new(
                         Address::try_from("f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e".to_string()).unwrap(),
                         Address::try_from("f7abe7c8bfdfb4ab9810b515057c8ad546ed50c3deb235a9a5f3ccb070a28f10".to_string()).unwrap(),
                         10);
        let transaction2=Transaction::new(
                         Address::try_from("51df097c03c0a6e64e54a6fce90cb6968adebd85955917ed438e3d3c05f2f00f".to_string()).unwrap(),
                         Address::try_from("b4f8293fb123ef3ff9ad49e923f4afc732774ee2bfdc3b278a359b54473c2277".to_string()).unwrap(),
                         10);
        vec.add(transaction1);
        vec.add(transaction2);
       vec
    }
    
    #[test]
    fn test_add(){
        let mut tranvec= init();
        let transaction3=Transaction::new(
            Address::try_from("f7abe7c8bfdfb4ab9810b515057c8ad546ed50c3deb235a9a5f3ccb070a28f10".to_string()).unwrap(),
            Address::try_from("b4f8293fb123ef3ff9ad49e923f4afc732774ee2bfdc3b278a359b54473c2277".to_string()).unwrap(),
            10);
        tranvec.add(transaction3);
        assert_eq!(tranvec.count(),3)
    }
    #[test]
    fn test_remove(){
        let mut tranvec= init();
        let transaction2=Transaction::new(
            Address::try_from("51df097c03c0a6e64e54a6fce90cb6968adebd85955917ed438e3d3c05f2f00f".to_string()).unwrap(),
            Address::try_from("b4f8293fb123ef3ff9ad49e923f4afc732774ee2bfdc3b278a359b54473c2277".to_string()).unwrap(),
            10);
        tranvec.remove(transaction2);
        assert_eq!(tranvec.count(),1);
    }
}