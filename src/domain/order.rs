use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub  enum OrderType {
    Buy,
    Sell,
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            OrderType::Buy => "buy".to_string(),
            OrderType::Sell => "sell".to_string(),
        };
        write!(f, "{}", str)
    }
}
