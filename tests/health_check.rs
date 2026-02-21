use alloy::transports::http::Client;
use intent_services::run;

#[tokio::test]
async fn health_check() {
    let address = spawn_app().await;

    let client = Client::new();

    let response = client.get(format!("{}/health_check", address))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

}

async fn spawn_app() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).await.expect("Failed to bind");
    let _ = tokio::spawn(async move {
        server.await.unwrap();
    });

    format!("http://127.0.0.1:{}", port)
}