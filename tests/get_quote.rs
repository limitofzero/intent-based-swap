use alloy::providers::{Provider, ProviderBuilder};
use alloy::transports::http::Client;
use alloy_primitives::{Address, U256, address};
use intent_services::domain::quote::{PriceQuality, QuoteRequest, QuoteResponse};
use intent_services::run;

// Well-known mainnet tokens with guaranteed Uniswap V3 liquidity at fee tier 3000
const OWNER: Address = address!("d8da6bf26964af9d7eed9e03e53415d37aa96045");
const WETH: Address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
const USDC: Address = address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");

#[tokio::test]
async fn get_quote_success() {
    let base_url = spawn_app().await.unwrap();

    let quote = QuoteRequest {
        owner: OWNER,
        receiver: None,
        sell_token: WETH,
        buy_token: USDC,
        sell_amount: Some("1000000000000000000".to_string()), // 1 WETH
        buy_amount: None,
        slippage_bps: 50,
        valid_for_sec: 10, // minimum for PriceQuality::Fast
        app_data: None,
        price_quality: PriceQuality::Fast,
    };

    let client = Client::new();

    let response = client
        .post(format!("{}/quote", base_url))
        .json(&quote)
        .send()
        .await
        .unwrap();

    let status = response.status();
    let result = response.json::<QuoteResponse>().await.unwrap();

    assert!(status.is_success());
    assert!(!result.id.is_empty());
    assert!(result.verified);
    assert_eq!(result.intent_to_sign.sell_token, WETH);
    assert_eq!(result.intent_to_sign.buy_token, USDC);
    assert!(result.intent_to_sign.buy_amount.parse::<U256>().unwrap() > U256::ZERO);
}

async fn spawn_app() -> Option<String> {
    dotenvy::dotenv().ok();

    let rpc_url = match std::env::var("RPC_URL") {
        Ok(url) => url,
        Err(_) => return None,
    };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let provider = ProviderBuilder::new()
        .connect(&rpc_url)
        .await
        .unwrap()
        .erased();

    let server = run(listener, provider).await.expect("Failed to bind");
    tokio::spawn(async move { server.await.unwrap() });

    Some(format!("http://127.0.0.1:{}", port))
}
