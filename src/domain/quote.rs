use alloy::consensus::private::serde_json;
use alloy_primitives::{Address, U256};
use crate::domain::order::OrderType;

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, PartialEq)]
pub enum PriceQuality {
    Fast,
    Optimal,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct QuoteRequest {
    pub owner: Address,
    pub receiver: Option<Address>,
    pub sell_token: Address,
    pub buy_token: Address,
    pub sell_amount: Option<U256>,
    pub buy_amount: Option<U256>,
    pub slippage_bps: u16,
    pub valid_for_sec: u32,
    pub app_data: Option<serde_json::Value>,
    pub price_quality: PriceQuality,
}


#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct QuoteResponse {
    pub id: String,
    pub  expires_at: u32,
    pub  verified: bool,
    pub  intent_to_sign: IntentToSign,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct IntentToSign {
    pub order_type: OrderType,
    pub owner: Address,
    pub receiver: Address,
    pub sell_token: Address,
    pub buy_token: Address,
    pub sell_amount: U256,
    pub buy_amount: U256,
}

