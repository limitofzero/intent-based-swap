use intent_services::run;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();

    match run(listener).await {
        Err(e) => eprintln!("Failed to bind: {e}"),
        Ok(server) => {
            if let Err(e) = server.await {
                eprintln!("Server error: {e}");
            }
        }
    }
}
