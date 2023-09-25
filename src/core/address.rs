use hashes::sha2::sha256;
use secp256k1::{Secp256k1,Error as ErrorKey, SecretKey, PublicKey, Verification, ecdsa, Message, Signing};
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    str::FromStr,
};
use thiserror::Error;

const LEN_ADDRESS: usize = 32;

#[derive(Debug, Error, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum AddressError {
    #[error("Invalid format")]
    InvalidFormat,
    #[error("Invalid length")]
    InvalidLength,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddressDTO {
    pub address: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub struct Address([u8; LEN_ADDRESS]);

impl Address {
    pub fn new() ->Address{
        let mut rng = rand::thread_rng();
        let secp = Secp256k1::new();
        let key = SecretKey::new(&mut rand::thread_rng());
        Address(*key.as_ref())
    }
    pub fn generate_public_address(&mut self) -> PublicKey {
        let secp=Secp256k1::new();
        let s=SecretKey::from_str(&Self::to_hex_address(&self)).unwrap();
        let key=PublicKey::from_secret_key(&secp,&s);
        key
    }
    pub fn verify_address(self,pubkey:PublicKey)->Result<bool,ErrorKey>{
        let secp=Secp256k1::new();
        let signature=sign(&secp,self.0)?;
        let serialize_sig=signature.serialize_compact();
        let result=verify(&secp,serialize_sig, pubkey);
        match result {
            Ok(r) => {
                Ok(r)
            },
            Err(e) => {
                Err(e)
            },
        }
    }
    pub fn to_hex_address(&self)->String{
        hex::encode(self.0)
    }
    //TODO recover address 
    // pub fn recover_address()->Result<PublicKey,ErrorKey>{

    // }
}

///Function
/// 
fn verify<C: Verification>(
    secp: &Secp256k1<C>,
    sig: [u8; 64],
    pubkey:PublicKey, //[u8;33],
) -> Result<bool, ErrorKey> {
    //let secp=Secp256k1::new();
    let sig = ecdsa::Signature::from_compact(&sig)?;
    //let pubkey = PublicKey::from_slice(&pubkey)?;
    let mes=sha256::hash(b"OK").into_bytes();
    let msg=Message::from_slice(&mes)?;
    Ok(secp.verify_ecdsa(&msg, &sig, &pubkey).is_ok())
}
fn sign<C: Signing>(
    secp: &Secp256k1<C>,
    seckey: [u8;32],
) -> Result<ecdsa::Signature, ErrorKey> {
    //let secp=Secp256k1::new();
    let mes = sha256::hash(b"OK").into_bytes();
    let msg = Message::from_slice(&mes)?;
    let seckey=SecretKey::from_slice(&seckey)?;
    Ok(secp.sign_ecdsa(&msg, &seckey))
}
// fn sign_recover<C:Signing>(
//    secp: &Secp256k1<C>,
//    seckey: SecretKey,
// ) -> Result<, ErrorKey>{
//}


impl TryFrom<Vec<u8>> for Address {
    type Error = AddressError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let slice = value.as_slice();
        match slice.try_into() {
            Ok(byte) => Ok(Address(byte)),
            Err(_) => Err(AddressError::InvalidLength),
        }
    }
}
impl TryFrom<String> for Address {
    type Error = AddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match hex::decode(value) {
            Ok(val) => val.try_into(),
            Err(_) => Err(AddressError::InvalidFormat),
        }
    }
}
impl FromStr for Address {
    type Err = AddressError;

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
        write!(f, "{}", hex::encode(self.0))
    }
}

#[cfg(test)]
mod tests{
    
}