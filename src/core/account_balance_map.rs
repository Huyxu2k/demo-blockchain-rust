use  std::{collections::HashMap, fmt::Display};
use ethereum_types::U256;
use  thiserror::Error;
use super::address::Address;
use serde::{Deserialize,Serialize};
use anyhow::Result;
use serde_json::json;
pub type Amount=u64;


#[derive(Debug,Error,PartialEq)]
pub enum AccountBalanceMapError {
    #[error("Sender account does not exit")]
    SENDER_ACCOUNT_DOSE_NOT_EXITS,
    #[error("Receiver account does not exit")]
    RECEIVER_ACCOUNT_DOSE_NOT_EXITS,
    #[error("Insufficient funds")]
    INSUFFICIENT_FUNDS,
}
#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Balance{
    pub address:String,
    pub amount:Amount,
}
impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"address:{},amount:{}",self.address.to_string(),self.amount)
    }
}

#[derive(Debug,Default,Clone)]
pub struct AccountBalanceMap(HashMap<Address,Amount>);

impl AccountBalanceMap {
    pub fn add_account(&mut self,vec:Vec<Balance>){
        for v in vec {
            let account=self.0.get_key_value(&Address::try_from(v.address.to_string()).unwrap());
            match account {
                Some(mut acc)=>{
                  self.0.insert(acc.0.clone(),  (acc.1+v.amount));
                },
                None=>{
                self.0.insert(Address::try_from(v.address.to_string()).unwrap(), v.amount);
               }
            }  
        }  
    }
    pub fn get_account_balances(&mut self)->Vec<Balance>{
        let accounts=self.0.clone().into_iter().map(|f|{
            Balance { address:f.0.to_string(), amount: f.1 }
        }).collect();
        accounts
    }
    pub fn get_balance(&mut self,address:String)->Option<Balance>{
        let binding = self.0.clone();
        let account=binding.get(&Address::try_from(address.clone()).unwrap());
        match account {
            Some(acc)=>{
                Some(Balance { address, amount:*acc })
            },
            None =>{
               None
            },
        }
        
    }
    pub fn add_amount(&mut self,receiver:&Address,amount:Amount){
        let balance=self.get_receiver_balance(receiver).unwrap();
        self.update_balance(receiver, balance+amount);
    }
    pub fn transfer(&mut self,sender:&Address,receiver:&Address,amount:Amount)->Result<(),AccountBalanceMapError>{
        let sender_balance=self.get_sender_balance(sender)?;
        let receiver_balance=self.get_receiver_balance(receiver)?;

        if sender_balance<amount{
            return Err(AccountBalanceMapError::INSUFFICIENT_FUNDS);
        }
        self.update_balance(sender,sender_balance-amount);
        self.update_balance(receiver,receiver_balance+amount);

        Ok(())
    }
    fn get_sender_balance(&self,serder:&Address)->Result<Amount,AccountBalanceMapError>{
        match self.0.get(serder) {
            Some(balance)=>Ok(*balance),
            None=>Err(AccountBalanceMapError::SENDER_ACCOUNT_DOSE_NOT_EXITS),
        }
    }
    fn get_receiver_balance(&self,receiver:&Address)->Result<Amount,AccountBalanceMapError>{
        match self.0.get(receiver) {
            Some(balance)=>Ok(*balance),
            None=>Err(AccountBalanceMapError::RECEIVER_ACCOUNT_DOSE_NOT_EXITS),
        }
    }
    fn update_balance(&mut self,address:&Address,new_balance:Amount){
        let balance=self.0.entry(address.clone()).or_insert(0);
        *balance=new_balance;
    }
    
}