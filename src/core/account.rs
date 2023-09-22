use super::address::Address;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
pub type Amount = u64;

#[derive(Debug, Error, PartialEq)]
pub enum AccountError {
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Account does not exist")]
    AccountNotExist
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountInfo {
    pub address: String,
    pub amount: Amount,
}

///New struct Account
//
#[derive(Debug, Clone,Default)]
pub struct Account(HashMap<Address, Store>);
impl Account {
    pub fn create_account(&mut self) -> Result<()> {
        let mut address = Address::new();
        //let pubkey=address.generate_public_address().to_string();
        let store=Store::new();
        self.0.insert(address, store);
        Ok(())
    }
    // pub fn get_public_key_account_(&mut self)->Result<()>{
        
    // }
    pub fn transfer_tokens(&mut self,sender:&Address,receiver: &Address,tokens:u128)->Result<(),AccountError>{
        if self.check_exist_account(sender) && self.check_exist_account(receiver){
            let sender_tokens=self.get_account_tokens(sender)?;
            let receiver_tokens=self.get_account_tokens(receiver)?;
            if sender_tokens<tokens{
                return  Err(AccountError::InsufficientFunds);
            }
            self.update_account_tokens(sender.clone(), sender_tokens-tokens);
            self.update_account_tokens(receiver.clone(),receiver_tokens+tokens);
        }
        else {
            return Err(AccountError::AccountNotExist);
        }
        Ok(())
    }
    fn check_exist_account(&mut self,address:&Address)->bool{
        match  self.0.get_key_value(address){
            Some(account)=>true,
            None=>false
        }
    }
    pub fn get_account_tokens(&mut self,account:&Address)->Result<u128, AccountError>{
        match self.0.get(account) {
            Some(store) =>{
                 Ok(store.tokens)
            },
            None => Err(AccountError::AccountNotExist),
        }
    }
    pub fn update_account_tokens(&mut self,address:Address,token:u128){
        let mut store=self.0.get(&address.clone()).unwrap();
        store.tokens=token;
        self.0.insert(address,store.clone());
    }
}

/// Store data of account
#[derive(Debug, Clone)]
pub struct Store {
    //pub pubkey:String,
    pub acc_type: AccountType,
    pub tokens: u128, //amount
}
impl Store {
    pub fn new() -> Store {
        Store {
            //pubkey,
            acc_type:AccountType::User,
            tokens:1000,//default token
        }
    }
    // ///add single transaction
    // pub fn add_single_transaction_store(&mut self, data_transaction: Transaction) {
    //     self.transactions.push(data_transaction);
    //     self.update_tokens();
    // }
    // ///add multi transaction
    // pub fn add_multi_transaction_store(&mut self, data_transactions: &mut Vec<Transaction>) {
    //     self.transactions.append(data_transactions);
    //     self.update_tokens();
    // }
    // //update tokens
    // fn update_tokens(&mut self) {
    //     let mut total_token: u128;
    //     for tran in self.transactions {
    //         total_token += tran.amount;
    //     }
    //     self.tokens = total_token;
    // }
}

///Type of Account
#[derive(Debug, Clone)]
pub enum AccountType {
    User,
    Contract,
    Validator {
        correctly_validated_blocks: u128,
        incorrectly_validated_blocks: u128,
        you_get_the_idea: bool,
    },
}
