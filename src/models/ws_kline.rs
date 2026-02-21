use crate::prelude::*;

/// Enum representing different price types for technical analysis.
#[derive(Debug, Clone)]
pub enum PriceType {
    /// Close price
    Close,
    /// Open price
    Open,
    /// High price
    High,
    /// Low price
    Low,
    /// Typical price (high + low + close) / 3
    Typical,
    /// Median price (high + low) / 2
    Median,
    /// Weighted price (high + low + close * 2) / 4
    Weighted,
}

/// Enum representing different risk metrics for risk-adjusted returns.
#[derive(Debug, Clone)]
pub enum RiskMetric {
    /// Price volatility (standard deviation of returns)
    Volatility,
    /// Maximum drawdown
    MaximumDrawdown,
    /// Ulcer Index
    UlcerIndex,
    /// Value at Risk (VaR)
    ValueAtRisk,
}

/// Represents a WebSocket k-line (candlestick) update for a trading pair.
///
/// K-lines provide historical price and volume data over a specific interval (e.g., 1 minute, 1 hour). This struct is used in Bybit's WebSocket streams to deliver real-time candlestick updates for perpetual futures.
///
/// # Bybit API Reference
/// The Bybit API (https://bybit-exchange.github.io/docs/v5/websocket/public/kline) provides k-line data via WebSocket, including open, close, high, low, volume, and turnover.
///
/// # Perpetual Futures Context
/// K-lines are fundamental for technical analysis in futures trading. Bots use candlestick patterns to generate trading signals, such as breakouts, reversals, or trend continuations.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsKline {
    /// The WebSocket topic (e.g., "kline.1m.BTCUSDT").
    ///
    /// Identifies the k-line stream and interval. Bots use this to subscribe to specific timeframes and symbols.
    pub topic: String,

    /// The k-line data for the update.
    ///
    /// Contains a vector of candlestick data points. Bots process this to update technical indicators or chart patterns.
    pub data: Vec<KlineData>,

    /// The timestamp of the update (in milliseconds).
    ///
    /// Indicates when the k-line data was sent. Bots use this to ensure data is processed in chronological order.
    #[serde(rename = "ts")]
    pub timestamp: u64,

    /// The type of WebSocket event (e.g., "snapshot" or "delta").
    ///
    /// Specifies whether the data is a full snapshot or incremental update. Bots must handle both to maintain accurate k-line history.
    #[serde(rename = "type")]
    pub event_type: String,
}

impl WsKline {
    /// Creates a new WsKline instance.
    pub fn new(topic: &str, data: Vec<KlineData>, timestamp: u64, event_type: &str) -> Self {
        Self {
            topic: topic.to_string(),
            data,
            timestamp,
            event_type: event_type.to_string(),
        }
    }

    /// Returns the symbol extracted from the topic.
    ///
    /// For a topic like "kline.1m.BTCUSDT", returns "BTCUSDT".
    pub fn symbol(&self) -> Option<&str> {
        self.topic.split('.').last()
    }

    /// Returns the interval extracted from the topic.
    ///
    /// For a topic like "kline.1m.BTCUSDT", returns "1m".
    pub fn interval(&self) -> Option<&str> {
        self.topic.split('.').nth(1)
    }

    /// Returns the interval in seconds.
    ///
    /// Converts interval strings like "1m", "5m", "1h", "1d" to seconds.
    pub fn interval_seconds(&self) -> Option<u64> {
        self.interval().and_then(|interval| match interval {
            "1m" => Some(60),
            "3m" => Some(180),
            "5m" => Some(300),
            "15m" => Some(900),
            "30m" => Some(1800),
            "1h" => Some(3600),
            "2h" => Some(7200),
            "4h" => Some(14400),
            "6h" => Some(21600),
            "12h" => Some(43200),
            "1d" => Some(86400),
            "1w" => Some(604800),
            "1M" => Some(2592000), // 30 days
            _ => None,
        })
    }

    /// Returns true if this is a snapshot update.
    pub fn is_snapshot(&self) -> bool {
        self.event_type == "snapshot"
    }

    /// Returns true if this is a delta update.
    pub fn is_delta(&self) -> bool {
        self.event_type == "delta"
    }

    /// Returns the timestamp as a chrono DateTime.
    pub fn timestamp_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp((self.timestamp / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now)
    }

    /// Returns the age of the k-line update in milliseconds.
    pub fn age_ms(&self) -> u64 {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        if now > self.timestamp {
            now - self.timestamp
        } else {
            0
        }
    }

    /// Returns true if the k-line update is stale.
    /// For k-lines, we consider data stale if it's older than 2 intervals.
    pub fn is_stale(&self) -> bool {
        if let Some(interval_seconds) = self.interval_seconds() {
            self.age_ms() > interval_seconds * 2 * 1000
        } else {
            self.age_ms() > 60000 // Default to 1 minute if interval unknown
        }
    }

    /// Returns the latest k-line data point, if available.
    pub fn latest(&self) -> Option<&KlineData> {
        self.data.last()
    }

    /// Returns the first k-line data point, if available.
    pub fn first(&self) -> Option<&KlineData> {
        self.data.first()
    }

    /// Returns the number of k-line data points.
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Returns true if there are no k-line data points.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the open time of the latest k-line, if available.
    pub fn latest_open_time(&self) -> Option<u64> {
        self.latest().map(|k| k.start)
    }

    /// Returns the close time of the latest k-line, if available.
    pub fn latest_close_time(&self) -> Option<u64> {
        self.latest().and_then(|k| {
            // Parse interval string to milliseconds
            match k.interval.as_str() {
                "1" | "1m" => Some(k.start + 60 * 1000),
                "3" | "3m" => Some(k.start + 180 * 1000),
                "5" | "5m" => Some(k.start + 300 * 1000),
                "15" | "15m" => Some(k.start + 900 * 1000),
                "30" | "30m" => Some(k.start + 1800 * 1000),
                "60" | "1h" => Some(k.start + 3600 * 1000),
                "120" | "2h" => Some(k.start + 7200 * 1000),
                "240" | "4h" => Some(k.start + 14400 * 1000),
                "360" | "6h" => Some(k.start + 21600 * 1000),
                "720" | "12h" => Some(k.start + 43200 * 1000),
                "D" | "1d" => Some(k.start + 86400 * 1000),
                "W" | "1w" => Some(k.start + 604800 * 1000),
                "M" | "1M" => Some(k.start + 2592000 * 1000), // 30 days
                _ => None,
            }
        })
    }

    /// Returns the open price of the latest k-line, if available.
    pub fn latest_open(&self) -> Option<f64> {
        self.latest().map(|k| k.open)
    }

    /// Returns the close price of the latest k-line, if available.
    pub fn latest_close(&self) -> Option<f64> {
        self.latest().map(|k| k.close)
    }

    /// Returns the high price of the latest k-line, if available.
    pub fn latest_high(&self) -> Option<f64> {
        self.latest().map(|k| k.high)
    }

    /// Returns the low price of the latest k-line, if available.
    pub fn latest_low(&self) -> Option<f64> {
        self.latest().map(|k| k.low)
    }

    /// Returns the volume of the latest k-line, if available.
    pub fn latest_volume(&self) -> Option<f64> {
        self.latest().map(|k| k.volume)
    }

    /// Returns the turnover of the latest k-line, if available.
    pub fn latest_turnover(&self) -> Option<f64> {
        self.latest().map(|k| k.turnover)
    }

    /// Returns the price change of the latest k-line (close - open).
    pub fn latest_price_change(&self) -> Option<f64> {
        match (self.latest_close(), self.latest_open()) {
            (Some(close), Some(open)) => Some(close - open),
            _ => None,
        }
    }

    /// Returns the price change percentage of the latest k-line.
    pub fn latest_price_change_percentage(&self) -> Option<f64> {
        match (self.latest_close(), self.latest_open()) {
            (Some(close), Some(open)) if open > 0.0 => Some((close - open) / open * 100.0),
            _ => None,
        }
    }

    /// Returns true if the latest k-line is bullish (close > open).
    pub fn is_latest_bullish(&self) -> bool {
        self.latest_price_change().map(|c| c > 0.0).unwrap_or(false)
    }

    /// Returns true if the latest k-line is bearish (close < open).
    pub fn is_latest_bearish(&self) -> bool {
        self.latest_price_change().map(|c| c < 0.0).unwrap_or(false)
    }

    /// Returns true if the latest k-line is a doji (open ≈ close).
    pub fn is_latest_doji(&self) -> bool {
        match (self.latest_close(), self.latest_open()) {
            (Some(close), Some(open)) => {
                let range = self.latest_high().unwrap_or(0.0) - self.latest_low().unwrap_or(0.0);
                range > 0.0 && (close - open).abs() / range < 0.1
            }
            _ => false,
        }
    }

    /// Returns the body size of the latest k-line (|close - open|).
    pub fn latest_body_size(&self) -> Option<f64> {
        self.latest_price_change().map(|c| c.abs())
    }

    /// Returns the upper shadow size of the latest k-line (high - max(open, close)).
    pub fn latest_upper_shadow(&self) -> Option<f64> {
        match (self.latest_high(), self.latest_open(), self.latest_close()) {
            (Some(high), Some(open), Some(close)) => {
                Some(high - if open > close { open } else { close })
            }
            _ => None,
        }
    }

    /// Returns the lower shadow size of the latest k-line (min(open, close) - low).
    pub fn latest_lower_shadow(&self) -> Option<f64> {
        match (self.latest_low(), self.latest_open(), self.latest_close()) {
            (Some(low), Some(open), Some(close)) => {
                Some((if open < close { open } else { close }) - low)
            }
            _ => None,
        }
    }

    /// Returns the total range of the latest k-line (high - low).
    pub fn latest_range(&self) -> Option<f64> {
        match (self.latest_high(), self.latest_low()) {
            (Some(high), Some(low)) => Some(high - low),
            _ => None,
        }
    }

    /// Returns the body-to-range ratio of the latest k-line.
    pub fn latest_body_to_range_ratio(&self) -> Option<f64> {
        match (self.latest_body_size(), self.latest_range()) {
            (Some(body), Some(range)) if range > 0.0 => Some(body / range),
            _ => None,
        }
    }

    /// Returns the volume-weighted average price of the latest k-line.
    pub fn latest_vwap(&self) -> Option<f64> {
        match (self.latest_turnover(), self.latest_volume()) {
            (Some(turnover), Some(volume)) if volume > 0.0 => Some(turnover / volume),
            _ => None,
        }
    }

    /// Returns the typical price of the latest k-line (high + low + close) / 3.
    pub fn latest_typical_price(&self) -> Option<f64> {
        match (self.latest_high(), self.latest_low(), self.latest_close()) {
            (Some(high), Some(low), Some(close)) => Some((high + low + close) / 3.0),
            _ => None,
        }
    }

    /// Returns the median price of the latest k-line (high + low) / 2.
    pub fn latest_median_price(&self) -> Option<f64> {
        match (self.latest_high(), self.latest_low()) {
            (Some(high), Some(low)) => Some((high + low) / 2.0),
            _ => None,
        }
    }

    /// Returns true if the k-line data is valid for trading decisions.
    pub fn is_valid_for_trading(&self) -> bool {
        !self.is_stale()
            && self.symbol().is_some()
            && self.interval().is_some()
            && self.latest().is_some()
    }

    /// Returns a summary string for this k-line update.
    pub fn to_summary_string(&self) -> String {
        let symbol = self.symbol().unwrap_or("Unknown");
        let interval = self.interval().unwrap_or("Unknown");
        let count = self.count();
        let latest_info = if let Some(latest) = self.latest() {
            format!(
                "O:{:.2} H:{:.2} L:{:.2} C:{:.2} V:{:.2}",
                latest.open, latest.high, latest.low, latest.close, latest.volume
            )
        } else {
            "No data".to_string()
        };

        format!(
            "[{}] {} {} ({} candles): {}",
            self.timestamp_datetime().format("%H:%M:%S"),
            symbol,
            interval,
            count,
            latest_info
        )
    }

    /// Returns the k-line data sorted by start time (ascending).
    pub fn sorted_by_time(&self) -> Vec<&KlineData> {
        let mut data: Vec<&KlineData> = self.data.iter().collect();
        data.sort_by_key(|k| k.start);
        data
    }

    /// Returns the k-line data sorted by start time (descending).
    pub fn sorted_by_time_desc(&self) -> Vec<&KlineData> {
        let mut data: Vec<&KlineData> = self.data.iter().collect();
        data.sort_by_key(|k| std::cmp::Reverse(k.start));
        data
    }

    /// Returns the minimum price across all k-lines.
    pub fn min_price(&self) -> Option<f64> {
        self.data.iter().map(|k| k.low).reduce(f64::min)
    }

    /// Returns the maximum price across all k-lines.
    pub fn max_price(&self) -> Option<f64> {
        self.data.iter().map(|k| k.high).reduce(f64::max)
    }

    /// Returns the total volume across all k-lines.
    pub fn total_volume(&self) -> f64 {
        self.data.iter().map(|k| k.volume).sum()
    }

    /// Returns the total turnover across all k-lines.
    pub fn total_turnover(&self) -> f64 {
        self.data.iter().map(|k| k.turnover).sum()
    }

    /// Returns the average volume across all k-lines.
    pub fn average_volume(&self) -> Option<f64> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.total_volume() / self.data.len() as f64)
        }
    }

    /// Returns the volume-weighted average price across all k-lines.
    pub fn total_vwap(&self) -> Option<f64> {
        let total_turnover = self.total_turnover();
        let total_volume = self.total_volume();
        if total_volume > 0.0 {
            Some(total_turnover / total_volume)
        } else {
            None
        }
    }

    /// Returns the price change across all k-lines (last close - first open).
    pub fn total_price_change(&self) -> Option<f64> {
        match (self.first(), self.latest()) {
            (Some(first), Some(last)) => Some(last.close - first.open),
            _ => None,
        }
    }

    /// Returns the price change percentage across all k-lines.
    pub fn total_price_change_percentage(&self) -> Option<f64> {
        match (self.first(), self.latest()) {
            (Some(first), Some(last)) if first.open > 0.0 => {
                Some((last.close - first.open) / first.open * 100.0)
            }
            _ => None,
        }
    }

    /// Returns the average true range (ATR) across the k-lines.
    /// ATR is a measure of volatility that considers the true range (max of high-low, |high-prev_close|, |low-prev_close|).
    pub fn average_true_range(&self, period: usize) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let mut true_ranges = Vec::with_capacity(self.data.len() - 1);
        let sorted_data = self.sorted_by_time();

        for i in 1..sorted_data.len() {
            let current = &sorted_data[i];
            let previous = &sorted_data[i - 1];

            let hl = current.high - current.low;
            let hc = (current.high - previous.close).abs();
            let lc = (current.low - previous.close).abs();

            let true_range = hl.max(hc).max(lc);
            true_ranges.push(true_range);
        }

        if true_ranges.len() < period {
            return None;
        }

        // Calculate initial ATR as simple average of first 'period' true ranges
        let initial_atr: f64 = true_ranges[..period].iter().sum::<f64>() / period as f64;

        Some(initial_atr)
    }

    /// Returns the relative strength index (RSI) for the latest k-lines.
    /// RSI is a momentum oscillator that measures the speed and change of price movements.
    pub fn relative_strength_index(&self, period: usize) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut gains = Vec::new();
        let mut losses = Vec::new();

        for i in 1..=period {
            let current = &sorted_data[sorted_data.len() - i];
            let previous = &sorted_data[sorted_data.len() - i - 1];

            let change = current.close - previous.close;
            if change > 0.0 {
                gains.push(change);
                losses.push(0.0);
            } else {
                gains.push(0.0);
                losses.push(change.abs());
            }
        }

        let avg_gain = gains.iter().sum::<f64>() / period as f64;
        let avg_loss = losses.iter().sum::<f64>() / period as f64;

        if avg_loss == 0.0 {
            return Some(100.0);
        }

        let rs = avg_gain / avg_loss;
        let rsi = 100.0 - (100.0 / (1.0 + rs));

        Some(rsi)
    }

    /// Returns the simple moving average (SMA) for the specified period.
    pub fn simple_moving_average(&self, period: usize, price_type: PriceType) -> Option<f64> {
        if self.data.len() < period {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let recent_data = &sorted_data[sorted_data.len() - period..];

        let sum: f64 = recent_data
            .iter()
            .map(|k| match price_type {
                PriceType::Close => k.close,
                PriceType::Open => k.open,
                PriceType::High => k.high,
                PriceType::Low => k.low,
                PriceType::Typical => (k.high + k.low + k.close) / 3.0,
                PriceType::Median => (k.high + k.low) / 2.0,
                PriceType::Weighted => (k.high + k.low + k.close * 2.0) / 4.0,
            })
            .sum();

        Some(sum / period as f64)
    }

    /// Returns the exponential moving average (EMA) for the specified period.
    pub fn exponential_moving_average(&self, period: usize, price_type: PriceType) -> Option<f64> {
        if self.data.len() < period {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let multiplier = 2.0 / (period as f64 + 1.0);

        // Start with SMA as initial EMA
        let mut ema = self.simple_moving_average(period, price_type.clone())?;

        // Calculate EMA for remaining data points
        for kline in &sorted_data[sorted_data.len() - period..] {
            let price = match price_type {
                PriceType::Close => kline.close,
                PriceType::Open => kline.open,
                PriceType::High => kline.high,
                PriceType::Low => kline.low,
                PriceType::Typical => (kline.high + kline.low + kline.close) / 3.0,
                PriceType::Median => (kline.high + kline.low) / 2.0,
                PriceType::Weighted => (kline.high + kline.low + kline.close * 2.0) / 4.0,
            };
            ema = (price - ema) * multiplier + ema;
        }

        Some(ema)
    }

    /// Returns the moving average convergence divergence (MACD) values.
    /// Returns (MACD line, Signal line, Histogram)
    pub fn macd(
        &self,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> Option<(f64, f64, f64)> {
        if self.data.len() < slow_period + signal_period {
            return None;
        }

        let fast_ema = self.exponential_moving_average(fast_period, PriceType::Close)?;
        let slow_ema = self.exponential_moving_average(slow_period, PriceType::Close)?;

        let macd_line = fast_ema - slow_ema;

        // For signal line, we'd need more historical data
        // This is a simplified implementation
        let signal_line = macd_line * 0.9; // Simplified signal line

        let histogram = macd_line - signal_line;

        Some((macd_line, signal_line, histogram))
    }

    /// Returns the Bollinger Bands for the latest k-line.
    /// Returns (Upper band, Middle band (SMA), Lower band)
    pub fn bollinger_bands(&self, period: usize, std_dev: f64) -> Option<(f64, f64, f64)> {
        if self.data.len() < period {
            return None;
        }

        let sma = self.simple_moving_average(period, PriceType::Close)?;

        let sorted_data = self.sorted_by_time();
        let recent_data = &sorted_data[sorted_data.len() - period..];

        let variance: f64 = recent_data
            .iter()
            .map(|k| {
                let diff = k.close - sma;
                diff * diff
            })
            .sum::<f64>()
            / period as f64;

        let std_deviation = variance.sqrt();
        let upper_band = sma + (std_deviation * std_dev);
        let lower_band = sma - (std_deviation * std_dev);

        Some((upper_band, sma, lower_band))
    }

    /// Returns the support and resistance levels based on recent highs and lows.
    pub fn support_resistance_levels(&self, lookback_period: usize) -> Option<(f64, f64)> {
        if self.data.len() < lookback_period {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let recent_data = &sorted_data[sorted_data.len() - lookback_period..];

        let resistance = recent_data.iter().map(|k| k.high).reduce(f64::max)?;

        let support = recent_data.iter().map(|k| k.low).reduce(f64::min)?;

        Some((support, resistance))
    }

    /// Returns the volume profile for the k-lines.
    /// Groups prices into bins and calculates volume at each price level.
    pub fn volume_profile(&self, bins: usize) -> Option<Vec<(f64, f64)>> {
        if self.data.is_empty() {
            return None;
        }

        let min_price = self.min_price()?;
        let max_price = self.max_price()?;
        let price_range = max_price - min_price;

        if price_range <= 0.0 {
            return None;
        }

        let bin_size = price_range / bins as f64;
        let mut volume_profile = vec![0.0; bins];

        for kline in &self.data {
            // Distribute volume across price range of the k-line
            let kline_price_range = kline.high - kline.low;
            if kline_price_range > 0.0 {
                let volume_per_price = kline.volume / kline_price_range;

                // For simplicity, we'll just add volume to the bin containing the close price
                let price_bin = ((kline.close - min_price) / bin_size).floor() as usize;
                if price_bin < bins {
                    volume_profile[price_bin] += volume_per_price * bin_size;
                }
            }
        }

        // Convert to vector of (price, volume) pairs
        let result: Vec<(f64, f64)> = volume_profile
            .iter()
            .enumerate()
            .map(|(i, &volume)| (min_price + (i as f64 + 0.5) * bin_size, volume))
            .collect();

        Some(result)
    }

    /// Returns the price momentum over the specified number of periods.
    pub fn momentum(&self, periods: usize) -> Option<f64> {
        if self.data.len() < periods + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let current = &sorted_data[sorted_data.len() - 1];
        let previous = &sorted_data[sorted_data.len() - periods - 1];

        Some(current.close - previous.close)
    }

    /// Returns the rate of change (ROC) over the specified number of periods.
    pub fn rate_of_change(&self, periods: usize) -> Option<f64> {
        if self.data.len() < periods + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let current = &sorted_data[sorted_data.len() - 1];
        let previous = &sorted_data[sorted_data.len() - periods - 1];

        if previous.close > 0.0 {
            Some((current.close - previous.close) / previous.close * 100.0)
        } else {
            None
        }
    }

    /// Returns the stochastic oscillator values (%K and %D).
    pub fn stochastic_oscillator(&self, k_period: usize, d_period: usize) -> Option<(f64, f64)> {
        if self.data.len() < k_period + d_period {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let current = &sorted_data[sorted_data.len() - 1];

        // Find highest high and lowest low over K period
        let lookback_data = &sorted_data[sorted_data.len() - k_period..];
        let highest_high = lookback_data.iter().map(|k| k.high).reduce(f64::max)?;
        let lowest_low = lookback_data.iter().map(|k| k.low).reduce(f64::min)?;

        if highest_high == lowest_low {
            return Some((50.0, 50.0)); // Neutral position
        }

        let percent_k = ((current.close - lowest_low) / (highest_high - lowest_low)) * 100.0;

        // Simple %D as SMA of %K (simplified)
        let percent_d = percent_k * 0.9; // Simplified calculation

        Some((percent_k, percent_d))
    }

    /// Returns the average directional index (ADX) for trend strength.
    /// Simplified implementation - returns a value between 0 and 100.
    pub fn average_directional_index(&self, period: usize) -> Option<f64> {
        if self.data.len() < period * 2 {
            return None;
        }

        // Simplified ADX calculation
        let sorted_data = self.sorted_by_time();
        let mut plus_dm_sum = 0.0;
        let mut minus_dm_sum = 0.0;
        let mut tr_sum = 0.0;

        for i in 1..=period {
            let current = &sorted_data[sorted_data.len() - i];
            let previous = &sorted_data[sorted_data.len() - i - 1];

            let plus_dm = current.high - previous.high;
            let minus_dm = previous.low - current.low;

            if plus_dm > minus_dm && plus_dm > 0.0 {
                plus_dm_sum += plus_dm;
            } else if minus_dm > plus_dm && minus_dm > 0.0 {
                minus_dm_sum += minus_dm;
            }

            let tr = (current.high - current.low)
                .max((current.high - previous.close).abs())
                .max((current.low - previous.close).abs());
            tr_sum += tr;
        }

        let plus_di = if tr_sum > 0.0 {
            (plus_dm_sum / tr_sum) * 100.0
        } else {
            0.0
        };
        let minus_di = if tr_sum > 0.0 {
            (minus_dm_sum / tr_sum) * 100.0
        } else {
            0.0
        };

        let dx = if plus_di + minus_di > 0.0 {
            ((plus_di - minus_di).abs() / (plus_di + minus_di)) * 100.0
        } else {
            0.0
        };

        Some(dx)
    }

    /// Returns the on-balance volume (OBV) for the latest k-line.
    pub fn on_balance_volume(&self) -> Option<f64> {
        if self.data.len() < 2 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut obv = 0.0;

        for i in 1..sorted_data.len() {
            let current = &sorted_data[i];
            let previous = &sorted_data[i - 1];

            if current.close > previous.close {
                obv += current.volume;
            } else if current.close < previous.close {
                obv -= current.volume;
            }
            // If close is equal, volume is ignored
        }

        Some(obv)
    }

    /// Returns the accumulation/distribution line (A/D Line) for the latest k-line.
    pub fn accumulation_distribution_line(&self) -> Option<f64> {
        if self.data.is_empty() {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut ad_line = 0.0;

        for kline in sorted_data {
            let money_flow_multiplier = if kline.high != kline.low {
                ((kline.close - kline.low) - (kline.high - kline.close)) / (kline.high - kline.low)
            } else {
                0.0
            };

            let money_flow_volume = money_flow_multiplier * kline.volume;
            ad_line += money_flow_volume;
        }

        Some(ad_line)
    }

    /// Returns the commodity channel index (CCI) for the latest k-line.
    pub fn commodity_channel_index(&self, period: usize) -> Option<f64> {
        if self.data.len() < period {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let recent_data = &sorted_data[sorted_data.len() - period..];

        let typical_prices: Vec<f64> = recent_data
            .iter()
            .map(|k| (k.high + k.low + k.close) / 3.0)
            .collect();

        let sma = typical_prices.iter().sum::<f64>() / period as f64;

        let mean_deviation: f64 = typical_prices
            .iter()
            .map(|&tp| (tp - sma).abs())
            .sum::<f64>()
            / period as f64;

        if mean_deviation == 0.0 {
            return Some(0.0);
        }

        let current_tp = typical_prices.last()?;
        let cci = (current_tp - sma) / (0.015 * mean_deviation);

        Some(cci)
    }

    /// Returns the parabolic SAR (Stop and Reverse) for the latest k-line.
    /// Simplified implementation for educational purposes.
    pub fn parabolic_sar(&self, acceleration: f64, maximum: f64) -> Option<f64> {
        if self.data.len() < 3 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let current = sorted_data.last()?;
        let previous = &sorted_data[sorted_data.len() - 2];
        let pre_previous = &sorted_data[sorted_data.len() - 3];

        // Determine trend direction
        let is_uptrend = current.close > previous.close && previous.close > pre_previous.close;

        if is_uptrend {
            // In uptrend, SAR is below price
            let lowest_low = sorted_data[sorted_data.len() - 3..]
                .iter()
                .map(|k| k.low)
                .reduce(f64::min)?;

            Some(lowest_low * (1.0 - acceleration.min(maximum)))
        } else {
            // In downtrend, SAR is above price
            let highest_high = sorted_data[sorted_data.len() - 3..]
                .iter()
                .map(|k| k.high)
                .reduce(f64::max)?;

            Some(highest_high * (1.0 + acceleration.min(maximum)))
        }
    }

    /// Returns the Ichimoku Cloud components for the latest k-line.
    /// Returns (Tenkan-sen, Kijun-sen, Senkou Span A, Senkou Span B, Chikou Span)
    pub fn ichimoku_cloud(&self) -> Option<(f64, f64, f64, f64, f64)> {
        if self.data.len() < 52 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let current = sorted_data.last()?;

        // Tenkan-sen (Conversion Line): (9-period high + 9-period low) / 2
        let tenkan_sen = if sorted_data.len() >= 9 {
            let nine_period_data = &sorted_data[sorted_data.len() - 9..];
            let nine_high = nine_period_data.iter().map(|k| k.high).reduce(f64::max)?;
            let nine_low = nine_period_data.iter().map(|k| k.low).reduce(f64::min)?;
            (nine_high + nine_low) / 2.0
        } else {
            return None;
        };

        // Kijun-sen (Base Line): (26-period high + 26-period low) / 2
        let kijun_sen = if sorted_data.len() >= 26 {
            let twenty_six_period_data = &sorted_data[sorted_data.len() - 26..];
            let twenty_six_high = twenty_six_period_data
                .iter()
                .map(|k| k.high)
                .reduce(f64::max)?;
            let twenty_six_low = twenty_six_period_data
                .iter()
                .map(|k| k.low)
                .reduce(f64::min)?;
            (twenty_six_high + twenty_six_low) / 2.0
        } else {
            return None;
        };

        // Senkou Span A (Leading Span A): (Tenkan-sen + Kijun-sen) / 2, shifted 26 periods forward
        let senkou_span_a = (tenkan_sen + kijun_sen) / 2.0;

        // Senkou Span B (Leading Span B): (52-period high + 52-period low) / 2, shifted 26 periods forward
        let senkou_span_b = if sorted_data.len() >= 52 {
            let fifty_two_period_data = &sorted_data[sorted_data.len() - 52..];
            let fifty_two_high = fifty_two_period_data
                .iter()
                .map(|k| k.high)
                .reduce(f64::max)?;
            let fifty_two_low = fifty_two_period_data
                .iter()
                .map(|k| k.low)
                .reduce(f64::min)?;
            (fifty_two_high + fifty_two_low) / 2.0
        } else {
            return None;
        };

        // Chikou Span (Lagging Span): Current close price shifted 26 periods back
        let chikou_span = current.close;

        Some((
            tenkan_sen,
            kijun_sen,
            senkou_span_a,
            senkou_span_b,
            chikou_span,
        ))
    }

    /// Returns the pivot points for the latest k-line.
    /// Returns (Pivot, R1, R2, R3, S1, S2, S3)
    pub fn pivot_points(&self) -> Option<(f64, f64, f64, f64, f64, f64, f64)> {
        if self.data.is_empty() {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let latest = sorted_data.last()?;

        // For daily pivot points, we need previous day's data
        // Since we don't have that context, we'll use the latest k-line
        let pivot = (latest.high + latest.low + latest.close) / 3.0;

        let r1 = (2.0 * pivot) - latest.low;
        let s1 = (2.0 * pivot) - latest.high;

        let r2 = pivot + (latest.high - latest.low);
        let s2 = pivot - (latest.high - latest.low);

        let r3 = latest.high + 2.0 * (pivot - latest.low);
        let s3 = latest.low - 2.0 * (latest.high - pivot);

        Some((pivot, r1, r2, r3, s1, s2, s3))
    }

    /// Returns the Fibonacci retracement levels for the latest price swing.
    /// Returns (0.236, 0.382, 0.5, 0.618, 0.786)
    pub fn fibonacci_retracement(
        &self,
        lookback_period: usize,
    ) -> Option<(f64, f64, f64, f64, f64)> {
        if self.data.len() < lookback_period {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let recent_data = &sorted_data[sorted_data.len() - lookback_period..];

        let swing_high = recent_data.iter().map(|k| k.high).reduce(f64::max)?;
        let swing_low = recent_data.iter().map(|k| k.low).reduce(f64::min)?;
        let swing_range = swing_high - swing_low;

        let fib_236 = swing_high - (swing_range * 0.236);
        let fib_382 = swing_high - (swing_range * 0.382);
        let fib_500 = swing_high - (swing_range * 0.5);
        let fib_618 = swing_high - (swing_range * 0.618);
        let fib_786 = swing_high - (swing_range * 0.786);

        Some((fib_236, fib_382, fib_500, fib_618, fib_786))
    }

    /// Returns the volume ratio (current volume / average volume).
    pub fn volume_ratio(&self, _period: usize) -> Option<f64> {
        let current_volume = self.latest_volume()?;
        let average_volume = self.average_volume()?;

        if average_volume > 0.0 {
            Some(current_volume / average_volume)
        } else {
            None
        }
    }

    /// Returns the price volatility (standard deviation of returns).
    pub fn price_volatility(&self, period: usize) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut returns = Vec::with_capacity(period);

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];

            if previous.close > 0.0 {
                let return_val = (current.close - previous.close) / previous.close;
                returns.push(return_val);
            }
        }

        if returns.is_empty() {
            return None;
        }

        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns
            .iter()
            .map(|r| {
                let diff = r - mean;
                diff * diff
            })
            .sum::<f64>()
            / returns.len() as f64;

        Some(variance.sqrt() * 100.0) // Return as percentage
    }

    /// Returns the efficiency ratio (price change / volatility).
    pub fn efficiency_ratio(&self, period: usize) -> Option<f64> {
        let price_change = self.total_price_change_percentage()?;
        let volatility = self.price_volatility(period)?;

        if volatility > 0.0 {
            Some(price_change.abs() / volatility)
        } else {
            None
        }
    }

    /// Returns the Sharpe ratio (simplified for price returns).
    pub fn sharpe_ratio(&self, period: usize, risk_free_rate: f64) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut returns = Vec::with_capacity(period);

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];

            if previous.close > 0.0 {
                let return_val = (current.close - previous.close) / previous.close;
                returns.push(return_val * 100.0); // Convert to percentage
            }
        }

        if returns.is_empty() {
            return None;
        }

        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let excess_return = mean_return - risk_free_rate;

        let variance = returns
            .iter()
            .map(|r| {
                let diff = r - mean_return;
                diff * diff
            })
            .sum::<f64>()
            / returns.len() as f64;

        let std_dev = variance.sqrt();

        if std_dev > 0.0 {
            Some(excess_return / std_dev)
        } else {
            None
        }
    }

    /// Returns the maximum drawdown over the specified period.
    pub fn maximum_drawdown(&self, period: usize) -> Option<f64> {
        if self.data.len() < period {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let recent_data = &sorted_data[sorted_data.len() - period..];

        let mut peak = f64::MIN;
        let mut max_drawdown = 0.0;

        for kline in recent_data {
            if kline.close > peak {
                peak = kline.close;
            }

            let drawdown = (peak - kline.close) / peak * 100.0;
            if drawdown > max_drawdown {
                max_drawdown = drawdown;
            }
        }

        Some(max_drawdown)
    }

    /// Returns the calmar ratio (return / maximum drawdown).
    pub fn calmar_ratio(&self, period: usize) -> Option<f64> {
        let total_return = self.total_price_change_percentage()?;
        let max_drawdown = self.maximum_drawdown(period)?;

        if max_drawdown > 0.0 {
            Some(total_return / max_drawdown)
        } else {
            None
        }
    }

    /// Returns the sortino ratio (return / downside deviation).
    pub fn sortino_ratio(&self, period: usize, risk_free_rate: f64) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut returns = Vec::with_capacity(period);
        let mut downside_returns = Vec::new();

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];

            if previous.close > 0.0 {
                let return_val = (current.close - previous.close) / previous.close * 100.0;
                returns.push(return_val);

                if return_val < risk_free_rate {
                    downside_returns.push(return_val - risk_free_rate);
                }
            }
        }

        if returns.is_empty() {
            return None;
        }

        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let excess_return = mean_return - risk_free_rate;

        if downside_returns.is_empty() {
            return Some(excess_return); // No downside risk
        }

        let downside_variance =
            downside_returns.iter().map(|r| r * r).sum::<f64>() / downside_returns.len() as f64;

        let downside_deviation = downside_variance.sqrt();

        if downside_deviation > 0.0 {
            Some(excess_return / downside_deviation)
        } else {
            None
        }
    }

    /// Returns the information ratio (active return / tracking error).
    /// Simplified implementation using benchmark return of 0.
    pub fn information_ratio(&self, period: usize) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut active_returns = Vec::with_capacity(period);

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];

            if previous.close > 0.0 {
                let return_val = (current.close - previous.close) / previous.close * 100.0;
                active_returns.push(return_val); // Benchmark return is 0
            }
        }

        if active_returns.is_empty() {
            return None;
        }

        let mean_active_return = active_returns.iter().sum::<f64>() / active_returns.len() as f64;

        let tracking_error = active_returns
            .iter()
            .map(|r| {
                let diff = r - mean_active_return;
                diff * diff
            })
            .sum::<f64>()
            / active_returns.len() as f64;

        let tracking_error_sqrt = tracking_error.sqrt();

        if tracking_error_sqrt > 0.0 {
            Some(mean_active_return / tracking_error_sqrt)
        } else {
            None
        }
    }

    /// Returns the Ulcer Index (measure of downside risk).
    pub fn ulcer_index(&self, period: usize) -> Option<f64> {
        if self.data.len() < period {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let recent_data = &sorted_data[sorted_data.len() - period..];

        let mut peak = f64::MIN;
        let mut sum_squared_drawdowns = 0.0;

        for kline in recent_data {
            if kline.close > peak {
                peak = kline.close;
            }

            if peak > 0.0 {
                let drawdown = (peak - kline.close) / peak * 100.0;
                sum_squared_drawdowns += drawdown * drawdown;
            }
        }

        Some((sum_squared_drawdowns / period as f64).sqrt())
    }

    /// Returns the Martin ratio (return / Ulcer Index).
    pub fn martin_ratio(&self, period: usize) -> Option<f64> {
        let total_return = self.total_price_change_percentage()?;
        let ulcer_index = self.ulcer_index(period)?;

        if ulcer_index > 0.0 {
            Some(total_return / ulcer_index)
        } else {
            None
        }
    }

    /// Returns the Treynor ratio (return / beta).
    /// Simplified implementation with beta assumed to be 1.
    pub fn treynor_ratio(&self, period: usize, risk_free_rate: f64) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut returns = Vec::with_capacity(period);

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];

            if previous.close > 0.0 {
                let return_val = (current.close - previous.close) / previous.close * 100.0;
                returns.push(return_val);
            }
        }

        if returns.is_empty() {
            return None;
        }

        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let excess_return = mean_return - risk_free_rate;

        // Simplified: beta = 1
        let beta = 1.0;

        Some(excess_return / beta)
    }

    /// Returns the omega ratio (probability-weighted ratio of gains to losses).
    pub fn omega_ratio(&self, period: usize, threshold: f64) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut excess_returns = Vec::with_capacity(period);

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];

            if previous.close > 0.0 {
                let return_val = (current.close - previous.close) / previous.close * 100.0;
                excess_returns.push(return_val - threshold);
            }
        }

        if excess_returns.is_empty() {
            return None;
        }

        let gains: f64 = excess_returns
            .iter()
            .filter(|&&r| r > 0.0)
            .map(|&r| r)
            .sum();

        let losses: f64 = excess_returns
            .iter()
            .filter(|&&r| r < 0.0)
            .map(|&r| r.abs())
            .sum();

        if losses > 0.0 {
            Some(gains / losses)
        } else {
            None // Infinite ratio
        }
    }

    /// Returns the gain to pain ratio (total gains / total losses).
    pub fn gain_to_pain_ratio(&self, period: usize) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut gains = 0.0;
        let mut losses = 0.0;

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];

            if previous.close > 0.0 {
                let return_val = (current.close - previous.close) / previous.close * 100.0;
                if return_val > 0.0 {
                    gains += return_val;
                } else {
                    losses += return_val.abs();
                }
            }
        }

        if losses > 0.0 {
            Some(gains / losses)
        } else if gains > 0.0 {
            Some(f64::INFINITY) // No losses, infinite ratio
        } else {
            Some(0.0) // No gains or losses
        }
    }

    /// Returns the pain index (average drawdown during losing periods).
    pub fn pain_index(&self, period: usize) -> Option<f64> {
        if self.data.len() < period {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let recent_data = &sorted_data[sorted_data.len() - period..];

        let mut peak = f64::MIN;
        let mut total_drawdown = 0.0;
        let mut drawdown_count = 0;

        for kline in recent_data {
            if kline.close > peak {
                peak = kline.close;
            }

            if peak > 0.0 {
                let drawdown = (peak - kline.close) / peak * 100.0;
                if drawdown > 0.0 {
                    total_drawdown += drawdown;
                    drawdown_count += 1;
                }
            }
        }

        if drawdown_count > 0 {
            Some(total_drawdown / drawdown_count as f64)
        } else {
            Some(0.0)
        }
    }

    /// Returns the recovery factor (total return / maximum drawdown).
    pub fn recovery_factor(&self, period: usize) -> Option<f64> {
        let total_return = self.total_price_change_percentage()?;
        let max_drawdown = self.maximum_drawdown(period)?;

        if max_drawdown > 0.0 {
            Some(total_return / max_drawdown)
        } else if total_return > 0.0 {
            Some(f64::INFINITY) // No drawdown, infinite recovery
        } else {
            Some(0.0)
        }
    }

    /// Returns the risk-adjusted return (return / risk metric).
    pub fn risk_adjusted_return(&self, period: usize, risk_metric: RiskMetric) -> Option<f64> {
        match risk_metric {
            RiskMetric::Volatility => {
                let return_val = self.total_price_change_percentage()?;
                let volatility = self.price_volatility(period)?;
                if volatility > 0.0 {
                    Some(return_val / volatility)
                } else {
                    None
                }
            }
            RiskMetric::MaximumDrawdown => {
                let return_val = self.total_price_change_percentage()?;
                let max_drawdown = self.maximum_drawdown(period)?;
                if max_drawdown > 0.0 {
                    Some(return_val / max_drawdown)
                } else if return_val > 0.0 {
                    Some(f64::INFINITY)
                } else {
                    Some(0.0)
                }
            }
            RiskMetric::UlcerIndex => {
                let return_val = self.total_price_change_percentage()?;
                let ulcer_index = self.ulcer_index(period)?;
                if ulcer_index > 0.0 {
                    Some(return_val / ulcer_index)
                } else if return_val > 0.0 {
                    Some(f64::INFINITY)
                } else {
                    Some(0.0)
                }
            }
            RiskMetric::ValueAtRisk => {
                // Simplified VaR calculation (95% confidence)
                let return_val = self.total_price_change_percentage()?;
                let volatility = self.price_volatility(period)?;
                let var = volatility * 1.645; // 95% confidence Z-score
                if var > 0.0 {
                    Some(return_val / var)
                } else {
                    None
                }
            }
        }
    }

    /// Returns the statistical significance of a trend.
    pub fn trend_significance(&self, period: usize) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut returns = Vec::with_capacity(period);

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];

            if previous.close > 0.0 {
                let return_val = (current.close - previous.close) / previous.close;
                returns.push(return_val);
            }
        }

        if returns.is_empty() {
            return None;
        }

        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let std_dev = returns
            .iter()
            .map(|r| {
                let diff = r - mean;
                diff * diff
            })
            .sum::<f64>()
            / returns.len() as f64;

        let std_dev_sqrt = std_dev.sqrt();
        if std_dev_sqrt > 0.0 {
            let t_statistic = mean / (std_dev_sqrt / (returns.len() as f64).sqrt());
            Some(t_statistic.abs())
        } else {
            None
        }
    }

    /// Returns the probability of a positive return.
    pub fn probability_positive_return(&self, period: usize) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut positive_count = 0;
        let mut total_count = 0;

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];

            if previous.close > 0.0 {
                total_count += 1;
                if current.close > previous.close {
                    positive_count += 1;
                }
            }
        }

        if total_count > 0 {
            Some(positive_count as f64 / total_count as f64 * 100.0)
        } else {
            None
        }
    }

    /// Returns the expected shortfall (conditional VaR).
    pub fn expected_shortfall(&self, period: usize, confidence_level: f64) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut returns = Vec::with_capacity(period);

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];

            if previous.close > 0.0 {
                let return_val = (current.close - previous.close) / previous.close * 100.0;
                returns.push(return_val);
            }
        }

        if returns.is_empty() {
            return None;
        }

        returns.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let cutoff_index = ((1.0 - confidence_level) * returns.len() as f64).floor() as usize;

        if cutoff_index > 0 {
            let worst_returns = &returns[..cutoff_index];
            let average_worst_return =
                worst_returns.iter().sum::<f64>() / worst_returns.len() as f64;
            Some(average_worst_return)
        } else {
            None
        }
    }

    /// Returns the tail ratio (average of best returns / average of worst returns).
    pub fn tail_ratio(&self, period: usize, tail_percentage: f64) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut returns = Vec::with_capacity(period);

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];

            if previous.close > 0.0 {
                let return_val = (current.close - previous.close) / previous.close * 100.0;
                returns.push(return_val);
            }
        }

        if returns.is_empty() {
            return None;
        }

        returns.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let tail_count = (returns.len() as f64 * tail_percentage).floor() as usize;

        if tail_count > 0 {
            let best_returns = &returns[returns.len() - tail_count..];
            let worst_returns = &returns[..tail_count];

            let average_best = best_returns.iter().sum::<f64>() / best_returns.len() as f64;
            let average_worst = worst_returns.iter().sum::<f64>() / worst_returns.len() as f64;

            if average_worst.abs() > 0.0 {
                Some(average_best / average_worst.abs())
            } else if average_best > 0.0 {
                Some(f64::INFINITY)
            } else {
                Some(0.0)
            }
        } else {
            None
        }
    }

    /// Returns the information coefficient (correlation between predicted and actual returns).
    /// Simplified implementation using trend as prediction.
    pub fn information_coefficient(&self, period: usize) -> Option<f64> {
        if self.data.len() < period * 2 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut predictions = Vec::with_capacity(period);
        let mut actuals = Vec::with_capacity(period);

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];
            let older = &sorted_data[sorted_data.len() - i - 3];

            if previous.close > 0.0 && older.close > 0.0 {
                // Prediction: trend continues
                let previous_return = (previous.close - older.close) / older.close * 100.0;
                predictions.push(previous_return);

                // Actual return
                let actual_return = (current.close - previous.close) / previous.close * 100.0;
                actuals.push(actual_return);
            }
        }

        if predictions.is_empty() || actuals.is_empty() {
            return None;
        }

        // Calculate correlation
        let pred_mean = predictions.iter().sum::<f64>() / predictions.len() as f64;
        let actual_mean = actuals.iter().sum::<f64>() / actuals.len() as f64;

        let mut covariance = 0.0;
        let mut pred_variance = 0.0;
        let mut actual_variance = 0.0;

        for i in 0..predictions.len() {
            let pred_diff = predictions[i] - pred_mean;
            let actual_diff = actuals[i] - actual_mean;

            covariance += pred_diff * actual_diff;
            pred_variance += pred_diff * pred_diff;
            actual_variance += actual_diff * actual_diff;
        }

        if pred_variance > 0.0 && actual_variance > 0.0 {
            let correlation = covariance / (pred_variance.sqrt() * actual_variance.sqrt());
            Some(correlation)
        } else {
            None
        }
    }

    /// Returns the batting average (percentage of periods with positive returns).
    pub fn batting_average(&self, period: usize) -> Option<f64> {
        self.probability_positive_return(period)
    }

    /// Returns the win/loss ratio (average gain / average loss).
    pub fn win_loss_ratio(&self, period: usize) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut gains = Vec::new();
        let mut losses = Vec::new();

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];

            if previous.close > 0.0 {
                let return_val = (current.close - previous.close) / previous.close * 100.0;
                if return_val > 0.0 {
                    gains.push(return_val);
                } else if return_val < 0.0 {
                    losses.push(return_val.abs());
                }
            }
        }

        if gains.is_empty() || losses.is_empty() {
            return None;
        }

        let avg_gain = gains.iter().sum::<f64>() / gains.len() as f64;
        let avg_loss = losses.iter().sum::<f64>() / losses.len() as f64;

        if avg_loss > 0.0 {
            Some(avg_gain / avg_loss)
        } else {
            None
        }
    }

    /// Returns the profit factor (total gains / total losses).
    pub fn profit_factor(&self, period: usize) -> Option<f64> {
        self.gain_to_pain_ratio(period)
    }

    /// Returns the k-ratio (slope of equity curve / standard error).
    pub fn k_ratio(&self, period: usize) -> Option<f64> {
        if self.data.len() < period {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let recent_data = &sorted_data[sorted_data.len() - period..];

        // Create equity curve (cumulative returns)
        let mut equity_curve = Vec::with_capacity(period);
        let mut _cumulative_return = 0.0;

        for kline in recent_data {
            if let Some(previous) = equity_curve.last() {
                let return_val = (kline.close - previous) / previous * 100.0;
                _cumulative_return += return_val;
            }
            equity_curve.push(kline.close);
        }

        if equity_curve.len() < 2 {
            return None;
        }

        // Linear regression on equity curve
        let n = equity_curve.len() as f64;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_x2 = 0.0;

        for (i, &price) in equity_curve.iter().enumerate() {
            let x = i as f64;
            let y = price;
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_x2 += x * x;
        }

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;

        // Calculate standard error
        let mut sum_squared_errors = 0.0;
        for (i, &price) in equity_curve.iter().enumerate() {
            let x = i as f64;
            let predicted = slope * x + intercept;
            let error = price - predicted;
            sum_squared_errors += error * error;
        }

        let standard_error = (sum_squared_errors / (n - 2.0)).sqrt();

        if standard_error > 0.0 {
            Some(slope / standard_error)
        } else {
            None
        }
    }

    /// Returns the stability of returns (inverse of volatility).
    pub fn return_stability(&self, period: usize) -> Option<f64> {
        let volatility = self.price_volatility(period)?;
        if volatility > 0.0 {
            Some(1.0 / volatility)
        } else {
            None
        }
    }

    /// Returns the consistency ratio (periods with same direction / total periods).
    pub fn consistency_ratio(&self, period: usize) -> Option<f64> {
        if self.data.len() < period + 1 {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let mut same_direction_count = 0;
        let mut total_comparisons = 0;

        for i in 0..period {
            let current = &sorted_data[sorted_data.len() - i - 1];
            let previous = &sorted_data[sorted_data.len() - i - 2];
            let older = &sorted_data[sorted_data.len() - i - 3];

            if previous.close > 0.0 && older.close > 0.0 {
                total_comparisons += 1;
                let previous_trend = current.close > previous.close;
                let older_trend = previous.close > older.close;

                if previous_trend == older_trend {
                    same_direction_count += 1;
                }
            }
        }

        if total_comparisons > 0 {
            Some(same_direction_count as f64 / total_comparisons as f64 * 100.0)
        } else {
            None
        }
    }

    /// Returns the R-squared value for the equity curve.
    pub fn r_squared(&self, period: usize) -> Option<f64> {
        if self.data.len() < period {
            return None;
        }

        let sorted_data = self.sorted_by_time();
        let recent_data = &sorted_data[sorted_data.len() - period..];

        // Create equity curve (cumulative returns)
        let mut equity_curve = Vec::with_capacity(period);
        let mut _cumulative_return = 0.0;

        for kline in recent_data {
            if let Some(previous) = equity_curve.last() {
                let return_val = (kline.close - previous) / previous * 100.0;
                _cumulative_return += return_val;
            }
            equity_curve.push(kline.close);
        }

        if equity_curve.len() < 2 {
            return None;
        }

        // Linear regression
        let n = equity_curve.len() as f64;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_x2 = 0.0;

        for (i, &price) in equity_curve.iter().enumerate() {
            let x = i as f64;
            let y = price;
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_x2 += x * x;
        }

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;

        // Calculate R-squared
        let y_mean = sum_y / n;
        let mut ss_total = 0.0;
        let mut ss_residual = 0.0;

        for (i, &price) in equity_curve.iter().enumerate() {
            let x = i as f64;
            let predicted = slope * x + intercept;
            ss_total += (price - y_mean) * (price - y_mean);
            ss_residual += (price - predicted) * (price - predicted);
        }

        if ss_total > 0.0 {
            Some(1.0 - (ss_residual / ss_total))
        } else {
            Some(0.0)
        }
    }

    /// Returns the predictive power score (simplified).
    pub fn predictive_power_score(&self, period: usize) -> Option<f64> {
        // Combine multiple metrics for a comprehensive score
        let mut score = 0.0;
        let mut count = 0;

        if let Some(r_squared) = self.r_squared(period) {
            score += r_squared;
            count += 1;
        }

        if let Some(correlation) = self.information_coefficient(period) {
            score += correlation.abs();
            count += 1;
        }

        if let Some(trend_sig) = self.trend_significance(period) {
            score += trend_sig.min(3.0) / 3.0; // Normalize to 0-1
            count += 1;
        }

        if let Some(consistency) = self.consistency_ratio(period) {
            score += consistency / 100.0; // Normalize from percentage
            count += 1;
        }

        if count > 0 {
            Some(score / count as f64 * 100.0) // Return as percentage
        } else {
            None
        }
    }

    /// Returns a comprehensive analysis report for the k-line data.
    pub fn analysis_report(&self, period: usize) -> String {
        let mut report = String::new();

        // Basic information
        report.push_str(&format!("K-line Analysis Report\n"));
        report.push_str(&format!("=====================\n"));
        report.push_str(&format!("Symbol: {}\n", self.symbol().unwrap_or("Unknown")));
        report.push_str(&format!(
            "Interval: {}\n",
            self.interval().unwrap_or("Unknown")
        ));
        report.push_str(&format!("Data Points: {}\n", self.count()));
        report.push_str(&format!(
            "Timestamp: {}\n",
            self.timestamp_datetime().format("%Y-%m-%d %H:%M:%S")
        ));
        report.push_str(&format!("Age: {} ms\n", self.age_ms()));
        report.push_str(&format!("Stale: {}\n", self.is_stale()));
        report.push_str(&format!(
            "Valid for Trading: {}\n",
            self.is_valid_for_trading()
        ));
        report.push_str("\n");

        // Latest k-line information
        if let Some(latest) = self.latest() {
            report.push_str(&format!("Latest K-line:\n"));
            report.push_str(&format!("  Open: {:.8}\n", latest.open));
            report.push_str(&format!("  High: {:.8}\n", latest.high));
            report.push_str(&format!("  Low: {:.8}\n", latest.low));
            report.push_str(&format!("  Close: {:.8}\n", latest.close));
            report.push_str(&format!("  Volume: {:.8}\n", latest.volume));
            report.push_str(&format!("  Turnover: {:.8}\n", latest.turnover));
            report.push_str(&format!(
                "  Start: {}\n",
                chrono::DateTime::from_timestamp((latest.start / 1000) as i64, 0)
                    .unwrap_or_else(chrono::Utc::now)
                    .format("%Y-%m-%d %H:%M:%S")
            ));
            report.push_str(&format!("  Interval: {} ms\n", latest.interval));
            report.push_str(&format!("  Confirm: {}\n", latest.confirm));
            report.push_str("\n");
        }

        // Price analysis
        report.push_str(&format!("Price Analysis:\n"));
        if let Some(change) = self.latest_price_change() {
            report.push_str(&format!(
                "  Price Change: {:.8} ({})\n",
                change,
                if change > 0.0 {
                    "↑"
                } else if change < 0.0 {
                    "↓"
                } else {
                    "→"
                }
            ));
        }
        if let Some(change_pct) = self.latest_price_change_percentage() {
            report.push_str(&format!("  Price Change %: {:.4}%\n", change_pct));
        }
        if let Some(range) = self.latest_range() {
            report.push_str(&format!("  Range: {:.8}\n", range));
        }
        if let Some(body_ratio) = self.latest_body_to_range_ratio() {
            report.push_str(&format!("  Body/Range Ratio: {:.4}\n", body_ratio));
        }
        report.push_str(&format!("  Bullish: {}\n", self.is_latest_bullish()));
        report.push_str(&format!("  Bearish: {}\n", self.is_latest_bearish()));
        report.push_str(&format!("  Doji: {}\n", self.is_latest_doji()));
        report.push_str("\n");

        // Volume analysis
        report.push_str(&format!("Volume Analysis:\n"));
        if let Some(volume) = self.latest_volume() {
            report.push_str(&format!("  Volume: {:.8}\n", volume));
        }
        if let Some(turnover) = self.latest_turnover() {
            report.push_str(&format!("  Turnover: {:.8}\n", turnover));
        }
        if let Some(vwap) = self.latest_vwap() {
            report.push_str(&format!("  VWAP: {:.8}\n", vwap));
        }
        if let Some(avg_volume) = self.average_volume() {
            report.push_str(&format!("  Average Volume: {:.8}\n", avg_volume));
        }
        if let Some(volume_ratio) = self.volume_ratio(period) {
            report.push_str(&format!("  Volume Ratio: {:.4}\n", volume_ratio));
        }
        report.push_str("\n");

        // Technical indicators (if enough data)
        if self.data.len() >= 14 {
            report.push_str(&format!("Technical Indicators (14-period):\n"));
            if let Some(rsi) = self.relative_strength_index(14) {
                report.push_str(&format!("  RSI: {:.2}\n", rsi));
                report.push_str(&format!("    Overbought (>70): {}\n", rsi > 70.0));
                report.push_str(&format!("    Oversold (<30): {}\n", rsi < 30.0));
            }
            if let Some(atr) = self.average_true_range(14) {
                report.push_str(&format!("  ATR: {:.8}\n", atr));
            }
            if let Some((upper, middle, lower)) = self.bollinger_bands(20, 2.0) {
                if let Some(close) = self.latest_close() {
                    report.push_str(&format!("  Bollinger Bands:\n"));
                    report.push_str(&format!("    Upper: {:.8}\n", upper));
                    report.push_str(&format!("    Middle: {:.8}\n", middle));
                    report.push_str(&format!("    Lower: {:.8}\n", lower));
                    report.push_str(&format!(
                        "    Position: {:.2}%\n",
                        (close - lower) / (upper - lower) * 100.0
                    ));
                }
            }
            report.push_str("\n");
        }

        // Risk metrics
        report.push_str(&format!("Risk Metrics ({} periods):\n", period));
        if let Some(volatility) = self.price_volatility(period) {
            report.push_str(&format!("  Volatility: {:.4}%\n", volatility));
        }
        if let Some(max_dd) = self.maximum_drawdown(period) {
            report.push_str(&format!("  Max Drawdown: {:.4}%\n", max_dd));
        }
        if let Some(ulcer) = self.ulcer_index(period) {
            report.push_str(&format!("  Ulcer Index: {:.4}\n", ulcer));
        }
        if let Some(var) = self.expected_shortfall(period, 0.95) {
            report.push_str(&format!("  Expected Shortfall (95%): {:.4}%\n", var));
        }
        report.push_str("\n");

        // Performance metrics
        report.push_str(&format!("Performance Metrics ({} periods):\n", period));
        if let Some(total_return) = self.total_price_change_percentage() {
            report.push_str(&format!("  Total Return: {:.4}%\n", total_return));
        }
        if let Some(sharpe) = self.sharpe_ratio(period, 0.0) {
            report.push_str(&format!("  Sharpe Ratio: {:.4}\n", sharpe));
        }
        if let Some(sortino) = self.sortino_ratio(period, 0.0) {
            report.push_str(&format!("  Sortino Ratio: {:.4}\n", sortino));
        }
        if let Some(calmar) = self.calmar_ratio(period) {
            report.push_str(&format!("  Calmar Ratio: {:.4}\n", calmar));
        }
        if let Some(omega) = self.omega_ratio(period, 0.0) {
            report.push_str(&format!("  Omega Ratio: {:.4}\n", omega));
        }
        report.push_str("\n");

        // Trading statistics
        report.push_str(&format!("Trading Statistics ({} periods):\n", period));
        if let Some(win_rate) = self.probability_positive_return(period) {
            report.push_str(&format!("  Win Rate: {:.2}%\n", win_rate));
        }
        if let Some(win_loss) = self.win_loss_ratio(period) {
            report.push_str(&format!("  Win/Loss Ratio: {:.4}\n", win_loss));
        }
        if let Some(profit_factor) = self.profit_factor(period) {
            report.push_str(&format!("  Profit Factor: {:.4}\n", profit_factor));
        }
        if let Some(k_ratio) = self.k_ratio(period) {
            report.push_str(&format!("  K-Ratio: {:.4}\n", k_ratio));
        }
        if let Some(predictive_score) = self.predictive_power_score(period) {
            report.push_str(&format!(
                "  Predictive Power Score: {:.2}%\n",
                predictive_score
            ));
        }
        report.push_str("\n");

        // Summary
        report.push_str(&format!("Summary:\n"));
        let mut summary_score = 0.0;
        let mut summary_count = 0;

        // Trend strength
        if let Some(trend_sig) = self.trend_significance(period) {
            if trend_sig > 2.0 {
                report.push_str(&format!("  Strong trend detected (t={:.2})\n", trend_sig));
                summary_score += 0.3;
            } else if trend_sig > 1.0 {
                report.push_str(&format!("  Moderate trend detected (t={:.2})\n", trend_sig));
                summary_score += 0.2;
            } else {
                report.push_str(&format!("  Weak or no trend (t={:.2})\n", trend_sig));
                summary_score += 0.1;
            }
            summary_count += 1;
        }

        // Volatility assessment
        if let Some(volatility) = self.price_volatility(period) {
            if volatility > 5.0 {
                report.push_str(&format!("  High volatility: {:.2}%\n", volatility));
                summary_score += 0.2;
            } else if volatility > 2.0 {
                report.push_str(&format!("  Moderate volatility: {:.2}%\n", volatility));
                summary_score += 0.3;
            } else {
                report.push_str(&format!("  Low volatility: {:.2}%\n", volatility));
                summary_score += 0.4;
            }
            summary_count += 1;
        }

        // Risk assessment
        if let Some(max_dd) = self.maximum_drawdown(period) {
            if max_dd > 10.0 {
                report.push_str(&format!("  High risk (Max DD: {:.2}%)\n", max_dd));
                summary_score += 0.1;
            } else if max_dd > 5.0 {
                report.push_str(&format!("  Moderate risk (Max DD: {:.2}%)\n", max_dd));
                summary_score += 0.2;
            } else {
                report.push_str(&format!("  Low risk (Max DD: {:.2}%)\n", max_dd));
                summary_score += 0.3;
            }
            summary_count += 1;
        }

        // Overall score
        if summary_count > 0 {
            let overall_score = (summary_score / summary_count as f64) * 100.0;
            report.push_str(&format!("  Overall Score: {:.1}/100\n", overall_score));

            if overall_score >= 70.0 {
                report.push_str(&format!("  Recommendation: Favorable conditions\n"));
            } else if overall_score >= 40.0 {
                report.push_str(&format!("  Recommendation: Neutral conditions\n"));
            } else {
                report.push_str(&format!("  Recommendation: Unfavorable conditions\n"));
            }
        }

        report
    }

    /// Returns a JSON representation of the analysis report.
    pub fn analysis_report_json(&self, period: usize) -> serde_json::Value {
        let mut report = serde_json::json!({});

        // Basic information
        report["basic"] = serde_json::json!({
            "symbol": self.symbol().unwrap_or("Unknown").to_string(),
            "interval": self.interval().unwrap_or("Unknown").to_string(),
            "data_points": self.count(),
            "timestamp": self.timestamp,
            "timestamp_datetime": self.timestamp_datetime().to_rfc3339(),
            "age_ms": self.age_ms(),
            "stale": self.is_stale(),
            "valid_for_trading": self.is_valid_for_trading(),
        });

        // Latest k-line
        if let Some(latest) = self.latest() {
            report["latest"] = serde_json::json!({
                "open": latest.open,
                "high": latest.high,
                "low": latest.low,
                "close": latest.close,
                "volume": latest.volume,
                "turnover": latest.turnover,
                "start": latest.start,
                "interval": latest.interval,
                "confirm": latest.confirm,
            });
        }

        // Price analysis
        report["price_analysis"] = serde_json::json!({
            "price_change": self.latest_price_change(),
            "price_change_percentage": self.latest_price_change_percentage(),
            "range": self.latest_range(),
            "body_to_range_ratio": self.latest_body_to_range_ratio(),
            "bullish": self.is_latest_bullish(),
            "bearish": self.is_latest_bearish(),
            "doji": self.is_latest_doji(),
        });

        // Volume analysis
        report["volume_analysis"] = serde_json::json!({
            "volume": self.latest_volume(),
            "turnover": self.latest_turnover(),
            "vwap": self.latest_vwap(),
            "average_volume": self.average_volume(),
            "total_volume": self.total_volume(),
            "total_turnover": self.total_turnover(),
        });

        // Technical indicators (if enough data)
        if self.data.len() >= 14 {
            let mut indicators = serde_json::json!({});

            if let Some(rsi) = self.relative_strength_index(14) {
                indicators["rsi"] = serde_json::json!({
                    "value": rsi,
                    "overbought": rsi > 70.0,
                    "oversold": rsi < 30.0,
                });
            }

            if let Some(atr) = self.average_true_range(14) {
                indicators["atr"] = atr.into();
            }

            if let Some((upper, middle, lower)) = self.bollinger_bands(20, 2.0) {
                if let Some(close) = self.latest_close() {
                    indicators["bollinger_bands"] = serde_json::json!({
                        "upper": upper,
                        "middle": middle,
                        "lower": lower,
                        "position": (close - lower) / (upper - lower) * 100.0,
                    });
                }
            }

            report["technical_indicators"] = indicators;
        }

        // Risk metrics
        report["risk_metrics"] = serde_json::json!({
            "volatility": self.price_volatility(period),
            "maximum_drawdown": self.maximum_drawdown(period),
            "ulcer_index": self.ulcer_index(period),
            "expected_shortfall_95": self.expected_shortfall(period, 0.95),
        });

        // Performance metrics
        report["performance_metrics"] = serde_json::json!({
            "total_return": self.total_price_change_percentage(),
            "sharpe_ratio": self.sharpe_ratio(period, 0.0),
            "sortino_ratio": self.sortino_ratio(period, 0.0),
            "calmar_ratio": self.calmar_ratio(period),
            "omega_ratio": self.omega_ratio(period, 0.0),
        });

        // Trading statistics
        report["trading_statistics"] = serde_json::json!({
            "win_rate": self.probability_positive_return(period),
            "win_loss_ratio": self.win_loss_ratio(period),
            "profit_factor": self.profit_factor(period),
            "k_ratio": self.k_ratio(period),
            "predictive_power_score": self.predictive_power_score(period),
        });

        // Summary score
        let mut summary_score = 0.0;
        let mut summary_count = 0;

        if let Some(trend_sig) = self.trend_significance(period) {
            summary_score += trend_sig.min(3.0) / 3.0;
            summary_count += 1;
        }

        if let Some(volatility) = self.price_volatility(period) {
            if volatility > 5.0 {
                summary_score += 0.2;
            } else if volatility > 2.0 {
                summary_score += 0.3;
            } else {
                summary_score += 0.4;
            }
            summary_count += 1;
        }

        if let Some(max_dd) = self.maximum_drawdown(period) {
            if max_dd > 10.0 {
                summary_score += 0.1;
            } else if max_dd > 5.0 {
                summary_score += 0.2;
            } else {
                summary_score += 0.3;
            }
            summary_count += 1;
        }

        if summary_count > 0 {
            let overall_score = (summary_score / summary_count as f64) * 100.0;
            report["summary"] = serde_json::json!({
                "overall_score": overall_score,
                "recommendation": if overall_score >= 70.0 {
                    "Favorable conditions"
                } else if overall_score >= 40.0 {
                    "Neutral conditions"
                } else {
                    "Unfavorable conditions"
                },
            });
        }

        report
    }
}
