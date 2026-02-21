use alloy::transports::http::Client;
use alloy_primitives::{Address, U256, address};
use intent_services::domain::quote::{PriceQuality, QuoteRequest, QuoteResponse};
use intent_services::run;

const OWNER: Address = address!("0xd8da6bf26964af9d7eed9e03e53415d37aa96045");
const SELL_TOKEN: Address = address!("0x749141F0F58ff5BBdf7cd8BAaa2fB2B7A4d38108");
const BUY_TOKEN: Address = address!("0xDDF7DCEFDc3edBcc0C13586DDF8F7B17aD881D3b");

#[tokio::test]
async fn get_quote_success() {
    let address = spawn_app().await;

    let quote = QuoteRequest {
        owner: OWNER,
        receiver: None,
        sell_token: SELL_TOKEN,
        buy_token: BUY_TOKEN,
        buy_amount: Some(U256::from(100)),
        slippage_bps: 0,
        valid_for_sec: 0,
        app_data: None,
        sell_amount: None,
        price_quality: PriceQuality::Fast,
    };

    let client = Client::new();

    let response = client
        .post(format!("{}/quote", address))
        .json(&quote)
        .send()
        .await
        .unwrap();

    let status = response.status();

    let result = response.json::<QuoteResponse>().await.unwrap();

    assert!(status.is_success());
    assert_eq!(result.status, "success")
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
