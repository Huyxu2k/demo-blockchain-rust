use  axum::{http::{StatusCode},response::{IntoResponse,Response},Json, extract::State};
use  crate::core::transaction::Transaction;
use  super::api::ApiState;

//TODO NO complete

///Return list of all blocks
/// 
/// GET
pub async fn get_blocks(State(state):State<ApiState>)-> impl IntoResponse{
  let blockchain = &state.blockchain;
  let blocks = blockchain.get_all_block();

  IntoResponse::into_response(Json::from(&blocks))
}

// ///Return balance of address
// /// 
// /// GET
// pub async fn get_balance(State(state):State<ApiState>,address:String)->impl IntoResponse{
//   todo!()
// }

// ///Call transaction 
// /// 
// ///CALL
// pub async fn call_transaction(State(state):State<ApiState>,transaction:Transaction)->impl IntoResponse{
//   todo!()
// }

