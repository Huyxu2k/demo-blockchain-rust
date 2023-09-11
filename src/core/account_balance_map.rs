use  std::collections::HashMap;
use  thiserror::Error;
use super::address::Address;

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

#[derive(Debug,Default,Clone)]
pub struct AccountBalanceMap(HashMap<Address,Amount>);

impl AccountBalanceMap {
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