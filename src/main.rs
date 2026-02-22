use alloy::providers::{Provider, ProviderBuilder};
use intent_services::run;
use intent_services::config::args::Args;

#[tokio::main]
async fn main() {
    let config = Args::from_env();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    let provider = ProviderBuilder::new()
        .connect(config.rpc_url.as_str())
        .await
        .unwrap()
        .erased();

    match run(listener, provider).await {
        Err(e) => eprintln!("Failed to bind: {e}"),
        Ok(server) => {
            if let Err(e) = server.await {
                eprintln!("Server error: {e}");
            }
        }
    }
}
