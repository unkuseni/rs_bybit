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
    /// If `true`, the buyer's order was on the order book (maker); if `false`, the buyer took liquidity (taker). Bots use this to analyze market dynamics and order flow.
    #[serde(rename = "BT")]
    pub buyer_is_maker: bool,
}

impl WsTrade {
    /// Creates a new WsTrade instance.
    pub fn new(
        timestamp: u64,
        symbol: &str,
        side: &str,
        volume: f64,
        price: f64,
        tick_direction: TickDirection,
        id: &str,
        buyer_is_maker: bool,
    ) -> Self {
        Self {
            timestamp,
            symbol: symbol.to_string(),
            side: side.to_string(),
            volume,
            price,
            tick_direction,
            id: id.to_string(),
            buyer_is_maker,
        }
    }

    /// Returns true if this is a buy trade.
    pub fn is_buy(&self) -> bool {
        self.side.eq_ignore_ascii_case("buy")
    }

    /// Returns true if this is a sell trade.
    pub fn is_sell(&self) -> bool {
        self.side.eq_ignore_ascii_case("sell")
    }

    /// Returns the timestamp as a chrono DateTime.
    pub fn timestamp_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp((self.timestamp / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now)
    }

    /// Returns the age of the trade in milliseconds.
    pub fn age_ms(&self) -> u64 {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        now.saturating_sub(self.timestamp)
    }

    /// Returns true if the trade is stale (older than 5 seconds).
    pub fn is_stale(&self) -> bool {
        self.age_ms() > 5000
    }

    /// Returns the trade value (price * volume).
    pub fn value(&self) -> f64 {
        self.price * self.volume
    }

    /// Returns true if this is a taker trade (buyer is not maker).
    pub fn is_taker(&self) -> bool {
        !self.buyer_is_maker
    }

    /// Returns true if this is a maker trade (buyer is maker).
    pub fn is_maker(&self) -> bool {
        self.buyer_is_maker
    }

    /// Returns the trade type as a string.
    pub fn trade_type(&self) -> String {
        if self.is_buy() {
            if self.is_maker() {
                "Buy Maker".to_string()
            } else {
                "Buy Taker".to_string()
            }
        } else {
            if self.is_maker() {
                "Sell Maker".to_string()
            } else {
                "Sell Taker".to_string()
            }
        }
    }

    /// Returns true if this is an uptick trade.
    pub fn is_uptick(&self) -> bool {
        matches!(
            self.tick_direction,
            TickDirection::PlusTick | TickDirection::ZeroPlusTick
        )
    }

    /// Returns true if this is a downtick trade.
    pub fn is_downtick(&self) -> bool {
        matches!(
            self.tick_direction,
            TickDirection::MinusTick | TickDirection::ZeroMinusTick
        )
    }

    /// Returns true if this is a neutral tick trade.
    pub fn is_neutral_tick(&self) -> bool {
        matches!(
            self.tick_direction,
            TickDirection::ZeroPlusTick | TickDirection::ZeroMinusTick
        )
    }

    /// Returns the tick direction as a human-readable string.
    pub fn tick_direction_string(&self) -> &'static str {
        match self.tick_direction {
            TickDirection::PlusTick => "PlusTick",
            TickDirection::ZeroPlusTick => "ZeroPlusTick",
            TickDirection::MinusTick => "MinusTick",
            TickDirection::ZeroMinusTick => "ZeroMinusTick",
        }
    }

    /// Returns true if the trade is valid for analysis.
    pub fn is_valid(&self) -> bool {
        self.timestamp > 0
            && !self.symbol.is_empty()
            && (self.is_buy() || self.is_sell())
            && self.volume > 0.0
            && self.price > 0.0
            && !self.id.is_empty()
    }

    /// Returns a summary string for this trade.
    pub fn to_summary_string(&self) -> String {
        format!(
            "[{}] {} {} {} @ {} (Value: {:.2}, {})",
            self.timestamp_datetime().format("%H:%M:%S%.3f"),
            self.symbol,
            self.side,
            self.volume,
            self.price,
            self.value(),
            self.trade_type()
        )
    }

    /// Returns a compact summary string for this trade.
    pub fn to_compact_string(&self) -> String {
        let side_char = if self.is_buy() { 'B' } else { 'S' };
        let maker_char = if self.is_maker() { 'M' } else { 'T' };
        let tick_char = match self.tick_direction {
            TickDirection::PlusTick => '↑',
            TickDirection::ZeroPlusTick => '↗',
            TickDirection::MinusTick => '↓',
            TickDirection::ZeroMinusTick => '↘',
        };

        format!(
            "{} {}{}{} {}@{:.2}",
            self.timestamp_datetime().format("%H:%M:%S"),
            side_char,
            maker_char,
            tick_char,
            self.volume,
            self.price
        )
    }

    /// Returns the trade size category.
    pub fn size_category(&self) -> TradeSizeCategory {
        let value = self.value();
        if value >= 1_000_000.0 {
            TradeSizeCategory::Whale
        } else if value >= 100_000.0 {
            TradeSizeCategory::Large
        } else if value >= 10_000.0 {
            TradeSizeCategory::Medium
        } else if value >= 1_000.0 {
            TradeSizeCategory::Small
        } else {
            TradeSizeCategory::Retail
        }
    }

    /// Returns the trade size category as a string.
    pub fn size_category_string(&self) -> &'static str {
        match self.size_category() {
            TradeSizeCategory::Whale => "Whale",
            TradeSizeCategory::Large => "Large",
            TradeSizeCategory::Medium => "Medium",
            TradeSizeCategory::Small => "Small",
            TradeSizeCategory::Retail => "Retail",
        }
    }

    /// Returns the notional value in quote currency.
    pub fn notional_value(&self) -> f64 {
        self.value()
    }

    /// Returns the trade impact (volume / price).
    /// This can be used to estimate the price impact of the trade.
    pub fn impact_ratio(&self) -> f64 {
        self.volume / self.price
    }

    /// Returns true if this trade occurred within the last N milliseconds.
    pub fn is_recent(&self, max_age_ms: u64) -> bool {
        self.age_ms() <= max_age_ms
    }

    /// Compares this trade with another trade and returns the price difference.
    pub fn price_diff(&self, other: &WsTrade) -> Option<f64> {
        if self.symbol == other.symbol {
            Some(self.price - other.price)
        } else {
            None
        }
    }

    /// Compares this trade with another trade and returns the price difference percentage.
    pub fn price_diff_percentage(&self, other: &WsTrade) -> Option<f64> {
        if self.symbol == other.symbol && other.price > 0.0 {
            Some((self.price - other.price) / other.price * 100.0)
        } else {
            None
        }
    }

    /// Returns the trade data as a tuple for easy pattern matching.
    pub fn as_tuple(&self) -> (u64, &str, &str, f64, f64, TickDirection, &str, bool) {
        (
            self.timestamp,
            &self.symbol,
            &self.side,
            self.volume,
            self.price,
            self.tick_direction,
            &self.id,
            self.buyer_is_maker,
        )
    }
}

/// Enum representing trade size categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeSizeCategory {
    /// Retail trade: < $1,000
    Retail,
    /// Small trade: $1,000 - $10,000
    Small,
    /// Medium trade: $10,000 - $100,000
    Medium,
    /// Large trade: $100,000 - $1,000,000
    Large,
    /// Whale trade: ≥ $1,000,000
    Whale,
}

impl TradeSizeCategory {
    /// Returns the minimum value for this category.
    pub fn min_value(&self) -> f64 {
        match self {
            TradeSizeCategory::Retail => 0.0,
            TradeSizeCategory::Small => 1_000.0,
            TradeSizeCategory::Medium => 10_000.0,
            TradeSizeCategory::Large => 100_000.0,
            TradeSizeCategory::Whale => 1_000_000.0,
        }
    }

    /// Returns the maximum value for this category.
    pub fn max_value(&self) -> Option<f64> {
        match self {
            TradeSizeCategory::Retail => Some(1_000.0),
            TradeSizeCategory::Small => Some(10_000.0),
            TradeSizeCategory::Medium => Some(100_000.0),
            TradeSizeCategory::Large => Some(1_000_000.0),
            TradeSizeCategory::Whale => None,
        }
    }

    /// Returns a string representation of the category.
    pub fn as_str(&self) -> &'static str {
        match self {
            TradeSizeCategory::Retail => "Retail",
            TradeSizeCategory::Small => "Small",
            TradeSizeCategory::Medium => "Medium",
            TradeSizeCategory::Large => "Large",
            TradeSizeCategory::Whale => "Whale",
        }
    }
}

impl std::fmt::Display for TradeSizeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
