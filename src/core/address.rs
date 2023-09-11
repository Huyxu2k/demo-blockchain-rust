use std::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};
use serde::{Deserialize,Serialize};
use thiserror::Error;

const LEN_ADDRESS:usize=32;


#[derive(Debug,Error,PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum AddressError{
   #[error("Invalid format")]
   INVALID_FORMAT,
   #[error("Invalid length")]
   INVALID_LENGTH,
}


#[derive(Debug,Default,Clone,Serialize,Deserialize,PartialEq,Eq,Hash)]
#[serde(try_from="String",into="String")]
pub struct Address([u8;LEN_ADDRESS]);


impl TryFrom<Vec<u8>> for Address {
    type Error = AddressError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let slice=value.as_slice();
        match slice.try_into() {
            Ok(byte)=>Ok(Address(byte)),
            Err(_)=>Err(AddressError::INVALID_LENGTH),
        }
    }
}
impl TryFrom<String> for Address {
    type Error = AddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match hex::decode(value) {
            Ok(val)=>val.try_into(),
            Err(_)=>Err(AddressError::INVALID_FORMAT)
        }
    }
}
impl FromStr for Address {
    type Err=AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Address::try_from(s.to_string())
    }
}
impl From<Address> for String {
    fn from(value: Address) -> Self {
        value.to_string()
    }
}
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f,"{}",hex::encode(self.0))
    }
}