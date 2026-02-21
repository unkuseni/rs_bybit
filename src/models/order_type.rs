use crate::prelude::*;

/// Defines the type of order, either limit or market.
///
/// Order types determine how an order is executed in the market. A **Limit** order
/// is placed at a specific price and only executes at that price or better, offering
/// control but risking non-execution if the market moves away. A **Market** order
/// executes immediately at the best available price, ensuring execution but with
/// potential slippage (difference between expected and actual price). In perpetual
/// futures, where price volatility can be high, choosing the right order type is
/// crucial for bots. Market orders are faster but may incur higher fees (taker fees),
/// while limit orders may qualify for lower fees (maker fees) if they add liquidity.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub enum OrderType {
    /// A limit order, executed at a specified price or better.
    Limit,

    /// A market order, executed immediately at the best available price.
    #[default]
    Market,
}

impl OrderType {
    /// Converts the `OrderType` enum to its string representation.
    ///
    /// This method ensures the order type is formatted as "Limit" or "Market" for
    /// Bybit API requests. Bots must use this to comply with API expectations.
    pub fn as_str(&self) -> &str {
        match self {
            OrderType::Limit => "Limit",
            OrderType::Market => "Market",
        }
    }
}
