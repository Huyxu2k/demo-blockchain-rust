use  std::panic;
use  super::{
    block::Block,
    blockchain::Blockchain,
};
use  crate::{utils::{
     execution::{sleep_millis,Runnable},
     context::Context,
}, api::response};
use  anyhow::Result;
use axum::http::StatusCode;
use isahc::{ReadResponseExt,Request};
use log::{info, error};


pub struct Peer{
    peer_addresser:Vec<String>,
    blockchain:Blockchain,
    peer_sync_ms:u64,
}
 
 impl Runnable for Peer {
   fn run(&self)->Result<()>{
    self.start()
   }
}
impl Peer {
    pub fn new(context:&Context)->Peer{
        Peer { 
            peer_addresser: context.config.peers.clone(), 
            blockchain: context.blockchain.clone(), 
            peer_sync_ms: context.config.peer_sync_ms
        }
     }
     pub fn start(&self)->Result<()>{
        if self.peer_addresser.is_empty(){
            info!("No peers configured ,exiting peer sync sytem");
            return Ok(());
        }
        info!("start peer system with peers:{}",self.peer_addresser.join(","));

        let mut last_sent_block_index=self.get_last_block_index();
        loop {
            self.try_receive_new_block();
            self.try_send_new_blocks(last_sent_block_index);
            last_sent_block_index=self.get_last_block_index();
            sleep_millis(self.peer_sync_ms);
        }
     }
     fn get_last_block_index(&self)->usize{
        self.blockchain.get_last_block().id as usize
     }
     fn try_receive_new_block(&self){
        for address in self.peer_addresser.iter() {
            let result=panic::catch_unwind(||{
               let new_blocks=self.get_new_blocks_from_peer(address);
               if !new_blocks.is_empty(){
                self.add_new_blocks(&new_blocks);
               }
            });

            if result.is_err(){
                error!("Could not sync blocks from peer {}",address);
            }
        }
     }
     fn add_new_blocks(&self,new_blocks:&[Block]){
        for block in new_blocks {
            let result =self.blockchain.add_block(block.clone());

            if result.is_err(){
                error!("Could not add peer block {} to the blockchain",block.id);
            }
            info!("Added new peer block {} to the blockchain",block.id);

        }
     }
     fn get_new_blocks_from_peer(&self,address:&str)->Vec<Block>{
        let our_last_index=self.blockchain.get_last_block().id as usize;

        let peer_blocks=self.get_blocks_from_peer(address);
        let peer_last_index=peer_blocks.last().unwrap().id as usize;

        if peer_last_index<=our_last_index{
            return Vec::<Block>::new();
        }
         
        let first_new=our_last_index +1;
        let last_new =peer_last_index;

        let new_blocks_range=first_new..=last_new;
        peer_blocks.get(new_blocks_range).unwrap().to_vec()

     }
     fn get_blocks_from_peer(&self,address:&str)->Vec<Block>{
        let url=format!("{}/blocks",address);
        let mut response=isahc::get(url).unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let raw_body=response.text().unwrap();
        serde_json::from_str(&raw_body).unwrap()
     }
     fn try_send_new_blocks(&self,last_send_block_index:usize){
        let new_blocks=self.get_new_blocks_since(last_send_block_index);

        for block in new_blocks.iter()  {
            for address in self.peer_addresser.iter() {
                let result=panic::catch_unwind(||{
                    Peer::send_block_to_peer(address,block);
                });

                if result.is_err(){
                    error!("Could not send block {} to peer {}",block.id,address);
                    return;
                }
                info!("Sended new block {} to peer {}",block.id,address);

            }
        }
     }
     fn get_new_blocks_since(&self,start_index:usize)->Vec<Block>{
        let last_block_index=self.get_last_block_index();
        let new_blocks_range=start_index+1..=last_block_index;

        self.blockchain.get_all_block()
                       .get(new_blocks_range)
                       .unwrap()
                       .to_vec()
     }
     fn send_block_to_peer(address:&str,block:&Block){
        let url=format!("{}/blocks",address);
        let body=serde_json::to_string(&block).unwrap();

        let request=Request::post(url)
                                     .header("Content_Type","application/json")
                                     .body(body)
                                     .unwrap();
        isahc::send(request).unwrap();
     }
}