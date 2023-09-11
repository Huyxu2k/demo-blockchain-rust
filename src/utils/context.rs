use super::config::Config;
use  crate::core::{blockchain::Blockchain,transaction_pool::TransactionPool};

pub struct Context{
    pub config:Config,
    pub blockchain:Blockchain,
    pub pool:TransactionPool,
}