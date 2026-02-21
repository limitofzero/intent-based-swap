use crate::domain::quote::QuoteRequest;
use thiserror::Error;

#[derive(Default)]
pub struct Quoter {}

#[derive(Error, Debug)]
pub enum QuoterError {
    #[error("invalid quote: {0}")]
    InvalidQuoteRequest(String),
}

impl Quoter {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn get_quote(&self, quote_request: &QuoteRequest) -> Result<(), QuoterError> {
        self.validate_quote_request(quote_request)?;

        Ok(())
    }

    fn validate_quote_request(&self, payload: &QuoteRequest) -> Result<(), QuoterError> {
        if payload.sell_amount.is_none() && payload.buy_amount.is_none() {
            return Err(QuoterError::InvalidQuoteRequest(
                "either buy_amount or sell_amount should be set".to_string(),
            ));
        }

        Ok(())
    }
}
