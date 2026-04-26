use crate::prelude::*;

/// Represents the product category for Bybit instruments.
///
/// This enum defines the possible categories of trading instruments on Bybit, used in various API requests (e.g., Kline, Orderbook, OpenInterest). For perpetual futures, `Linear` (USDT-margined) and `Inverse` (coin-margined) are the most relevant categories.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Default)]
pub enum Category {
    /// Spot trading pairs (e.g., BTC/USDT spot market).
    ///
    /// Represents spot markets, where assets are traded for immediate delivery. Not relevant for perpetual futures but included for completeness. Bots trading perpetuals should avoid this category.
    #[serde(rename = "spot")]
    Spot,

    /// Linear perpetual futures (USDT-margined, e.g., BTCUSDT).
    ///
    /// Represents USDT-margined perpetual futures contracts, which use USDT as the margin and settlement currency. These are popular for their simplicity and stable margin value. Bots trading perpetuals typically use this category for pairs like `BTCUSDT`.
    #[serde(rename = "linear")]
    #[default]
    Linear,

    /// Inverse perpetual futures (coin-margined, e.g., BTCUSD).
    ///
    /// Represents coin-margined perpetual futures contracts, where the base asset (e.g., BTC) is used for margin and settlement. These are more complex due to the inverse relationship between contract value and price. Bots trading inverse perpetuals (e.g., `BTCUSD`) should use this category.
    #[serde(rename = "inverse")]
    Inverse,

    /// Options contracts.
    ///
    /// Represents options markets, which involve contracts giving the right to buy or sell an asset at a specific price. Not relevant for perpetual futures but included for completeness. Bots should avoid this category for perpetuals trading.
    #[serde(rename = "option")]
    Option,
}

impl Category {
    pub fn as_str(&self) -> &str {
        match self {
            Category::Spot => "spot",
            Category::Linear => "linear",
            Category::Inverse => "inverse",
            Category::Option => "option",
        }
    }

    /// Returns the public WebSocket API endpoint for this category.
    pub fn public_ws_endpoint(&self) -> WebsocketAPI {
        match self {
            Category::Linear => WebsocketAPI::PublicLinear,
            Category::Inverse => WebsocketAPI::PublicInverse,
            Category::Spot => WebsocketAPI::PublicSpot,
            Category::Option => WebsocketAPI::PublicOption,
        }
    }
}
