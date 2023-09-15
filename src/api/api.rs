use std::net::SocketAddr;

use crate::{
    core::{
        block::Block, blockchain::Blockchain, transaction::Transaction,
        transaction_pool::TransactionPool,
    },
    utils::{context::Context, execution::Runnable},
};
use anyhow::Result;
use log::info;
use super::handler::{get_blocks,add_block,add_balances, get_balances, get_balance_by_address, get_transactions};
use axum::{
    http::{HeaderValue, Method, header::{AUTHORIZATION, ACCEPT}}, 
    routing::{get, post}, Router, extract::State,
};
use tower_http::cors::CorsLayer;

#[derive(Debug,Clone)]
pub struct ApiState {
    pub blockchain: Blockchain,
    pub pool: TransactionPool,
}
pub struct Api {
    port: u16,
    blockchain: Blockchain,
    pool: TransactionPool,
}
impl Runnable for Api {
    fn run(&self) -> Result<()> {
        let api = self.blockchain.clone();
        let pool = self.pool.clone();

        start_server(self.port, api, pool)
    }
}
impl Api {
    pub fn new(context: &Context) -> Api {
        Api {
            port: context.config.port,
            blockchain: context.blockchain.clone(),
            pool: context.pool.clone(),
        }
    }
}

#[tokio::main]
async fn start_server(port: u16, blockchain: Blockchain, pool: TransactionPool) -> Result<()> {
    let url = format!("127.0.0.1:{}", port);
    // let addr=SocketAddr::from([127,0,0,1],8000);
    let state=ApiState { blockchain, pool };

    // let cors = CorsLayer::new()
    //     .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    //     .allow_methods([Method::GET, Method::POST])
    //     .allow_credentials(true)
    //     .allow_headers([AUTHORIZATION, ACCEPT]);
    let app = Router::new()
                             .nest("/api",App())
                             .with_state(state);
    info!("🚀 Server started successfully: {}",url.clone());
    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}


pub fn App()->Router<ApiState>{
    Router::new()
       .route("/blocks",get(get_blocks))
       .route("/add_block",post(add_block))
       .route("/add_account_balance",post(add_balances))
       .route("/get_balances", get(get_balances))
       .route("/get_balance_by_address", get(get_balance_by_address))
       .route("/get_transactions", get(get_transactions))
}
