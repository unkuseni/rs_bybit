use crate::prelude::*;

/// Represents the side of an order in trading, either buying or selling.
///
/// In financial markets, the "side" of an order indicates whether the trader is
/// purchasing (Buy) or selling (Sell) an asset. For perpetual futures on Bybit,
/// this determines whether the trader is opening a long position (Buy) or a short
/// position (Sell). When writing trading bots, the side is critical for position
/// management, as it affects margin requirements and exposure to price movements.
/// For example, a Buy order in a rising market aims to profit from price increases,
/// while a Sell order in a falling market aims to profit from price declines.
/// Incorrectly setting the side can lead to unintended positions, so bots must
/// validate this field against their strategy.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub enum Side {
    /// Represents a buy order, initiating a long position or closing a short position.
    #[default]
    Buy,

    /// Represents a sell order, initiating a short position or closing a long position.
    Sell,
}

impl Side {
    /// Converts the `Side` enum to its string representation.
    ///
    /// This method is used to serialize the `Side` enum into a string format
    /// compatible with Bybit's API, which expects "Buy" or "Sell". Bots should use
    /// this method when constructing API requests to ensure correct formatting.
    pub fn as_str(&self) -> &str {
        match self {
            Side::Buy => "Buy",
            Side::Sell => "Sell",
        }
    }
}
