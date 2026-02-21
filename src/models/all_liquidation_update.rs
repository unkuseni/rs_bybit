use crate::prelude::*;

/// Represents a single liquidation entry in the "all liquidation" WebSocket stream.
///
/// Contains details of a liquidation event that occurred on Bybit across all contract types.
/// Liquidations happen when a trader's position cannot meet margin requirements, leading to
/// forced closure. This struct provides information about the size, price, and direction
/// of liquidated positions.
///
/// # Bybit API Reference
/// The Bybit WebSocket API (https://bybit-exchange.github.io/docs/v5/websocket/public/all-liquidation)
/// provides all liquidation data with a push frequency of 500ms.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AllLiquidationData {
    /// The timestamp when the liquidation was updated (in milliseconds).
    ///
    /// Indicates the exact time when the liquidation event occurred.
    /// Bots can use this to correlate liquidation events with price movements.
    #[serde(rename = "T")]
    #[serde(with = "string_to_u64")]
    pub updated_time: u64,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Specifies the market where the liquidation occurred.
    /// Bots can filter by symbol to focus on relevant markets.
    #[serde(rename = "s")]
    pub symbol: String,

    /// The side of the liquidated position ("Buy" or "Sell").
    ///
    /// Indicates whether the liquidated position was long (Buy) or short (Sell).
    /// When you receive a "Buy" update, this means that a long position has been liquidated.
    /// A high volume of liquidations on one side can signal a potential price reversal.
    #[serde(rename = "S")]
    pub side: String,

    /// The executed size of the liquidated position.
    ///
    /// Represents the volume of the position that was forcibly closed.
    /// Large liquidations can cause significant price movements and increased volatility.
    #[serde(rename = "v")]
    #[serde(with = "string_to_float")]
    pub size: f64,

    /// The price at which the position was liquidated.
    ///
    /// This is the bankruptcy price at which the position was forcibly closed.
    /// Liquidation price levels often act as support or resistance zones.
    #[serde(rename = "p")]
    #[serde(with = "string_to_float")]
    pub price: f64,
}

impl AllLiquidationData {
    /// Constructs a new AllLiquidationData with specified parameters.
    pub fn new(symbol: &str, side: &str, size: f64, price: f64, updated_time: u64) -> Self {
        Self {
            symbol: symbol.to_string(),
            side: side.to_string(),
            size,
            price,
            updated_time,
        }
    }

    /// Returns true if the liquidated position was a long position.
    ///
    /// Long positions are liquidated when prices fall below the liquidation price.
    pub fn is_long(&self) -> bool {
        self.side.eq_ignore_ascii_case("Buy")
    }

    /// Returns true if the liquidated position was a short position.
    ///
    /// Short positions are liquidated when prices rise above the liquidation price.
    pub fn is_short(&self) -> bool {
        self.side.eq_ignore_ascii_case("Sell")
    }

    /// Returns the notional value of the liquidation.
    ///
    /// Calculated as `size * price`. This represents the total value of the position
    /// that was liquidated, useful for assessing the market impact.
    pub fn notional_value(&self) -> f64 {
        self.size * self.price
    }

    /// Returns the updated time as a chrono DateTime.
    pub fn updated_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp((self.updated_time / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now)
    }

    /// Returns the age of the liquidation in milliseconds.
    ///
    /// Calculates how long ago this liquidation occurred relative to current time.
    pub fn age_ms(&self) -> u64 {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        if now > self.updated_time {
            now - self.updated_time
        } else {
            0
        }
    }

    /// Returns true if the liquidation is recent (≤ 1 second old).
    ///
    /// Recent liquidations are more relevant for real-time trading decisions.
    pub fn is_recent(&self) -> bool {
        self.age_ms() <= 1000
    }

    /// Returns a string representation of the liquidation.
    pub fn to_display_string(&self) -> String {
        format!(
            "{} {} {}: {:.8} @ {:.8} (Value: {:.2})",
            self.symbol,
            self.side,
            if self.is_long() { "LONG" } else { "SHORT" },
            self.size,
            self.price,
            self.notional_value()
        )
    }

    /// Returns the liquidation as a tuple for easy pattern matching.
    pub fn as_tuple(&self) -> (&str, &str, f64, f64, u64) {
        (
            &self.symbol,
            &self.side,
            self.size,
            self.price,
            self.updated_time,
        )
    }

    /// Returns true if this liquidation is for a specific symbol.
    pub fn is_symbol(&self, symbol: &str) -> bool {
        self.symbol.eq_ignore_ascii_case(symbol)
    }

    /// Returns the side as an enum-like value.
    pub fn side_enum(&self) -> LiquidationSide {
        if self.is_long() {
            LiquidationSide::Long
        } else {
            LiquidationSide::Short
        }
    }

    /// Returns the price impact assuming linear market impact model.
    ///
    /// This is a simplified model: impact = k * sqrt(notional_value)
    /// where k is an impact coefficient (default 0.001).
    pub fn estimated_price_impact(&self, impact_coefficient: f64) -> f64 {
        impact_coefficient * self.notional_value().sqrt()
    }

    /// Returns the percentage price impact relative to the liquidation price.
    pub fn estimated_price_impact_percentage(&self, impact_coefficient: f64) -> f64 {
        if self.price != 0.0 {
            self.estimated_price_impact(impact_coefficient) / self.price * 100.0
        } else {
            0.0
        }
    }
}

/// Simple enum representation of liquidation side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiquidationSide {
    Long,
    Short,
}

impl LiquidationSide {
    /// Returns the string representation as used in Bybit API.
    pub fn as_str(&self) -> &'static str {
        match self {
            LiquidationSide::Long => "Buy",
            LiquidationSide::Short => "Sell",
        }
    }

    /// Returns the opposite side.
    pub fn opposite(&self) -> Self {
        match self {
            LiquidationSide::Long => LiquidationSide::Short,
            LiquidationSide::Short => LiquidationSide::Long,
        }
    }
}

/// Represents a WebSocket "all liquidation" update event.
///
/// Contains real-time liquidation events that occur across all Bybit markets.
/// This stream pushes all liquidations that occur on Bybit, covering:
/// - USDT contracts (Perpetual and Delivery)
/// - USDC contracts (Perpetual and Delivery)
/// - Inverse contracts
///
/// Push frequency: 500ms
///
/// # Bybit API Reference
/// Topic: `allLiquidation.{symbol}` (e.g., `allLiquidation.BTCUSDT`)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AllLiquidationUpdate {
    /// The WebSocket topic for the event (e.g., "allLiquidation.BTCUSDT").
    ///
    /// Specifies the data stream for the liquidation update.
    /// Bots use this to determine which symbol the update belongs to.
    #[serde(rename = "topic")]
    pub topic: String,

    /// The event type (e.g., "snapshot").
    ///
    /// All liquidation updates are snapshot type, containing the latest liquidation events.
    #[serde(rename = "type")]
    pub event_type: String,

    /// The timestamp of the event in milliseconds.
    ///
    /// Indicates when the liquidation update was generated by the system.
    /// Bots use this to ensure data freshness and time-based analysis.
    #[serde(rename = "ts")]
    #[serde(with = "string_to_u64")]
    pub timestamp: u64,

    /// The liquidation data.
    ///
    /// Contains a list of liquidation entries. Each entry represents a single
    /// liquidation event that occurred on Bybit.
    #[serde(rename = "data")]
    pub data: Vec<AllLiquidationData>,
}

impl AllLiquidationUpdate {
    /// Extracts the symbol from the topic.
    ///
    /// Parses the WebSocket topic to extract the trading symbol.
    /// Example: "allLiquidation.BTCUSDT" -> "BTCUSDT"
    pub fn symbol_from_topic(&self) -> Option<&str> {
        self.topic.split('.').last()
    }

    /// Returns true if this is a snapshot update.
    ///
    /// All liquidation updates are snapshot type.
    pub fn is_snapshot(&self) -> bool {
        self.event_type == "snapshot"
    }

    /// Returns the timestamp as a chrono DateTime.
    pub fn timestamp_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp((self.timestamp / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now)
    }

    /// Returns the age of the update in milliseconds.
    ///
    /// Calculates how old this update is relative to the current time.
    pub fn age_ms(&self) -> u64 {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        if now > self.timestamp {
            now - self.timestamp
        } else {
            0
        }
    }

    /// Returns true if the update is stale (older than 1 second).
    ///
    /// Since liquidation updates are pushed every 500ms, data older than 1 second
    /// might be considered stale for real-time trading decisions.
    pub fn is_stale(&self) -> bool {
        self.age_ms() > 1000
    }

    /// Returns the number of liquidation entries in this update.
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Returns true if there are no liquidation entries in this update.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the total notional value of all liquidations in this update.
    ///
    /// Sums the notional values of all liquidation entries.
    /// Useful for assessing the overall market impact of liquidations.
    pub fn total_notional_value(&self) -> f64 {
        self.data.iter().map(|liq| liq.notional_value()).sum()
    }

    /// Returns the total size of long liquidations.
    pub fn total_long_size(&self) -> f64 {
        self.data
            .iter()
            .filter(|liq| liq.is_long())
            .map(|liq| liq.size)
            .sum()
    }

    /// Returns the total size of short liquidations.
    pub fn total_short_size(&self) -> f64 {
        self.data
            .iter()
            .filter(|liq| liq.is_short())
            .map(|liq| liq.size)
            .sum()
    }

    /// Returns the total notional value of long liquidations.
    pub fn total_long_notional(&self) -> f64 {
        self.data
            .iter()
            .filter(|liq| liq.is_long())
            .map(|liq| liq.notional_value())
            .sum()
    }

    /// Returns the total notional value of short liquidations.
    pub fn total_short_notional(&self) -> f64 {
        self.data
            .iter()
            .filter(|liq| liq.is_short())
            .map(|liq| liq.notional_value())
            .sum()
    }

    /// Returns the net liquidation imbalance.
    ///
    /// Calculated as (total_long_notional - total_short_notional).
    /// A positive value indicates more long liquidations (bearish pressure).
    /// A negative value indicates more short liquidations (bullish pressure).
    pub fn net_imbalance(&self) -> f64 {
        self.total_long_notional() - self.total_short_notional()
    }

    /// Returns the net liquidation imbalance as a percentage of total notional.
    pub fn net_imbalance_percentage(&self) -> f64 {
        let total = self.total_notional_value();
        if total != 0.0 {
            self.net_imbalance() / total * 100.0
        } else {
            0.0
        }
    }

    /// Returns the average price of all liquidations.
    pub fn average_price(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            let total_notional = self.total_notional_value();
            let total_size = self.data.iter().map(|liq| liq.size).sum::<f64>();
            if total_size != 0.0 {
                Some(total_notional / total_size)
            } else {
                None
            }
        }
    }

    /// Returns the weighted average price (by size) of liquidations.
    pub fn weighted_average_price(&self) -> Option<f64> {
        self.average_price()
    }

    /// Returns the maximum liquidation size in this update.
    pub fn max_size(&self) -> Option<f64> {
        self.data.iter().map(|liq| liq.size).reduce(f64::max)
    }

    /// Returns the minimum liquidation size in this update.
    pub fn min_size(&self) -> Option<f64> {
        self.data.iter().map(|liq| liq.size).reduce(f64::min)
    }

    /// Returns the maximum liquidation price in this update.
    pub fn max_price(&self) -> Option<f64> {
        self.data.iter().map(|liq| liq.price).reduce(f64::max)
    }

    /// Returns the minimum liquidation price in this update.
    pub fn min_price(&self) -> Option<f64> {
        self.data.iter().map(|liq| liq.price).reduce(f64::min)
    }

    /// Returns all liquidation entries for a specific side.
    pub fn filter_by_side(&self, side: &str) -> Vec<&AllLiquidationData> {
        self.data
            .iter()
            .filter(|liq| liq.side.eq_ignore_ascii_case(side))
            .collect()
    }

    /// Returns all long liquidation entries.
    pub fn long_liquidations(&self) -> Vec<&AllLiquidationData> {
        self.filter_by_side("Buy")
    }

    /// Returns all short liquidation entries.
    pub fn short_liquidations(&self) -> Vec<&AllLiquidationData> {
        self.filter_by_side("Sell")
    }

    /// Returns the most recent liquidation entry (by updated_time).
    pub fn most_recent(&self) -> Option<&AllLiquidationData> {
        self.data.iter().max_by_key(|liq| liq.updated_time)
    }

    /// Returns the oldest liquidation entry (by updated_time).
    pub fn oldest(&self) -> Option<&AllLiquidationData> {
        self.data.iter().min_by_key(|liq| liq.updated_time)
    }

    /// Returns a summary string for this liquidation update.
    pub fn to_summary_string(&self) -> String {
        let symbol = self.symbol_from_topic().unwrap_or("unknown");
        format!(
            "[{}] {}: {} liquidations ({} long, {} short), Total=${:.2}, Imbalance={:.2}%",
            self.timestamp_datetime().format("%H:%M:%S%.3f"),
            symbol,
            self.count(),
            self.long_liquidations().len(),
            self.short_liquidations().len(),
            self.total_notional_value(),
            self.net_imbalance_percentage()
        )
    }

    /// Validates the update for trading use.
    ///
    /// Returns `true` if:
    /// 1. The update is not stale (≤ 1 second old)
    /// 2. The symbol can be extracted from the topic
    /// 3. The update is a snapshot (all liquidation updates should be snapshots)
    pub fn is_valid_for_trading(&self) -> bool {
        !self.is_stale() && self.symbol_from_topic().is_some() && self.is_snapshot()
    }

    /// Returns the update latency in milliseconds.
    ///
    /// For comparing with other market data timestamps.
    pub fn latency_ms(&self, other_timestamp: u64) -> i64 {
        if self.timestamp > other_timestamp {
            (self.timestamp - other_timestamp) as i64
        } else {
            (other_timestamp - self.timestamp) as i64
        }
    }

    /// Groups liquidations by symbol (useful for multi-symbol topics if supported).
    pub fn group_by_symbol(&self) -> std::collections::HashMap<String, Vec<&AllLiquidationData>> {
        let mut groups = std::collections::HashMap::new();
        for liq in &self.data {
            groups
                .entry(liq.symbol.clone())
                .or_insert_with(Vec::new)
                .push(liq);
        }
        groups
    }

    /// Returns the estimated total market impact of all liquidations.
    ///
    /// Using a simplified model: total_impact = sum(impact_coefficient * sqrt(notional_value))
    pub fn estimated_total_market_impact(&self, impact_coefficient: f64) -> f64 {
        self.data
            .iter()
            .map(|liq| liq.estimated_price_impact(impact_coefficient))
            .sum()
    }

    /// Returns the liquidation with the largest notional value.
    pub fn largest_liquidation(&self) -> Option<&AllLiquidationData> {
        self.data.iter().max_by(|a, b| {
            a.notional_value()
                .partial_cmp(&b.notional_value())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Returns the liquidation with the smallest notional value.
    pub fn smallest_liquidation(&self) -> Option<&AllLiquidationData> {
        self.data.iter().min_by(|a, b| {
            a.notional_value()
                .partial_cmp(&b.notional_value())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}
