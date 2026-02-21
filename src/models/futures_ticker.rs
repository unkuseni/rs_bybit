use crate::prelude::*;
use chrono::{DateTime, Utc};

/// Represents ticker data for a futures contract (including perpetuals).
///
/// Contains real-time market data for a perpetual futures contract, such as prices, volumes, open interest, and funding rates. Bots use this for price monitoring, liquidity analysis, and funding cost estimation.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FuturesTicker {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract. Bots should verify this matches their target pair.
    pub symbol: String,

    /// The last traded price.
    ///
    /// The most recent price at which the contract was traded. Bots use this for real-time price tracking and as a reference for order placement.
    #[serde(with = "string_to_float")]
    pub last_price: f64,

    /// The index price.
    ///
    /// The spot market price of the underlying asset, aggregated from multiple exchanges. In perpetual futures, this anchors the mark price to prevent manipulation. Bots use this to predict mark price movements and funding rates.
    #[serde(with = "string_to_float")]
    pub index_price: f64,

    /// The mark price.
    ///
    /// A smoothed price used for funding rate calculations and liquidation triggers in perpetual futures. Bots must monitor this to avoid liquidations, as positions are closed if the mark price hits the liquidation price.
    #[serde(with = "string_to_float")]
    pub mark_price: f64,

    /// The price 24 hours ago.
    ///
    /// The price of the contract 24 hours prior. Bots use this to calculate daily price changes and assess market trends.
    #[serde(with = "string_to_float")]
    pub prev_price_24h: f64,

    /// The percentage price change over the last 24 hours.
    ///
    /// Calculated as `(last_price - prev_price_24h) / prev_price_24h`. Bots use this to identify trends and volatility in perpetual futures.
    #[serde(rename = "price24hPcnt", with = "string_to_float")]
    pub daily_change_percentage: f64,

    /// The highest price in the last 24 hours.
    ///
    /// Indicates the peak price. Bots use this to assess resistance levels and volatility.
    #[serde(rename = "highPrice24h", with = "string_to_float")]
    pub high_24h: f64,

    /// The lowest price in the last 24 hours.
    ///
    /// Indicates the trough price. Bots use this to assess support levels and volatility.
    #[serde(rename = "lowPrice24h", with = "string_to_float")]
    pub low_24h: f64,

    /// The price 1 hour ago.
    ///
    /// The price of the contract 1 hour prior. Bots use this for short-term trend analysis.
    #[serde(with = "string_to_float")]
    pub prev_price_1h: f64,

    /// The total open interest.
    ///
    /// The total number of open contracts (in base asset units, e.g., BTC). High open interest indicates strong market participation, which can affect liquidity and volatility in perpetual futures. Bots use this to gauge market sentiment.
    #[serde(with = "string_to_float")]
    pub open_interest: f64,

    /// The value of open interest in the quote asset.
    ///
    /// The total value of open contracts in the quote asset (e.g., USDT). Bots use this to assess the financial scale of market participation.
    #[serde(with = "string_to_float")]
    pub open_interest_value: f64,

    /// The trading turnover in the last 24 hours (in quote asset).
    ///
    /// The total value traded in the quote asset (e.g., USDT). Bots use this to assess market activity and liquidity.
    #[serde(with = "string_to_float")]
    pub turnover_24h: f64,

    /// The trading volume in the last 24 hours (in base asset).
    ///
    /// The total quantity traded in the base asset (e.g., BTC). Bots use this to confirm trade signals, as high volume often indicates strong trends.
    #[serde(with = "string_to_float")]
    pub volume_24h: f64,

    /// The current funding rate.
    ///
    /// The rate paid between long and short position holders every funding interval (typically 8 hours). Positive rates mean longs pay shorts; negative rates mean shorts pay longs. Bots must monitor this to estimate funding costs, which can significantly impact profitability for long-term positions in perpetual futures.
    pub funding_rate: String,

    /// The timestamp of the next funding settlement (Unix timestamp in milliseconds).
    ///
    /// Indicates when the next funding rate will be applied. Bots should use this to time position adjustments to minimize funding costs.
    #[serde(with = "string_to_u64")]
    pub next_funding_time: u64,

    /// The predicted delivery price (empty for perpetuals).
    ///
    /// For perpetual futures, this is typically empty since there’s no delivery. Bots can ignore this.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub predicted_delivery_price: String,

    /// The basis rate (empty for perpetuals).
    ///
    /// For perpetual futures, this is typically empty. Bots can ignore this.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub basis_rate: String,

    /// The delivery fee rate (empty for perpetuals).
    ///
    /// For perpetual futures, this is typically empty. Bots can ignore this.
    pub delivery_fee_rate: String,

    /// The delivery time (empty for perpetuals).
    ///
    /// For perpetual futures, this is typically empty. Bots can ignore this.
    #[serde(with = "string_to_u64")]
    pub delivery_time: u64,

    /// The size of the best ask order.
    ///
    /// The quantity available at the best ask price. Bots use this to assess immediate selling liquidity.
    #[serde(rename = "ask1Size", with = "string_to_float")]
    pub ask_size: f64,

    /// The best bid price.
    ///
    /// The highest price buyers are willing to pay. Bots use this to determine the current market bid price for sell orders.
    #[serde(rename = "bid1Price", with = "string_to_float")]
    pub bid_price: f64,

    /// The best ask price.
    ///
    /// The lowest price sellers are willing to accept. Bots use this to determine the current market ask price for buy orders.
    #[serde(rename = "ask1Price", with = "string_to_float")]
    pub ask_price: f64,

    /// The size of the best bid order.
    ///
    /// The quantity available at the best bid price. Bots use this to assess immediate buying liquidity.
    #[serde(rename = "bid1Size", with = "string_to_float")]
    pub bid_size: f64,

    /// The basis (empty for perpetuals).
    ///
    /// For perpetual futures, this is typically empty. Bots can ignore this.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub basis: String,

    /// Estimated pre-market contract open price.
    ///
    /// Meaningless once the market opens. Only for pre-market contracts.
    #[serde(rename = "preOpenPrice", skip_serializing_if = "String::is_empty")]
    pub pre_open_price: String,

    /// Estimated pre-market contract open quantity.
    ///
    /// The value is meaningless once the market opens. Only for pre-market contracts.
    #[serde(rename = "preQty", skip_serializing_if = "String::is_empty")]
    pub pre_qty: String,

    /// The current pre-market contract phase.
    ///
    /// Only for pre-market contracts.
    #[serde(
        rename = "curPreListingPhase",
        skip_serializing_if = "String::is_empty"
    )]
    pub cur_pre_listing_phase: String,

    /// Funding interval in hours.
    ///
    /// This value currently only supports whole hours.
    #[serde(
        rename = "fundingIntervalHour",
        skip_serializing_if = "String::is_empty"
    )]
    pub funding_interval_hour: String,

    /// Funding rate upper and lower limits.
    ///
    /// Maximum and minimum funding rates.
    #[serde(rename = "fundingCap", skip_serializing_if = "String::is_empty")]
    pub funding_cap: String,

    /// Annual basis rate.
    ///
    /// Only for Futures. For Perpetual, it will return empty string.
    #[serde(rename = "basisRateYear", skip_serializing_if = "String::is_empty")]
    pub basis_rate_year: String,
}

impl FuturesTicker {
    /// Returns the bid-ask spread.
    pub fn spread(&self) -> f64 {
        self.ask_price - self.bid_price
    }

    /// Returns the mid price (average of bid and ask).
    pub fn mid_price(&self) -> f64 {
        (self.bid_price + self.ask_price) / 2.0
    }

    /// Returns the bid-ask size ratio.
    pub fn bid_ask_size_ratio(&self) -> f64 {
        if self.ask_size == 0.0 {
            return 0.0;
        }
        self.bid_size / self.ask_size
    }

    /// Returns the total size at top of book.
    pub fn total_top_size(&self) -> f64 {
        self.bid_size + self.ask_size
    }

    /// Returns the price impact for a given quantity (in base asset).
    pub fn price_impact(&self, quantity: f64) -> f64 {
        if self.bid_size == 0.0 || self.ask_size == 0.0 {
            return 0.0;
        }
        // Simple linear impact model
        let impact_bid = quantity / self.bid_size;
        let impact_ask = quantity / self.ask_size;
        (impact_bid + impact_ask) / 2.0
    }

    /// Returns the funding rate as a floating-point number.
    pub fn funding_rate_f64(&self) -> Option<f64> {
        self.funding_rate.parse().ok()
    }

    /// Returns the next funding time as a DateTime.
    pub fn next_funding_time_datetime(&self) -> DateTime<Utc> {
        DateTime::from_timestamp((self.next_funding_time / 1000) as i64, 0)
            .unwrap_or_else(|| Utc::now())
    }

    /// Returns the time until next funding in seconds.
    pub fn time_to_next_funding(&self) -> i64 {
        let now = Utc::now().timestamp_millis() as u64;
        if self.next_funding_time > now {
            ((self.next_funding_time - now) / 1000) as i64
        } else {
            0
        }
    }

    /// Returns the annualized funding rate.
    pub fn annualized_funding_rate(&self) -> Option<f64> {
        let rate = self.funding_rate_f64()?;
        let intervals_per_year = 365.0 * 24.0 / self.funding_interval_hour.parse::<f64>().ok()?;
        Some(rate * intervals_per_year)
    }

    /// Returns the open interest value in USD.
    pub fn open_interest_usd(&self) -> f64 {
        self.open_interest_value
    }

    /// Returns the 24-hour turnover in USD.
    pub fn turnover_24h_usd(&self) -> f64 {
        self.turnover_24h
    }

    /// Returns the price deviation from index.
    pub fn price_deviation_from_index(&self) -> f64 {
        self.last_price - self.index_price
    }

    /// Returns the percentage deviation from index.
    pub fn price_deviation_percentage(&self) -> f64 {
        if self.index_price == 0.0 {
            return 0.0;
        }
        (self.last_price - self.index_price) / self.index_price
    }

    /// Returns the mark price deviation from index.
    pub fn mark_price_deviation(&self) -> f64 {
        self.mark_price - self.index_price
    }

    /// Returns the mark price percentage deviation.
    pub fn mark_price_deviation_percentage(&self) -> f64 {
        if self.index_price == 0.0 {
            return 0.0;
        }
        (self.mark_price - self.index_price) / self.index_price
    }

    /// Returns the basis (futures - spot) if available.
    pub fn basis_f64(&self) -> Option<f64> {
        if self.basis.is_empty() {
            None
        } else {
            self.basis.parse().ok()
        }
    }

    /// Returns the basis rate if available.
    pub fn basis_rate_f64(&self) -> Option<f64> {
        if self.basis_rate.is_empty() {
            None
        } else {
            self.basis_rate.parse().ok()
        }
    }

    /// Returns the delivery fee rate as a floating-point number.
    pub fn delivery_fee_rate_f64(&self) -> Option<f64> {
        if self.delivery_fee_rate.is_empty() {
            None
        } else {
            self.delivery_fee_rate.parse().ok()
        }
    }

    /// Returns whether this is a perpetual contract.
    pub fn is_perpetual(&self) -> bool {
        self.delivery_time == 0
    }

    /// Returns the delivery time as a DateTime if available.
    pub fn delivery_time_datetime(&self) -> Option<DateTime<Utc>> {
        if self.delivery_time == 0 {
            None
        } else {
            DateTime::from_timestamp((self.delivery_time / 1000) as i64, 0)
        }
    }

    /// Returns the time to delivery in seconds if available.
    pub fn time_to_delivery(&self) -> Option<i64> {
        if self.delivery_time == 0 {
            None
        } else {
            let now = Utc::now().timestamp_millis() as u64;
            if self.delivery_time > now {
                Some(((self.delivery_time - now) / 1000) as i64)
            } else {
                Some(0)
            }
        }
    }

    /// Returns the predicted delivery price as a floating-point number.
    pub fn predicted_delivery_price_f64(&self) -> Option<f64> {
        if self.predicted_delivery_price.is_empty() {
            None
        } else {
            self.predicted_delivery_price.parse().ok()
        }
    }

    /// Returns the pre-open price as a floating-point number.
    pub fn pre_open_price_f64(&self) -> Option<f64> {
        if self.pre_open_price.is_empty() {
            None
        } else {
            self.pre_open_price.parse().ok()
        }
    }

    /// Returns the pre-quantity as a floating-point number.
    pub fn pre_qty_f64(&self) -> Option<f64> {
        if self.pre_qty.is_empty() {
            None
        } else {
            self.pre_qty.parse().ok()
        }
    }

    /// Returns the funding cap as a floating-point number.
    pub fn funding_cap_f64(&self) -> Option<f64> {
        if self.funding_cap.is_empty() {
            None
        } else {
            self.funding_cap.parse().ok()
        }
    }

    /// Returns the basis rate year as a floating-point number.
    pub fn basis_rate_year_f64(&self) -> Option<f64> {
        if self.basis_rate_year.is_empty() {
            None
        } else {
            self.basis_rate_year.parse().ok()
        }
    }

    /// Returns the funding interval in hours as a floating-point number.
    pub fn funding_interval_hour_f64(&self) -> Option<f64> {
        if self.funding_interval_hour.is_empty() {
            None
        } else {
            self.funding_interval_hour.parse().ok()
        }
    }

    /// Returns whether this is a pre-market contract.
    pub fn is_pre_market(&self) -> bool {
        !self.cur_pre_listing_phase.is_empty()
    }

    /// Returns the 24-hour price range.
    pub fn price_range_24h(&self) -> f64 {
        self.high_24h - self.low_24h
    }

    /// Returns the 24-hour price range percentage.
    pub fn price_range_percentage_24h(&self) -> f64 {
        if self.low_24h == 0.0 {
            return 0.0;
        }
        self.price_range_24h() / self.low_24h
    }

    /// Returns the current price position within the 24-hour range (0-1).
    pub fn price_position_in_range(&self) -> f64 {
        let range = self.price_range_24h();
        if range == 0.0 {
            return 0.5;
        }
        (self.last_price - self.low_24h) / range
    }

    /// Returns the volume-weighted average price estimate.
    pub fn vwap_estimate(&self) -> f64 {
        if self.volume_24h == 0.0 {
            return self.last_price;
        }
        self.turnover_24h / self.volume_24h
    }

    /// Returns the market depth score (bid size / ask size ratio).
    pub fn market_depth_score(&self) -> f64 {
        self.bid_ask_size_ratio()
    }

    /// Returns the liquidity score (higher is more liquid).
    pub fn liquidity_score(&self) -> f64 {
        let spread_percentage = self.spread() / self.mid_price();
        let depth = self.total_top_size();

        // Normalize spread (lower is better)
        let spread_score = 1.0 / (1.0 + spread_percentage * 100.0);

        // Normalize depth (higher is better)
        let depth_score = depth / (depth + 1000.0);

        (spread_score + depth_score) / 2.0
    }
}
