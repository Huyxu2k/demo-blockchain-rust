use super::{
    account::Account,
    block::Block,
    transaction::{Transaction, TransactionData},
};
use anyhow::Result;
use ethereum_types::U256;
use std::sync::{Arc, Mutex};
use thiserror::Error;

pub type SyncedBlockVec = Arc<Mutex<Vec<Block>>>;
pub type SyncedAccountVec = Arc<Mutex<Account>>;

#[derive(Error, PartialEq, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum BlockChainError {
    #[error("Invalid index")]
    InvalidId,
    #[error("Invalid previous hash")]
    // InvalidPreviousHash,
    // #[error("Invalid hash")]
    InvalidHash,
    #[error("Invalid difficulty")]
    InvalidDifficulty,
    #[error("Coinbase transaction not found")]
    CoinbaseTransactionNotFound,
    #[error("Invalid coinbase amount")]
    InvalidCoinbaseAmount,
    #[error("Invalid verify hash")]
    InvalidVerifyHash,
    #[error("Can't not create new Account ")]
    ErrorCreateAccount,
}

pub trait State {
    fn create_account() -> Result<()>;
    fn create_contract() -> Result<()>;
    fn validator() -> Result<()>;
}
#[derive(Debug, Clone)]
pub struct Blockchain {
    pub difficulty: u32,
    pub blocks: SyncedBlockVec,
    pub accounts: SyncedAccountVec,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Blockchain {
        let genesis_block = Blockchain::create_genesis_block();

        let blocks = vec![genesis_block];

        let synced_blocks = Arc::new(Mutex::new(blocks));
        let synced_accounts = SyncedAccountVec::default();

        Blockchain {
            difficulty,
            blocks: synced_blocks,
            accounts: synced_accounts,
        }
    }
    fn create_genesis_block() -> Block {
        let id = 0;
        let nonce = 0;
        let previous_hash = U256::default();
        let transactions = Vec::new();

        let mut block = Block::new(id, nonce, previous_hash, transactions);

        block.timestamp = 0;
        block.hash = block.calculate_hash();

        block
    }
    pub fn get_last_block(&self) -> Block {
        let blocks = self.blocks.lock().unwrap();
        blocks[blocks.len() - 1].clone()
    }
    pub fn create_account(&self, transaction: Vec<Transaction>) -> Result<()> {
        let mut blocks = self.blocks.lock().unwrap();
        let last_block = &blocks[&blocks.len() - 1];

        let mut block = Block::new(last_block.id + 1, 0, last_block.hash, transaction);
        block.hash = block.calculate_hash();
        block.sign_transaction()?;
        self.execute_transaction(block.clone().transactions)?;
        blocks.push(block);
        Ok(())
    }

    pub fn get_all_block(&self) -> Vec<Block> {
        let blocks = self.blocks.lock().unwrap();
        blocks.clone()
    }
    //pub fn get_all_account(&self)->

    pub fn add_block(&self, block: Block) -> Result<()> {
        let mut blocks = self.blocks.lock().unwrap();
        let last = &blocks[&blocks.len() - 1];

        if block.id != last.id + 1 {
            return Err(BlockChainError::InvalidId.into());
        }

        if block.previous_hash != last.hash {
            return Err(BlockChainError::InvalidHash.into());
        }
        //sign transaction
        block.clone().sign_transaction()?;
        
        //execute tran
        self.execute_transaction(block.clone().transactions)?;

        blocks.push(block);

        Ok(())
    }
    pub fn execute_transaction(&self, transactions: Vec<Transaction>) -> Result<()> {
        for tran in transactions {
            match tran.data {
            TransactionData::CreateAccount { user } => {
                let _result = self.accounts.lock().unwrap().create_account(user).unwrap();
            }
            TransactionData::TransferTokens { receiver, token } => {
                self.accounts
                    .lock()
                    .unwrap()
                    .transfer_tokens(&tran.sender, &receiver, token)
                    .unwrap();
            }
            // TransactionData::ChangeStoreValue { address, store } => {
            //     self.accounts.lock().unwrap().
            // },
            // TransactionData::CreateTokens { receiver, amount } => {
            //     self.accounts.lock().unwrap().create_account_tokens(, token)
            // },
        }
        }
        Ok(())
    }
    // fn update_account(&self,transaction:&[Transaction])->Result<()>{
    //     let mut account=self.accounts.lock().unwrap();

    //     let new_account=calculate_new_acccount(&account,transaction)?;

    //     *account=new_account;

    //     Ok(())
    // }
}

// fn calculate_new_acccount(account:&Account,transacion:&[Transaction])->Result<Account>{
//    let mut new_account=account.clone();
//    let mut iter=transacion.iter();
//    process_coinbase(&mut new_account_balances, iter.next())?;
//    process_transfers(&mut new_account, iter)?;

//    Ok(new_account)
// }
// fn process_coinbase(account:&mut Account,coinbase:Option<&Transaction>)->Result<()>{
//   let coinbase=match  coinbase {
//       Some(tran)=>tran,
//       None=>return  Err(BlockChainError::CoinbaseTransactionNotFound.into()),
//   };

//   let is_valid_amount=coinbase.amount==100;
//   if !is_valid_amount{
//     return  Err(BlockChainError::InvalidCoinbaseAmount.into());
//   }

//   account.add_amount(&coinbase.receiver,coinbase.amount);
//   Ok(())
// }
// fn process_transfers(new_account:&mut Account,transaction_iter:Iter<Transaction>)->Result<()>{
//     for tx in transaction_iter {
//         new_account.transfer_tokens(&tx.sender, &tx.receiver, tx.amount)?;
//     }
//     Ok(())
// }
