extern crate dotenv;

use dotenv::dotenv;
use  std::env;
use  std::str::FromStr;

use  crate::core::address::Address;

type  StringVec=Vec<String>;


pub struct Config{
    pub port:u16,

    pub peers:StringVec,
    pub peer_sync_ms:u64,

    pub max_blocks:u64,
    pub max_nonce:u64,
    pub difficulty:u32,
    pub tx_waiting_ms:u64,
    pub miner_address:Address,
}

impl Config {
    pub fn read()->Config{
        dotenv().ok();
        Config { 
            port: read_env_var::<u16>("PORT", 8000), 
            peers: read_vec_env_var("PEERS",",", StringVec::default()), 
            peer_sync_ms: read_env_var::<u64>("PEER_SYNC_MS", 10000), 
            max_blocks:read_env_var::<u64>("MAX_BLOCKS",0), 
            difficulty: read_env_var::<u32>("DIFFICULTY",100), 
            tx_waiting_ms: read_env_var::<u64>("TX_WAITING_MS",10000), 
            miner_address: read_env_var::<Address>("MINER_ADDRESS",Address::default()),
            max_nonce:read_env_var::<u64>("MAX_NONCE",1_000_000) , 
        }
    }
}
fn read_env_var<T:FromStr>(key:&str,default_value:T)->T{
    match env::var(key) {
        Ok(val)=>val.parse::<T>().unwrap_or(default_value),
        Err(e)=>default_value,
    }
}
fn read_vec_env_var(key:&str,separator:&str,default_value:StringVec)->StringVec{
    match env::var(key) {
        Ok(val)=>val
              .trim()
              .split_terminator(separator)
              .map(str::to_string)
              .collect(),
        Err(e)=>default_value,
    }
}