use super::api::ApiState;
use crate::core::{block::Block, transaction::{Transaction, self}, blockchain::{BLOCK_SUBSIDY, self}, address::{Address, self, AddressDTO}, account_balance_map::{AccountBalanceMap, Balance}};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;


///Return list of all blocks
///
/// GET
pub async fn get_blocks(State(state): State<ApiState>) -> impl IntoResponse {
    let blockchain = &state.blockchain;
    let blocks = blockchain.get_all_block();
    
    IntoResponse::into_response(Json::from(&blocks))
}
///Add a new block to blockchain
///
/// POST
pub async fn add_block(
    State(state): State<ApiState>,
    Json(block_data): Json<Block>,
) -> impl IntoResponse {
    let last_block=&state.blockchain.get_last_block();
    let mut block = block_data;

    block.hash = block.calculate_hash();
    block.previous_hash=last_block.hash;

    let blockchain = &state.blockchain;

    //add block
    let result = blockchain.add_block(block.clone());

    //add transaction 
    let mut transaction=block.transaction;
    let state_clone=state.clone().pool.add_vec(transaction);


    match result {
        Ok(_) => {
            info!("Received new block {}", block.id);
            IntoResponse::into_response((StatusCode::OK,Json(json!("add block success"))))
        }
        Err(err) => {
            let json_data = json!({"error" : err.to_string()});
            IntoResponse::into_response((StatusCode::BAD_REQUEST, Json(json_data)))
        }
    }


}
///Return balance of address
///
/// GET
pub async fn get_balance_by_address(State(state): State<ApiState>, Json(address):Json<AddressDTO>) -> impl IntoResponse {
    let mut account_balances=state.blockchain.account_balances.lock().unwrap();
    
    let account=account_balances.get_balance(address.address.clone());
    match account {
        Some(account)=>{
             IntoResponse::into_response((StatusCode::OK,Json::from(account)))
        },
        None=>{
            let rerult=format!("Not found address: {}",address.address);
            IntoResponse::into_response((StatusCode::BAD_REQUEST,Json(rerult)))
        }
    }
}
///get all account balances
/// 
/// 
pub async fn get_balances(State(state): State<ApiState>)->impl IntoResponse{
    let mut account_balances=state.blockchain.account_balances.lock().unwrap();

    let accounts=account_balances.get_account_balances();
    info!("{:#?}",accounts.clone());
    IntoResponse::into_response((StatusCode::OK,Json(accounts.clone())))
}
///Add amount 
pub async fn add_balances(State(state): State<ApiState>,Json(balances): Json<Vec<Balance>>)->impl IntoResponse{
     let mut vec_account=balances.to_vec();
     let account_balances=&state.blockchain.account_balances.lock().unwrap().add_account(vec_account);  

     IntoResponse::into_response((StatusCode::OK,Json(json!("Add balances success"))))
}
///Call transaction
///
///CALL
pub async fn get_transactions(
    State(state): State<ApiState>
) -> impl IntoResponse {
   let transactions=state.pool.get_transactions();


   IntoResponse::into_response((StatusCode::OK,Json::from(&transactions)))

}
///Call transaction
/// 
/// 
pub async fn call_transaction(State(state):State<ApiState>,Json(transaction_json):Json<Transaction>)->impl IntoResponse{
  let transaction=transaction_json;
  let pool=&state.pool;
  pool.add(transaction);

  IntoResponse::into_response(StatusCode::OK)
}
