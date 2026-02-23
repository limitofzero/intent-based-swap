use crate::domain::order::OrderType;
use crate::domain::quote::{IntentToSign, QuoteRequest, QuoteResponse};
use crate::services::price_provider::PriceProvider;
use alloy_primitives::{Address, U256};
use std::sync::Arc;
use thiserror::Error;

pub struct Quoter {
    price_provider: Arc<PriceProvider>,
}

#[derive(Error, Debug)]
pub enum QuoterError {
    #[error("invalid quote request: {0}")]
    InvalidQuoteRequest(String),
    #[error("failed to get price")]
    FailedToGetPrice(String),
}

#[derive(Debug, Clone)]
struct ValidQuoteParams {
    sell_token: Address,
    buy_token: Address,
    amount: U256,
    is_sell: bool,
    receiver: Address,
    owner: Address,
    slippage_bps: u16,
    valid_for_sec: u32,
}

const FULL_BPS: u16 = 10_000;

impl Quoter {
    pub fn new(price_provider: Arc<PriceProvider>) -> Self {
        Self { price_provider }
    }
    pub async fn get_quote(
        &self,
        quote_request: QuoteRequest,
    ) -> Result<QuoteResponse, QuoterError> {
        let valid_quote_params = self.validate_and_extract_quote_params(quote_request)?;
        let ValidQuoteParams {
            sell_token,
            buy_token,
            amount,
            is_sell,
            receiver,
            owner,
            valid_for_sec,
            slippage_bps,
            ..
        } = valid_quote_params;

        let amount_out = self
            .price_provider
            .get_price(sell_token, buy_token, amount)
            .await
            .map_err(|err| QuoterError::FailedToGetPrice(err.to_string()))?;

        let min_received = if slippage_bps > 0 {
            amount_out * U256::from(FULL_BPS - slippage_bps) / U256::from(FULL_BPS)
        } else {
            amount_out
        };

        let order_type = if is_sell {
            OrderType::Sell
        } else {
            OrderType::Buy
        };

        let (sell_amount, buy_amount) = if is_sell {
            (amount, amount_out)
        } else {
            (amount_out, amount)
        };

        let intent_to_sign = IntentToSign {
            order_type,
            owner,
            receiver,
            sell_token,
            buy_token,
            sell_amount: sell_amount.to_string(),
            buy_amount: buy_amount.to_string(),
        };

        let quote = QuoteResponse {
            id: uuid::Uuid::new_v4().to_string(),
            expires_at: valid_for_sec,
            intent_to_sign,
            verified: true,
            min_received: min_received.to_string(),
        };

        Ok(quote)
    }

    fn validate_and_extract_quote_params(
        &self,
        payload: QuoteRequest,
    ) -> Result<ValidQuoteParams, QuoterError> {
        let owner = payload.owner;
        if owner == Address::ZERO {
            return Err(QuoterError::InvalidQuoteRequest(
                "owner should not be zero address".to_string(),
            ));
        }

        let (sell_token, buy_token, amount, is_sell) =
            if let Some(sell_amount) = payload.sell_amount {
                (payload.sell_token, payload.buy_token, sell_amount, true)
            } else if let Some(buy_amount) = payload.buy_amount {
                (payload.buy_token, payload.sell_token, buy_amount, false)
            } else {
                return Err(QuoterError::InvalidQuoteRequest(
                    "either buy_amount or sell_amount should be set".to_string(),
                ));
            };

        let amount = amount.parse::<U256>().map_err(|_| {
            QuoterError::InvalidQuoteRequest("amount should be a valid u256".to_string())
        })?;

        if amount.is_zero() {
            return Err(QuoterError::InvalidQuoteRequest(
                "amount should be greater than zero".to_string(),
            ));
        }

        if sell_token == buy_token {
            return Err(QuoterError::InvalidQuoteRequest(
                "sell_token and buy_token should be different".to_string(),
            ));
        }

        let receiver = payload.receiver.unwrap_or(owner);

        Ok(ValidQuoteParams {
            sell_token,
            buy_token,
            amount,
            is_sell,
            receiver,
            owner: payload.owner,
            slippage_bps: payload.slippage_bps,
            valid_for_sec: payload.valid_for_sec,
        })
    }
}
