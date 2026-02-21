use axum::Router;
use axum::routing::get;
use axum::serve::Serve;
use tokio::net::TcpListener;

pub  async fn run(tcp_listener: TcpListener) -> Result<Serve<TcpListener, Router, Router>, std::io::Error> {
    let app = Router::new().route("/health_check", get(|| async { "is live" } ));
    Ok(axum::serve(tcp_listener, app))
}