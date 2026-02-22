pub mod api;
pub mod domain;
pub mod handlers;
pub mod services;
pub mod contracts;
pub  mod config;

use std::sync::Arc;
use alloy::network::Ethereum;
use alloy::providers::{DynProvider};
use crate::api::routes::get_router;
use axum::Router;
use axum::serve::Serve;
use tokio::net::TcpListener;
use crate::services::price_provider::PriceProvider;

pub struct AppState {
    quoter: Arc<services::quoter::Quoter>,
}

pub async fn run(
    tcp_listener: TcpListener,
    rpc_provider: DynProvider<Ethereum>,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let rpc_provider = Arc::new(rpc_provider);

    let price_provider = PriceProvider::new(Arc::clone(&rpc_provider));
    let quoter = services::quoter::Quoter::new(Arc::new(price_provider));

    let app_state = AppState {
        quoter: Arc::new(quoter),
    };

    let app = get_router(app_state);
    Ok(axum::serve(tcp_listener, app))
}
