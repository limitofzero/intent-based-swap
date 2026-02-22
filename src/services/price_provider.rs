use crate::contracts::quoter_v2;
use alloy::network::Ethereum;
use alloy::providers::DynProvider;
use alloy_primitives::aliases::U24;
use alloy_primitives::ruint::aliases::U160;
use alloy_primitives::{Address, U256, address};
use std::sync::Arc;
use thiserror::Error;

const QUOTER_V2_ADDRESS: Address = address!("0x61fFE014bA17989E743c5F6cB21bF9697530B21e");
#[derive(Debug, Error, Clone)]
pub enum PriceProviderError {
    #[error("failed to get price")]
    FailedToGetPrice(String),
}

pub struct PriceProvider {
    provider: Arc<DynProvider<Ethereum>>,
}

impl PriceProvider {
    pub fn new(provider: Arc<DynProvider<Ethereum>>) -> Self {
        Self {
            provider: Arc::clone(&provider),
        }
    }

    pub async fn get_price(
        &self,
        sell_token: Address,
        buy_token: Address,
        amount: U256,
    ) -> Result<U256, PriceProviderError> {
        let provider = Arc::clone(&self.provider);
        let quoter_v2 = quoter_v2::QuoterV2::new(QUOTER_V2_ADDRESS, provider);

        let params = quoter_v2::QuoterV2::QuoteExactInputSingleParams {
            tokenIn: sell_token,
            tokenOut: buy_token,
            amountIn: amount,
            fee: U24::from(3000),
            sqrtPriceLimitX96: U160::from(0),
        };

        let responce = quoter_v2
            .quoteExactInputSingle(params)
            .call()
            .await
            .map_err(|err| {
                eprintln!("Failed to get price: {}", err);
                PriceProviderError::FailedToGetPrice(err.to_string())
            })?;

        Ok(responce.amountOut)
    }
}
