use super::{
   account_balance_map::{AccountBalanceMap, self},
   block::Block, transaction::Transaction,
};
use  anyhow::Result;
use ethereum_types::U256;
use thiserror::Error;
use std::{slice::Iter,sync::{Arc,Mutex}};

pub type BlockVec = Vec<Block>;
type  SyncedBlockVec=Arc<Mutex<BlockVec>>;
type  SyncedAccountBalanceVec=Arc<Mutex<AccountBalanceMap>>;

pub const BLOCK_SUBSIDY:u64=100;


#[derive(Error,PartialEq,Debug)]
#[allow(clippy::enum_variant_names)]
pub enum BlockChainError{
   #[error("Invalid index")]
   INVALID_ID,
   #[error("Invalid previous hash")]
   INVALID_PREVIOUS_HASH,
   #[error("Invalid hash")]
   INVALID_HASH,
   #[error("Invalid difficulty")]
   INVALID_DIFFICULTY,
   #[error("Coinbase transaction not found")]
   COINBASE_TRANSACTION_NOT_FOUND,
   #[error("Invalid coinbase amount")]
   INVALID_COINBASE_AMOUNT,
}

#[derive(Debug,Clone)]
pub struct Blockchain{
    pub difficulty:u32,
    pub blocks:SyncedBlockVec,
    pub account_balances:SyncedAccountBalanceVec,
}

impl Blockchain {
    pub fn new(difficulty:u32)->Blockchain{
        let genesis_block=Blockchain::create_genesis_block();

        let blocks=vec![genesis_block];

        let synced_blocks=Arc::new(Mutex::new(blocks));
        let synced_account_balances=SyncedAccountBalanceVec::default();

        Blockchain { 
            difficulty, 
            blocks: synced_blocks, 
            account_balances:synced_account_balances
        }
    }
    fn create_genesis_block()->Block{
        let id=0;
        let nonce=0;
        let previous_hash=U256::default();
        let transactions=Vec::new();

        let mut block=Block::new(id, nonce, previous_hash, transactions);

        block.timestamp=0;
        block.hash=block.calculate_hash();

        block
    }
    pub fn get_last_block(&self)->Block{
        let blocks=self.blocks.lock().unwrap();
        blocks[blocks.len()-1].clone()
    }

    pub fn get_all_block(&self)->BlockVec{
        let blocks=self.blocks.lock().unwrap();
        blocks.clone()
    }

    pub fn add_block(&self,block:Block)->Result<()>{
        
        let mut blocks=self.blocks.lock().unwrap();
        let last=&blocks[&blocks.len()-1];
        
        if block.id!=last.id+1{
            return Err(BlockChainError::INVALID_PREVIOUS_HASH.into());
        }

        if block.previous_hash!=last.hash{
            return  Err(BlockChainError::INVALID_HASH.into());
        }

        if block.hash.leading_zeros()<self.difficulty{
            return  Err(BlockChainError::INVALID_DIFFICULTY.into());
        }

        self.update_account_balances(&block.transaction)?;
        blocks.push(block);

        Ok(())
    }
    fn update_account_balances(&self,transaction:&[Transaction])->Result<()>{
        let mut account_balances=self.account_balances.lock().unwrap();

        let new_account_balances=calculate_new_acccount_balances(&account_balances,transaction)?;

        *account_balances=new_account_balances;

        Ok(())
    }

}
fn calculate_new_acccount_balances(account_balance:&AccountBalanceMap,transacion:&[Transaction])->Result<AccountBalanceMap>{
   let mut new_account_balances=account_balance.clone();
   let mut iter=transacion.iter();

   process_coinbase(&mut new_account_balances, iter.next())?;

   process_transfers(&mut new_account_balances, iter)?;

   Ok(new_account_balances)
   
   
}
fn process_coinbase(account_balance:&mut AccountBalanceMap,coinbase:Option<&Transaction>)->Result<()>{
  let coinbase=match  coinbase {
      Some(tran)=>tran,
      None=>return  Err(BlockChainError::COINBASE_TRANSACTION_NOT_FOUND.into()),
  };

  let is_valid_amount=coinbase.amount==BLOCK_SUBSIDY;
  if !is_valid_amount{
    return  Err(BlockChainError::INVALID_COINBASE_AMOUNT.into());
  }

  account_balance.add_amount(&coinbase.receiver,coinbase.amount);
  Ok(())
}
fn process_transfers(new_account_balances:&mut AccountBalanceMap,transaction_iter:Iter<Transaction>)->Result<()>{
    for tx in transaction_iter {
        new_account_balances.transfer(&tx.sender, &tx.receiver, tx.amount)?;
    }
    Ok(())
}