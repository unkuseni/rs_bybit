use crate::prelude::*;

/// Structure for individual trade data in WebSocket trade updates.
///
/// Contains details of a single executed trade, such as price, volume, and side. Bots use this to monitor market activity and inform trading decisions.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsTrade {
    /// The timestamp of the trade in milliseconds.
    ///
    /// Indicates when the trade was executed. Bots use this to align trade data with other time-series data.
    #[serde(rename = "T")]
    pub timestamp: u64,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for the trade. Bots use this to verify the correct market.
    #[serde(rename = "s")]
    pub symbol: String,

    /// The trade side ("Buy" or "Sell").
    ///
    /// Indicates whether the trade was initiated by a buyer or seller. Bots use this to assess market direction and momentum.
    #[serde(rename = "S")]
    pub side: String,

    /// The trade volume.
    ///
    /// The quantity of the base asset traded. Bots use this to gauge trade size and market liquidity.
    #[serde(rename = "v", with = "string_to_float")]
    pub volume: f64,

    /// The trade price.
    ///
    /// The price at which the trade was executed. Bots use this for price discovery and technical analysis.
    #[serde(rename = "p", with = "string_to_float")]
    pub price: f64,

    /// The tick direction of the trade.
    ///
    /// Indicates whether the trade was an uptick, downtick, or neutral (e.g., "PlusTick", "MinusTick"). Bots use this to analyze short-term price momentum.
    #[serde(rename = "L")]
    pub tick_direction: TickDirection,

    /// The unique trade ID.
    ///
    /// A unique identifier for the trade execution. Bots use this to track specific trades and avoid duplicates.
    #[serde(rename = "i")]
    pub id: String,

    /// Whether the buyer was the maker.
    ///
    /// If `true`, the buyer’s order was on the order book (maker); if `false`, the buyer took liquidity (taker). Bots use this to analyze market dynamics and order flow.
    #[serde(rename = "BT")]
    pub buyer_is_maker: bool,
}
