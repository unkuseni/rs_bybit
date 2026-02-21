use crate::prelude::*;

/// Structure for WebSocket RPI (Real-time Price Improvement) order book update events.
///
/// Contains real-time updates to the RPI order book for a trading pair, including bids, asks,
/// RPI sizes, and sequence numbers. RPI orders can provide price improvement for takers.
/// Bots use this for market depth analysis with RPI information and liquidity monitoring.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RPIOrderbookUpdate {
    /// The WebSocket topic for the event (e.g., "orderbook.rpi.BTCUSDT").
    ///
    /// Specifies the data stream for the RPI order book update, including depth and symbol.
    /// Bots use this to verify the correct market and RPI depth level.
    #[serde(rename = "topic")]
    pub topic: String,

    /// The event type (e.g., "snapshot", "delta").
    ///
    /// Indicates whether the update is a full snapshot or incremental delta.
    /// Bots use this to initialize or update their RPI order book state.
    #[serde(rename = "type")]
    pub event_type: String,

    /// The timestamp of the event in milliseconds.
    ///
    /// Indicates when the RPI order book update was generated.
    /// Bots use this to ensure data freshness and align with other market data.
    #[serde(rename = "ts")]
    pub timestamp: u64,

    /// The RPI order book data.
    ///
    /// Contains the bids, asks with RPI information, and sequence numbers for the order book.
    /// Bots use this to update their internal RPI order book representation.
    pub data: RPIOrderbook,

    /// The creation timestamp in milliseconds.
    ///
    /// Indicates when the RPI order book update was created by Bybit’s matching engine.
    /// Bots use this to measure latency and ensure data consistency.
    pub cts: u64,
}

impl RPIOrderbookUpdate {
    /// Returns the symbol from the topic.
    ///
    /// Extracts the trading pair symbol from the WebSocket topic.
    /// Example: "orderbook.rpi.BTCUSDT" -> "BTCUSDT"
    pub fn symbol_from_topic(&self) -> Option<&str> {
        self.topic.split('.').last()
    }

    /// Returns true if this is a snapshot update.
    ///
    /// Snapshot updates contain the full order book state and should replace the local order book.
    pub fn is_snapshot(&self) -> bool {
        self.event_type == "snapshot"
    }

    /// Returns true if this is a delta update.
    ///
    /// Delta updates contain incremental changes and should be applied to the local order book.
    pub fn is_delta(&self) -> bool {
        self.event_type == "delta"
    }

    /// Returns the processing latency in milliseconds.
    ///
    /// Calculates the difference between the matching engine timestamp (cts) and
    /// the system generation timestamp (ts).
    pub fn processing_latency_ms(&self) -> i64 {
        if self.cts > self.timestamp {
            (self.cts - self.timestamp) as i64
        } else {
            (self.timestamp - self.cts) as i64
        }
    }

    /// Returns the timestamp as a chrono DateTime.
    pub fn timestamp_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp((self.timestamp / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now)
    }

    /// Returns the creation timestamp as a chrono DateTime.
    pub fn creation_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp((self.cts / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now)
    }

    /// Returns the best ask price (lowest ask).
    pub fn best_ask(&self) -> Option<f64> {
        self.data.best_ask()
    }

    /// Returns the best bid price (highest bid).
    pub fn best_bid(&self) -> Option<f64> {
        self.data.best_bid()
    }

    /// Returns the best ask with RPI information.
    pub fn best_ask_with_rpi(&self) -> Option<&RPIOrderbookLevel> {
        self.data.best_ask_with_rpi()
    }

    /// Returns the best bid with RPI information.
    pub fn best_bid_with_rpi(&self) -> Option<&RPIOrderbookLevel> {
        self.data.best_bid_with_rpi()
    }

    /// Returns the bid-ask spread.
    pub fn spread(&self) -> Option<f64> {
        self.data.spread()
    }

    /// Returns the mid price (average of best bid and ask).
    pub fn mid_price(&self) -> Option<f64> {
        self.data.mid_price()
    }

    /// Returns the spread as a percentage of mid price.
    pub fn spread_percentage(&self) -> Option<f64> {
        self.data.spread_percentage()
    }

    /// Returns the total RPI size on the ask side.
    pub fn total_ask_rpi_size(&self) -> f64 {
        self.data.total_ask_rpi_size()
    }

    /// Returns the total non-RPI size on the ask side.
    pub fn total_ask_non_rpi_size(&self) -> f64 {
        self.data.total_ask_non_rpi_size()
    }

    /// Returns the total RPI size on the bid side.
    pub fn total_bid_rpi_size(&self) -> f64 {
        self.data.total_bid_rpi_size()
    }

    /// Returns the total non-RPI size on the bid side.
    pub fn total_bid_non_rpi_size(&self) -> f64 {
        self.data.total_bid_non_rpi_size()
    }

    /// Returns the average RPI ratio on the ask side.
    pub fn average_ask_rpi_ratio(&self) -> f64 {
        self.data.average_ask_rpi_ratio()
    }

    /// Returns the average RPI ratio on the bid side.
    pub fn average_bid_rpi_ratio(&self) -> f64 {
        self.data.average_bid_rpi_ratio()
    }

    /// Returns the bid-ask RPI ratio difference.
    pub fn rpi_ratio_imbalance(&self) -> f64 {
        self.data.rpi_ratio_imbalance()
    }

    /// Returns the order book imbalance considering RPI sizes.
    pub fn order_book_imbalance_with_rpi(&self) -> f64 {
        self.data.order_book_imbalance_with_rpi()
    }

    /// Returns the liquidity score considering RPI availability.
    pub fn liquidity_score_with_rpi(&self) -> f64 {
        self.data.liquidity_score_with_rpi()
    }

    /// Returns the expected price improvement for takers.
    pub fn expected_taker_improvement(&self, is_buy: bool, quantity: f64) -> Option<f64> {
        self.data.expected_taker_improvement(is_buy, quantity)
    }

    /// Returns the price impact for a given quantity considering RPI.
    pub fn ask_price_impact_with_rpi(&self, quantity: f64) -> Option<f64> {
        self.data.ask_price_impact_with_rpi(quantity)
    }

    /// Returns the price impact for a given quantity considering RPI.
    pub fn bid_price_impact_with_rpi(&self, quantity: f64) -> Option<f64> {
        self.data.bid_price_impact_with_rpi(quantity)
    }

    /// Returns the weighted average ask price considering RPI improvement.
    pub fn weighted_average_ask_price_with_rpi(&self, target_quantity: f64) -> Option<f64> {
        self.data
            .weighted_average_ask_price_with_rpi(target_quantity)
    }

    /// Returns the weighted average bid price considering RPI improvement.
    pub fn weighted_average_bid_price_with_rpi(&self, target_quantity: f64) -> Option<f64> {
        self.data
            .weighted_average_bid_price_with_rpi(target_quantity)
    }
}
