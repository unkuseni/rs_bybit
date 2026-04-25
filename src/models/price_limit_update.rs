use crate::prelude::*;

/// Represents the order price limits for a single trading symbol in a WebSocket update.
///
/// Contains the highest bid price (buyLmt) and lowest ask price (sellLmt) for a given symbol.
/// These limits define the order price boundaries for derivative or spot trading and are
/// important for risk management and order validation.
///
/// # Bybit API Reference
/// The Bybit WebSocket API (https://bybit-exchange.github.io/docs/v5/websocket/public/order-price-limit)
/// provides real-time order price limit updates with a push frequency of 300ms.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceLimitData {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the specific instrument for which the price limits apply.
    /// Trading bots should verify this matches the requested symbol to ensure data integrity.
    pub symbol: String,

    /// The highest bid price allowed (buy limit) as a string.
    ///
    /// Represents the maximum price at which buy orders can be placed.
    /// Orders with prices above this limit will be rejected.
    /// Trading bots must ensure buy order prices do not exceed this limit.
    #[serde(with = "string_to_float")]
    pub buy_lmt: f64,

    /// The lowest ask price allowed (sell limit) as a string.
    ///
    /// Represents the minimum price at which sell orders can be placed.
    /// Orders with prices below this limit will be rejected.
    /// Trading bots must ensure sell order prices are not below this limit.
    #[serde(with = "string_to_float")]
    pub sell_lmt: f64,
}

impl PriceLimitData {
    /// Constructs a new PriceLimitData with specified parameters.
    pub fn new(symbol: &str, buy_lmt: f64, sell_lmt: f64) -> Self {
        Self {
            symbol: symbol.to_string(),
            buy_lmt,
            sell_lmt,
        }
    }

    /// Returns true if the buy limit is valid (positive value).
    pub fn is_buy_limit_valid(&self) -> bool {
        self.buy_lmt > 0.0
    }

    /// Returns true if the sell limit is valid (positive value).
    pub fn is_sell_limit_valid(&self) -> bool {
        self.sell_lmt > 0.0
    }

    /// Returns true if both price limits are valid.
    pub fn is_valid(&self) -> bool {
        self.is_buy_limit_valid() && self.is_sell_limit_valid()
    }

    /// Returns the price range between buy and sell limits.
    ///
    /// Calculated as `sell_lmt - buy_lmt`. A positive value indicates normal market conditions
    /// where sell limit is higher than buy limit. A negative value would indicate abnormal
    /// market conditions.
    pub fn price_range(&self) -> f64 {
        self.sell_lmt - self.buy_lmt
    }

    /// Returns the mid price between buy and sell limits.
    ///
    /// Calculated as `(buy_lmt + sell_lmt) / 2.0`. This represents the theoretical
    /// fair value within the allowed trading range.
    pub fn mid_price(&self) -> f64 {
        (self.buy_lmt + self.sell_lmt) / 2.0
    }

    /// Returns the price range as a percentage of the mid price.
    ///
    /// Useful for understanding the relative width of the allowed trading range.
    /// Calculated as `price_range() / mid_price() * 100.0`.
    pub fn price_range_percentage(&self) -> f64 {
        let range = self.price_range();
        let mid = self.mid_price();
        if mid != 0.0 {
            (range / mid) * 100.0
        } else {
            0.0
        }
    }

    /// Checks if a buy price is within the allowed limit.
    ///
    /// Returns `true` if `price <= buy_lmt`, meaning the buy order price is acceptable.
    pub fn is_buy_price_allowed(&self, price: f64) -> bool {
        price <= self.buy_lmt
    }

    /// Checks if a sell price is within the allowed limit.
    ///
    /// Returns `true` if `price >= sell_lmt`, meaning the sell order price is acceptable.
    pub fn is_sell_price_allowed(&self, price: f64) -> bool {
        price >= self.sell_lmt
    }

    /// Returns the maximum allowable slippage for buy orders.
    ///
    /// Calculated as `(buy_lmt - reference_price) / reference_price` where `reference_price`
    /// is typically the current market price or the price a bot intends to use.
    pub fn buy_slippage_limit(&self, reference_price: f64) -> f64 {
        if reference_price != 0.0 {
            (self.buy_lmt - reference_price) / reference_price
        } else {
            0.0
        }
    }

    /// Returns the maximum allowable slippage for sell orders.
    ///
    /// Calculated as `(reference_price - sell_lmt) / reference_price` where `reference_price`
    /// is typically the current market price or the price a bot intends to use.
    pub fn sell_slippage_limit(&self, reference_price: f64) -> f64 {
        if reference_price != 0.0 {
            (reference_price - self.sell_lmt) / reference_price
        } else {
            0.0
        }
    }

    /// Returns the price limits as a tuple (buy_limit, sell_limit).
    pub fn limits(&self) -> (f64, f64) {
        (self.buy_lmt, self.sell_lmt)
    }

    /// Returns a string representation of the price limits.
    pub fn to_display_string(&self) -> String {
        format!(
            "{}: Buy ≤ {:.8}, Sell ≥ {:.8} (Range: {:.2}%)",
            self.symbol,
            self.buy_lmt,
            self.sell_lmt,
            self.price_range_percentage()
        )
    }

    /// Returns the timestamp from the symbol, if it contains expiry information.
    ///
    /// For futures contracts with expiry dates in the symbol (e.g., "BTC-26DEC25"),
    /// this attempts to extract and parse the expiry date.
    pub fn expiry_timestamp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        // Try to parse futures symbol format: BASE-EXPIRY or BASE-EXPIRY-STRIKE-TYPE
        let parts: Vec<&str> = self.symbol.split('-').collect();
        if parts.len() >= 2 {
            let expiry_str = parts[1];
            // Try to parse as DDMMMYY or DDMMMYYYY
            if let Ok(dt) = chrono::NaiveDate::parse_from_str(expiry_str, "%d%b%y") {
                return Some(chrono::DateTime::from_naive_utc_and_offset(
                    dt.and_hms_opt(8, 0, 0).unwrap_or_default(), // Bybit delivery at 08:00 UTC
                    chrono::Utc,
                ));
            } else if let Ok(dt) = chrono::NaiveDate::parse_from_str(expiry_str, "%d%b%Y") {
                return Some(chrono::DateTime::from_naive_utc_and_offset(
                    dt.and_hms_opt(8, 0, 0).unwrap_or_default(),
                    chrono::Utc,
                ));
            }
        }
        None
    }

    /// Returns true if this is a perpetual contract (no expiry).
    pub fn is_perpetual(&self) -> bool {
        !self.symbol.contains('-') || self.symbol.ends_with("USDT") || self.symbol.ends_with("USDC")
    }

    /// Returns true if this is a futures contract (has expiry).
    pub fn is_futures(&self) -> bool {
        !self.is_perpetual() && self.symbol.contains('-')
    }
}

/// Represents a WebSocket price limit update event.
///
/// Contains real-time updates to order price limits for trading symbols.
/// Push frequency: 300ms.
///
/// # Bybit API Reference
/// Topic: `priceLimit.{symbol}` (e.g., `priceLimit.BTCUSDT`)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceLimitUpdate {
    /// The WebSocket topic for the event (e.g., "priceLimit.BTCUSDT").
    ///
    /// Specifies the data stream for the price limit update.
    /// Bots use this to determine which symbol the update belongs to.
    #[serde(rename = "topic")]
    pub topic: String,

    /// The timestamp of the event in milliseconds.
    ///
    /// Indicates when the price limit update was generated by the system.
    /// Bots use this to ensure data freshness and time-based analysis.
    #[serde(rename = "ts")]
    pub timestamp: u64,

    /// The price limit data.
    ///
    /// Contains the current buy and sell price limits for the symbol.
    #[serde(rename = "data")]
    pub data: PriceLimitData,
}

impl PriceLimitUpdate {
    /// Extracts the symbol from the topic.
    ///
    /// Parses the WebSocket topic to extract the trading symbol.
    /// Example: "priceLimit.BTCUSDT" -> "BTCUSDT"
    pub fn symbol_from_topic(&self) -> Option<&str> {
        self.topic.split('.').next_back()
    }

    /// Returns true if the symbol in the topic matches the symbol in the data.
    ///
    /// Validates data consistency between the topic and the embedded data.
    pub fn is_consistent(&self) -> bool {
        if let Some(topic_symbol) = self.symbol_from_topic() {
            topic_symbol == self.data.symbol
        } else {
            false
        }
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
        now.saturating_sub(self.timestamp)
    }

    /// Returns true if the update is stale (older than 1 second).
    ///
    /// Since price limit updates are pushed every 300ms, data older than 1 second
    /// might be considered stale for real-time trading decisions.
    pub fn is_stale(&self) -> bool {
        self.age_ms() > 1000
    }

    /// Returns the buy limit from the embedded data.
    pub fn buy_limit(&self) -> f64 {
        self.data.buy_lmt
    }

    /// Returns the sell limit from the embedded data.
    pub fn sell_limit(&self) -> f64 {
        self.data.sell_lmt
    }

    /// Returns the price range between buy and sell limits.
    pub fn price_range(&self) -> f64 {
        self.data.price_range()
    }

    /// Returns the mid price between buy and sell limits.
    pub fn mid_price(&self) -> f64 {
        self.data.mid_price()
    }

    /// Returns the price range as a percentage of the mid price.
    pub fn price_range_percentage(&self) -> f64 {
        self.data.price_range_percentage()
    }

    /// Checks if a buy price is within the allowed limit.
    pub fn is_buy_price_allowed(&self, price: f64) -> bool {
        self.data.is_buy_price_allowed(price)
    }

    /// Checks if a sell price is within the allowed limit.
    pub fn is_sell_price_allowed(&self, price: f64) -> bool {
        self.data.is_sell_price_allowed(price)
    }

    /// Returns the maximum allowable slippage for buy orders.
    pub fn buy_slippage_limit(&self, reference_price: f64) -> f64 {
        self.data.buy_slippage_limit(reference_price)
    }

    /// Returns the maximum allowable slippage for sell orders.
    pub fn sell_slippage_limit(&self, reference_price: f64) -> f64 {
        self.data.sell_slippage_limit(reference_price)
    }

    /// Returns a string representation of the update.
    pub fn to_display_string(&self) -> String {
        format!(
            "[{}] {} (Age: {}ms)",
            self.timestamp_datetime().format("%H:%M:%S%.3f"),
            self.data.to_display_string(),
            self.age_ms()
        )
    }

    /// Validates the update for trading use.
    ///
    /// Returns `true` if:
    /// 1. The topic and data symbols are consistent
    /// 2. The price limits are valid (positive values)
    /// 3. The update is not stale (≤ 1 second old)
    /// 4. The sell limit is greater than the buy limit (normal market)
    pub fn is_valid_for_trading(&self) -> bool {
        self.is_consistent()
            && self.data.is_valid()
            && !self.is_stale()
            && self.data.sell_lmt > self.data.buy_lmt
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
}
