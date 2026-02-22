use crate::domain::order::OrderType;
use crate::domain::quote::{IntentToSign, PriceQuality, QuoteRequest, QuoteResponse};
use crate::services::price_provider::PriceProvider;
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

impl Quoter {
    pub fn new(price_provider: Arc<PriceProvider>) -> Self {
        Self { price_provider }
    }
    pub async fn get_quote(
        &self,
        quote_request: &QuoteRequest,
    ) -> Result<QuoteResponse, QuoterError> {
        self.validate_quote_request(quote_request)?;

        let (sell_token, buy_token, amount, is_sell) = if quote_request.sell_amount.is_some() {
            (
                quote_request.sell_token,
                quote_request.buy_token,
                quote_request.sell_amount.unwrap(),
                true,
            )
        } else {
            (
                quote_request.buy_token,
                quote_request.sell_token,
                quote_request.buy_amount.unwrap(),
                false,
            )
        };

        let amount_out = self
            .price_provider
            .get_price(sell_token, buy_token, amount)
            .await
            .map_err(|err| QuoterError::FailedToGetPrice(err.to_string()))?;

        let order_type = if is_sell {
            OrderType::Sell
        } else {
            OrderType::Buy
        };
        let receiver = quote_request.receiver.unwrap_or(quote_request.owner);

        let (sell_amount, buy_amount) = if is_sell {
            (amount, amount_out)
        } else {
            (amount_out, amount)
        };

        let intent_to_sign = IntentToSign {
            order_type,
            owner: quote_request.owner,
            receiver,
            sell_token,
            buy_token,
            sell_amount,
            buy_amount,
        };

        let quote = QuoteResponse {
            id: uuid::Uuid::new_v4().to_string(),
            expires_at: quote_request.valid_for_sec,
            intent_to_sign,
            verified: true,
        };

        Ok(quote)
    }

    fn validate_quote_request(&self, payload: &QuoteRequest) -> Result<(), QuoterError> {
        if payload.sell_amount.is_none() && payload.buy_amount.is_none() {
            return Err(QuoterError::InvalidQuoteRequest(
                "either buy_amount or sell_amount should be set".to_string(),
            ));
        }

        if payload.sell_token == payload.buy_token {
            return Err(QuoterError::InvalidQuoteRequest(
                "sell_token and buy_token should be different".to_string(),
            ));
        }

        if payload.price_quality == PriceQuality::Fast && payload.valid_for_sec < 10 {
            return Err(QuoterError::InvalidQuoteRequest(
                "valid_for_sec should be at least 10".to_string(),
            ));
        }

        if payload.price_quality == PriceQuality::Optimal && payload.valid_for_sec < 30 {
            return Err(QuoterError::InvalidQuoteRequest(
                "valid_for_sec should be at least 30".to_string(),
            ));
        }

        if payload.sell_amount.is_some() && payload.sell_amount.unwrap().is_zero() {
            return Err(QuoterError::InvalidQuoteRequest(
                "sell_amount should be greater than zero".to_string(),
            ));
        } else if payload.buy_amount.is_some() && payload.buy_amount.unwrap().is_zero() {
            return Err(QuoterError::InvalidQuoteRequest(
                "buy_amount should be greater than zero".to_string(),
            ));
        }

        Ok(())
    }
}
