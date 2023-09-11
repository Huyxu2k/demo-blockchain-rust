use serde::{Deserialize,Serialize};
use  super::address::Address;

///Transaction struct
/// 
/// -sender : address of sender
/// 
/// -receiver: address of receiver
/// 
/// -amount: value of sender send to receiver
#[derive(Debug,Clone,Deserialize,Serialize,PartialEq, Eq)]
pub struct Transaction{
    pub sender:Address,
    pub receiver:Address,
    pub amount:u64,
}
impl Transaction {
    pub fn new(sender:Address,receiver:Address,amount:u64)->Transaction{
        Transaction { sender, receiver, amount }
    }
}