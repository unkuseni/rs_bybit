use chrono::{DateTime, Utc};

use crate::models::linear_ticker::LinearTickerData;
use crate::prelude::*;

/// WebSocket ticker message from Bybit.
///
/// This struct represents a ticker update received via WebSocket for either linear perpetual futures or spot markets. Bots use this to process real-time market data for trading decisions, including price tracking, volume analysis, and funding rate monitoring.
///
/// # Bybit API Reference
/// The Bybit WebSocket API (https://bybit-exchange.github.io/docs/v5/ws/connect) streams ticker data for subscribed topics. This struct deserializes the JSON payload into a structured format for bot consumption.
///
/// # Perpetual Futures Context
/// For perpetual futures, ticker data includes funding rates, open interest, and mark/index prices, which are critical for bots managing positions and monitoring funding costs. Spot ticker data provides reference prices for arbitrage strategies.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsTicker {
    /// The topic (channel) of the WebSocket message.
    ///
    /// Identifies the type of data stream (e.g., "tickers.BTCUSDT"). Bots use this to route messages to appropriate handlers.
    pub topic: String,

    /// The event type (e.g., "snapshot", "delta").
    ///
    /// Indicates whether the message is a full snapshot or incremental update. Bots use this to determine how to update their internal state.
    #[serde(rename = "type")]
    pub event_type: String,

    /// The ticker data payload.
    ///
    /// Contains market-specific metrics for linear perpetuals or spot markets. Bots extract fields like last price, volume, and funding rates from this data.
    pub data: Ticker,

    /// The cross sequence identifier.
    ///
    /// Used to ensure message ordering and detect gaps in the data stream. Bots can use this to validate data integrity.
    #[serde(rename = "cs")]
    pub cs: i64,

    /// The timestamp in milliseconds.
    ///
    /// The server timestamp when the message was generated. Bots use this to calculate latency and age of market data.
    #[serde(rename = "ts")]
    pub ts: u64,
}

impl WsTicker {
    /// Creates a new `WsTicker` instance.
    ///
    /// Bots use this constructor when synthesizing ticker data for testing or simulation purposes.
    pub fn new(topic: String, event_type: String, data: Ticker, cs: i64, ts: u64) -> Self {
        Self {
            topic,
            event_type,
            data,
            cs,
            ts,
        }
    }

    /// Extracts the symbol from the ticker data.
    ///
    /// Returns the trading pair symbol (e.g., "BTCUSDT"). Bots use this to identify the market for the ticker update.
    pub fn symbol(&self) -> &str {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => &snapshot.symbol,
                LinearTickerData::Delta(delta) => &delta.symbol,
            },
            Ticker::Spot(spot_data) => &spot_data.symbol,
            Ticker::Options(options_ticker) => &options_ticker.symbol,
            Ticker::Futures(futures_ticker) => &futures_ticker.symbol,
        }
    }

    /// Checks if the ticker data is a snapshot.
    ///
    /// Returns `true` if the event type is "snapshot". Bots use this to determine whether to replace or update their market data.
    pub fn is_snapshot(&self) -> bool {
        self.event_type == "snapshot"
    }

    /// Checks if the ticker data is a delta (incremental update).
    ///
    /// Returns `true` if the event type is "delta". Bots use this to apply incremental updates to their market data.
    pub fn is_delta(&self) -> bool {
        self.event_type == "delta"
    }

    /// Converts the timestamp to a `DateTime<Utc>`.
    ///
    /// Returns a UTC datetime representation of the timestamp. Bots use this for time-based analysis and logging.
    pub fn timestamp_datetime(&self) -> DateTime<Utc> {
        let secs = (self.ts / 1000) as i64;
        let nanos = ((self.ts % 1000) * 1_000_000) as u32;
        DateTime::from_timestamp(secs, nanos).unwrap_or_else(|| Utc::now())
    }

    /// Calculates the age of the ticker data in milliseconds.
    ///
    /// Returns the time elapsed since the ticker was generated. Bots use this to filter stale data and monitor latency.
    pub fn age_ms(&self) -> u64 {
        let now = Utc::now().timestamp_millis() as u64;
        now.saturating_sub(self.ts)
    }

    /// Checks if the ticker data is stale.
    ///
    /// Returns `true` if the data is older than 5000ms (5 seconds). Bots use this to avoid acting on outdated market information.
    pub fn is_stale(&self) -> bool {
        self.age_ms() > 5000
    }

    /// Extracts the last traded price.
    ///
    /// Returns the most recent trade price, or `None` if not available. Bots use this for real-time price tracking and order execution.
    pub fn last_price(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.last_price),
                LinearTickerData::Delta(delta) => delta.last_price,
            },
            Ticker::Spot(spot_data) => Some(spot_data.last_price),
            Ticker::Options(options_ticker) => options_ticker.last_price_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.last_price),
        }
    }

    /// Extracts the 24-hour price change percentage.
    ///
    /// Returns the percentage price change over the last 24 hours, or `None` if not available. Bots use this to assess market trends and volatility.
    pub fn price_change_24h(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.price_24h_pcnt),
                LinearTickerData::Delta(delta) => delta.price_24h_pcnt,
            },
            Ticker::Spot(spot_data) => Some(spot_data.price_24h_pcnt),
            Ticker::Options(options_ticker) => options_ticker.change_24h_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.daily_change_percentage),
        }
    }

    /// Extracts the 24-hour high price.
    ///
    /// Returns the highest price in the last 24 hours, or `None` if not available. Bots use this to identify resistance levels and assess volatility.
    pub fn high_24h(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.high_price_24h),
                LinearTickerData::Delta(delta) => delta.high_price_24h,
            },
            Ticker::Spot(spot_data) => Some(spot_data.high_price_24h),
            Ticker::Options(options_ticker) => options_ticker.high_price_24h_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.high_24h),
        }
    }

    /// Extracts the 24-hour low price.
    ///
    /// Returns the lowest price in the last 24 hours, or `None` if not available. Bots use this to identify support levels and assess volatility.
    pub fn low_24h(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.low_price_24h),
                LinearTickerData::Delta(delta) => delta.low_price_24h,
            },
            Ticker::Spot(spot_data) => Some(spot_data.low_price_24h),
            Ticker::Options(options_ticker) => options_ticker.low_price_24h_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.low_24h),
        }
    }

    /// Extracts the 24-hour trading volume.
    ///
    /// Returns the trading volume in the last 24 hours, or `None` if not available. Bots use this to analyze market activity and liquidity.
    pub fn volume_24h(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.volume_24h),
                LinearTickerData::Delta(delta) => delta.volume_24h,
            },
            Ticker::Spot(spot_data) => Some(spot_data.volume_24h),
            Ticker::Options(options_ticker) => options_ticker.volume_24h_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.volume_24h),
        }
    }

    /// Extracts the 24-hour trading turnover.
    ///
    /// Returns the trading value in the last 24 hours, or `None` if not available. Bots use this to assess market activity and liquidity in monetary terms.
    pub fn turnover_24h(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.turnover_24h),
                LinearTickerData::Delta(delta) => delta.turnover_24h,
            },
            Ticker::Spot(spot_data) => Some(spot_data.turnover_24h),
            Ticker::Options(options_ticker) => options_ticker.turnover_24h_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.turnover_24h),
        }
    }

    /// Extracts the funding rate (linear perpetuals only).
    ///
    /// Returns the funding rate for linear perpetuals, or `None` for spot markets. Bots use this to monitor funding costs and arbitrage opportunities.
    pub fn funding_rate(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.funding_rate),
                LinearTickerData::Delta(delta) => delta.funding_rate,
            },
            Ticker::Spot(_) => None,
            Ticker::Options(_) => None,
            Ticker::Futures(futures_ticker) => futures_ticker.funding_rate_f64(),
        }
    }

    /// Extracts the next funding time (linear perpetuals only).
    ///
    /// Returns the timestamp of the next funding settlement, or `None` for spot markets. Bots use this to schedule funding-related operations.
    pub fn next_funding_time(&self) -> Option<u64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.next_funding_time),
                LinearTickerData::Delta(delta) => delta.next_funding_time,
            },
            Ticker::Spot(_) => None,
            Ticker::Options(_) => None,
            Ticker::Futures(futures_ticker) => Some(futures_ticker.next_funding_time),
        }
    }

    /// Extracts the best bid price.
    ///
    /// Returns the highest bid price in the order book, or `None` if not available. Bots use this for spread calculations and liquidity assessment.
    pub fn bid_price(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.bid_price),
                LinearTickerData::Delta(delta) => delta.bid_price,
            },
            Ticker::Spot(_) => None, // Spot ticker doesn't have bid/ask prices
            Ticker::Options(options_ticker) => options_ticker.bid1_price_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.bid_price),
        }
    }

    /// Extracts the best bid size.
    ///
    /// Returns the quantity available at the best bid price, or `None` if not available. Bots use this to evaluate buy-side liquidity.
    pub fn bid_size(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.bid_size),
                LinearTickerData::Delta(delta) => delta.bid_size,
            },
            Ticker::Spot(_) => None, // Spot ticker doesn't have bid/ask sizes
            Ticker::Options(options_ticker) => options_ticker.bid1_size_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.bid_size),
        }
    }

    /// Extracts the best ask price.
    ///
    /// Returns the lowest ask price in the order book, or `None` if not available. Bots use this for spread calculations and liquidity assessment.
    pub fn ask_price(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.ask_price),
                LinearTickerData::Delta(delta) => delta.ask_price,
            },
            Ticker::Spot(_) => None, // Spot ticker doesn't have bid/ask prices
            Ticker::Options(options_ticker) => options_ticker.ask1_price_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.ask_price),
        }
    }

    /// Extracts the best ask size.
    ///
    /// Returns the quantity available at the best ask price, or `None` if not available. Bots use this to evaluate sell-side liquidity.
    pub fn ask_size(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.ask_size),
                LinearTickerData::Delta(delta) => delta.ask_size,
            },
            Ticker::Spot(_) => None, // Spot ticker doesn't have bid/ask sizes
            Ticker::Options(options_ticker) => options_ticker.ask1_size_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.ask_size),
        }
    }

    /// Calculates the bid-ask spread.
    ///
    /// Returns the difference between ask and bid prices, or `None` if either is missing. Bots use this to assess market liquidity and trading costs.
    pub fn spread(&self) -> Option<f64> {
        match (self.ask_price(), self.bid_price()) {
            (Some(ask), Some(bid)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Calculates the mid price.
    ///
    /// Returns the average of bid and ask prices, or `None` if either is missing. Bots use this as a reference price for market analysis.
    pub fn mid_price(&self) -> Option<f64> {
        match (self.ask_price(), self.bid_price()) {
            (Some(ask), Some(bid)) => Some((ask + bid) / 2.0),
            _ => None,
        }
    }

    /// Calculates the spread as a percentage of the mid price.
    ///
    /// Returns the relative spread (spread / mid price), or `None` if insufficient data. Bots use this to compare liquidity across different markets.
    pub fn spread_percentage(&self) -> Option<f64> {
        match (self.spread(), self.mid_price()) {
            (Some(spread), Some(mid)) if mid > 0.0 => Some(spread / mid * 100.0),
            _ => None,
        }
    }

    /// Extracts the open interest (linear perpetuals only).
    ///
    /// Returns the total number of open contracts, or `None` for spot markets. Bots use this to gauge market sentiment and positioning.
    pub fn open_interest(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.open_interest),
                LinearTickerData::Delta(delta) => delta.open_interest,
            },
            Ticker::Spot(_) => None,
            Ticker::Options(options_ticker) => options_ticker.open_interest_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.open_interest),
        }
    }

    /// Extracts the open interest value (linear perpetuals only).
    ///
    /// Returns the monetary value of open interest, or `None` for spot markets. Bots use this to assess market exposure and leverage levels.
    pub fn open_interest_value(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.open_interest_value),
                LinearTickerData::Delta(delta) => delta.open_interest_value,
            },
            Ticker::Spot(_) => None,
            Ticker::Options(_) => None, // Options ticker doesn't have open interest value
            Ticker::Futures(futures_ticker) => Some(futures_ticker.open_interest_value),
        }
    }

    /// Extracts the mark price.
    ///
    /// Returns the mark price used for liquidation calculations, or `None` if not available. Bots use this to monitor position health and avoid liquidation.
    pub fn mark_price(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.mark_price),
                LinearTickerData::Delta(delta) => delta.mark_price,
            },
            Ticker::Spot(_) => None, // Spot ticker doesn't have mark price
            Ticker::Options(options_ticker) => options_ticker.mark_price_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.mark_price),
        }
    }

    /// Extracts the index price.
    ///
    /// Returns the underlying index price, or `None` if not available. Bots use this to monitor basis (futures-spot spread) for arbitrage opportunities.
    pub fn index_price(&self) -> Option<f64> {
        match &self.data {
            Ticker::Linear(linear_data) => match linear_data {
                LinearTickerData::Snapshot(snapshot) => Some(snapshot.index_price),
                LinearTickerData::Delta(delta) => delta.index_price,
            },
            Ticker::Spot(spot_data) => Some(spot_data.usd_index_price),
            Ticker::Options(options_ticker) => options_ticker.index_price_f64(),
            Ticker::Futures(futures_ticker) => Some(futures_ticker.index_price),
        }
    }

    /// Returns true if the ticker data is valid for trading decisions.
    ///
    /// Checks that the data is not stale and has essential fields like symbol and last price. Bots use this to filter out invalid or incomplete market data.
    pub fn is_valid_for_trading(&self) -> bool {
        !self.is_stale() && self.last_price().is_some()
    }

    /// Returns a summary string for this ticker update.
    ///
    /// Provides a human-readable summary of key ticker metrics. Bots use this for logging and debugging purposes.
    pub fn to_summary_string(&self) -> String {
        let symbol = self.symbol();
        let last_price = self
            .last_price()
            .map(|p| format!("{:.2}", p))
            .unwrap_or_else(|| "N/A".to_string());
        let change_24h = self
            .price_change_24h()
            .map(|c| format!("{:+.2}%", c * 100.0))
            .unwrap_or_else(|| "N/A".to_string());
        let volume = self
            .volume_24h()
            .map(|v| format!("{:.2}", v))
            .unwrap_or_else(|| "N/A".to_string());

        format!(
            "[{}] {}: Last={}, 24h Change={}, Volume={}",
            self.timestamp_datetime().format("%H:%M:%S"),
            symbol,
            last_price,
            change_24h,
            volume
        )
    }

    /// Returns the price change amount over 24 hours.
    ///
    /// Calculates the absolute price change based on last price and 24-hour percentage change. Bots use this for volatility-based strategies and risk management.
    pub fn price_change_amount_24h(&self) -> Option<f64> {
        match (self.last_price(), self.price_change_24h()) {
            (Some(price), Some(change_pct)) => Some(price * change_pct),
            _ => None,
        }
    }

    /// Returns true if the price has increased over 24 hours.
    ///
    /// Checks if the 24-hour price change percentage is positive. Bots use this for trend-following strategies and market sentiment analysis.
    pub fn is_price_up_24h(&self) -> bool {
        self.price_change_24h().map(|c| c > 0.0).unwrap_or(false)
    }

    /// Returns true if the price has decreased over 24 hours.
    ///
    /// Checks if the 24-hour price change percentage is negative. Bots use this for trend-following strategies and market sentiment analysis.
    pub fn is_price_down_24h(&self) -> bool {
        self.price_change_24h().map(|c| c < 0.0).unwrap_or(false)
    }

    /// Returns the price range over 24 hours.
    ///
    /// Calculates the difference between 24-hour high and low prices. Bots use this to assess volatility and set stop-loss/take-profit levels.
    pub fn price_range_24h(&self) -> Option<f64> {
        match (self.high_24h(), self.low_24h()) {
            (Some(high), Some(low)) => Some(high - low),
            _ => None,
        }
    }

    /// Returns the current price position within the 24-hour range.
    ///
    /// Returns a value between 0.0 (at 24-hour low) and 1.0 (at 24-hour high). Bots use this for mean-reversion strategies and overbought/oversold indicators.
    pub fn price_position_in_range(&self) -> Option<f64> {
        match (self.last_price(), self.high_24h(), self.low_24h()) {
            (Some(price), Some(high), Some(low)) if high > low => {
                Some((price - low) / (high - low))
            }
            _ => None,
        }
    }

    /// Returns the volume-weighted average price over 24 hours.
    ///
    /// Calculates VWAP using 24-hour turnover and volume. Bots use this as a benchmark price for execution quality assessment.
    pub fn vwap_24h(&self) -> Option<f64> {
        match (self.turnover_24h(), self.volume_24h()) {
            (Some(turnover), Some(volume)) if volume > 0.0 => Some(turnover / volume),
            _ => None,
        }
    }

    /// Returns the premium/discount to index price.
    ///
    /// Calculates the percentage difference between last price and index price. Bots use this for arbitrage strategies between spot and futures markets.
    pub fn premium_to_index(&self) -> Option<f64> {
        match (self.last_price(), self.index_price()) {
            (Some(last), Some(index)) if index > 0.0 => Some((last - index) / index * 100.0),
            _ => None,
        }
    }

    /// Returns the funding rate annualized.
    ///
    /// Converts the funding rate to an annualized percentage. Bots use this to compare funding costs across different timeframes and markets.
    pub fn funding_rate_annualized(&self) -> Option<f64> {
        self.funding_rate().map(|rate| rate * 3.0 * 365.0)
    }

    /// Validates the checksum against the ticker data.
    ///
    /// Note: This is a placeholder implementation. Actual checksum validation
    /// would require the original message bytes. Bots should implement proper
    /// checksum validation for production use.
    pub fn validate_checksum(&self) -> bool {
        // In a real implementation, this would validate the checksum
        // against the actual data. For now, we assume it's valid.
        true
    }
}
