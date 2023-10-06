#[macro_use]
extern crate log;

mod utils;
mod core;
mod api;

use api::api::Api;
use crate::core::{
   miner::Miner,
   blockchain::Blockchain,
   transaction_pool::TransactionPool,
   peer::Peer,
};
use  utils::{
    execution,
    logger::init_logger,
    termination,
    config::Config,
    context::Context
};
 fn main(){
   init_logger();
   info!("starting up");

   termination::set_ctrlc_handler();

   let config=Config::read();
   let difficulty=config.difficulty;
   let context=Context{
    config,
    blockchain:Blockchain::new(difficulty),
    pool:TransactionPool::new(),
   };

   let miner=Miner::new(&context);
   let api=Api::new(&context);
   let peer=Peer::new(&context);


   execution::run_in_parallel(vec![&miner,&api,&peer]);
}
