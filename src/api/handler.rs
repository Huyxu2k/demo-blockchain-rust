use super::api::ApiState;
use crate::core::{block::Block, transaction::{Transaction, TransactionData}, address::{Address, self},account::AccountInfo};
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
    let mut transaction=block.transactions;
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
///Return tokens of address
///
/// GET
pub async fn get_tokens_by_address(State(state): State<ApiState>, Json(address):Json<Address>) -> impl IntoResponse {
    let mut accounts=state.blockchain.accounts.lock().unwrap();
    
    let account=accounts.get_account_tokens(&address);
    match account {
        Ok(acc)=>{
             IntoResponse::into_response((StatusCode::OK,Json::from(acc)))
        },
        Err(err)=>{
            let rerult=format!("Not found address: {}",address.to_string());
            IntoResponse::into_response((StatusCode::BAD_REQUEST,Json(rerult)))
        }
    }
}
// ///get all account 
// /// 
// /// 
// pub async fn get_accounts(State(state): State<ApiState>)->impl IntoResponse{
//     let mut blockchain_clone=state.blockchain.clone();

//     let accounts=blockchain_clone.get_all
//     info!("{:#?}",accounts.clone());
//     IntoResponse::into_response((StatusCode::OK,Json(accounts.clone())))
// }

///Create account
/// 
pub async fn create_account(State(state): State<ApiState>,Json(user):Json<String>)->impl IntoResponse{
    let transaction=Transaction::new(1000,TransactionData::CreateAccount { user });
    let result=&state.blockchain.create_account(transaction.clone().to_vec()).unwrap();

    //add transaction
    &state.pool.add(transaction.clone());

    IntoResponse::into_response((StatusCode::OK,Json(format!("Your address is {}",transaction.sender.to_string()))))
}

///Add tokens
pub async fn create_tokens_account(State(state): State<ApiState>,Json(info): Json<Vec<AccountInfo>>)->impl IntoResponse{
     let  vec_info=info.to_vec();
     for i in vec_info  {
        &state.blockchain.accounts.lock().unwrap().create_account_tokens(i.address, i.tokens);
     }
     IntoResponse::into_response((StatusCode::OK,Json(json!("Create tokens success"))))
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
