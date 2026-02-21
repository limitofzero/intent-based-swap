pub mod api;
pub mod domain;
pub mod handlers;
pub mod services;

use crate::api::routes::get_router;
use axum::Router;
use axum::serve::Serve;
use tokio::net::TcpListener;

pub struct AppState {
    quoter: services::quoter::Quoter,
}

pub async fn run(
    tcp_listener: TcpListener,
) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let quoter = services::quoter::Quoter::new();
    let app_state = AppState { quoter };

    let app = get_router(app_state);
    Ok(axum::serve(tcp_listener, app))
}
