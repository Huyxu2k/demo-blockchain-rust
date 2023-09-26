use  super::{
    address::Address,
    block::Block,
    blockchain::{Blockchain},
    transaction::{Transaction, TransactionData},
    transaction_pool::{TransactionPool}
};
use  crate::utils::{
    execution::{sleep_millis,Runnable},
    context::Context,
};
use  anyhow::Result;
use ethereum_types::U256;
use log::{info, error};
use  thiserror::Error;

#[derive(Error,Debug)]
pub enum MinerError{
    #[error("No valid block was mined at index `{0}`")]
    BLOCK_NOT_MINED(u64),
}

pub struct Miner{
    miner_address:Address,
    max_blocks:u64,
    max_nonce:u64,
    tx_waiting_ms:u64,
    blockchain:Blockchain,
    pool:TransactionPool,
    target:U256,
}
impl  Runnable for Miner {
    fn run(&self)->Result<()>{
        self.start()
    }
}
impl  Miner {
    pub fn new(context:&Context)->Miner{
        let target=Self::create_target(context.config.difficulty);

        Miner { 
            miner_address:context.config.miner_address.clone(), 
            max_blocks:context.config.max_blocks, 
            max_nonce: context.config.max_nonce, 
            tx_waiting_ms: context.config.tx_waiting_ms, 
            blockchain: context.blockchain.clone(), 
            pool: context.pool.clone(), 
            target
        }
    }
    pub fn start(&self)->Result<()>{
        info!("start mining with difficulty {}",self.blockchain.difficulty);

        let mut block_count=0;
        loop {
            if self.must_stop_mining(block_count){
                info!("block limit reached,stopping mining");
                return Ok(());
            }

            let transcations=self.pool.pop();
            if transcations.is_empty(){
                sleep_millis(self.tx_waiting_ms);
                continue;
            }

            let last_block=self.blockchain.get_last_block();
            let mining_result=self.mine_block(&last_block,&transcations.clone());
            match mining_result {
                Some(block)=>{
                    info!("valid block found for index {}",block.id);
                    self.blockchain.add_block(block.clone())?;
                    block_count+=1;
                }
                None=>{
                    let index=last_block.id+1;
                    error!("no valid block was foun for index {}",index);
                    return  Err(MinerError::BLOCK_NOT_MINED(index).into());
                }
            }
        }
        Ok(())
    }
    fn create_target(diffculty:u32)->U256{
        U256::MAX>>diffculty
    }
    fn must_stop_mining(&self,block_count:u64)->bool{
        self.max_blocks>0 && block_count>= self.max_blocks
    }
    fn mine_block(&self,last_block:&Block,transactions:&Vec<Transaction>)->Option<Block>{

        let coinbase=self.create_coinbase_transaction();
        let mut block_transactions=transactions.clone();
        block_transactions.insert(0, coinbase);

        for nonce in 0..self.max_nonce {
            let next_block=self.create_next_block(last_block,block_transactions.clone(),nonce);

            if next_block.hash<self.target{
                return  Some(next_block);
            }
        }
        None
    }
    fn create_next_block(&self, last_block:&Block,transactions:Vec<Transaction>,nonce:u64)->Block{
        let id=(last_block.id+1) as u64;
        let previous_hash=last_block.hash;

        Block::new(id, nonce, previous_hash, transactions)
    }
    fn create_coinbase_transaction(&self)->Transaction{
        Transaction { 
            sender: Address::default(), 
            data: TransactionData::TransferTokens { receiver: self.miner_address.clone(), token: 1 },
            signature: None,
            tokens:1, 
        }
    }
}