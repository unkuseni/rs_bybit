use crate::prelude::*;
use chrono::{DateTime, Utc};

/// Represents ticker data for a spot trading pair.
///
/// Contains market data for spot markets. Not relevant for perpetual futures but included for completeness.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpotTicker {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the spot trading pair. Not relevant for perpetuals.
    pub symbol: String,

    /// The best bid price.
    ///
    /// The highest price buyers are willing to pay in the spot market. Not relevant for perpetuals.
    #[serde(rename = "bid1Price", with = "string_to_float")]
    pub bid_price: f64,

    /// The size of the best bid order.
    ///
    /// The quantity at the best bid price. Not relevant for perpetuals.
    #[serde(rename = "bid1Size", with = "string_to_float")]
    pub bid_size: f64,

    /// The best ask price.
    ///
    /// The lowest price sellers are willing to accept in the spot market. Not relevant for perpetuals.
    #[serde(rename = "ask1Price", with = "string_to_float")]
    pub ask_price: f64,

    /// The size of the best ask order.
    ///
    /// The quantity at the best ask price. Not relevant for perpetuals.
    #[serde(rename = "ask1Size", with = "string_to_float")]
    pub ask_size: f64,

    /// The last traded price.
    ///
    /// The most recent price in the spot market. Not relevant for perpetuals.
    #[serde(with = "string_to_float")]
    pub last_price: f64,

    /// The price 24 hours ago.
    ///
    /// The price in the spot market 24 hours prior. Not relevant for perpetuals.
    #[serde(rename = "prevPrice24h", with = "string_to_float")]
    pub prev_price_24h: f64,

    /// The percentage price change over the last 24 hours.
    ///
    /// Calculated for the spot market. Not relevant for perpetuals.
    #[serde(rename = "price24hPcnt", with = "string_to_float")]
    pub daily_change_percentage: f64,

    /// The highest price in the last 24 hours.
    ///
    /// The peak price in the spot market. Not relevant for perpetuals.
    #[serde(rename = "highPrice24h", with = "string_to_float")]
    pub high_24h: f64,

    /// The lowest price in the last 24 hours.
    ///
    /// The trough price in the spot market. Not relevant for perpetuals.
    #[serde(rename = "lowPrice24h", with = "string_to_float")]
    pub low_24h: f64,

    /// The trading turnover in the last 24 hours (in quote asset).
    ///
    /// The total value traded in the spot market. Not relevant for perpetuals.
    #[serde(with = "string_to_float")]
    pub turnover_24h: f64,

    /// The trading volume in the last 24 hours (in base asset).
    ///
    /// The total quantity traded in the spot market. Not relevant for perpetuals.
    #[serde(with = "string_to_float")]
    pub volume_24h: f64,

    /// The USD index price.
    ///
    /// The spot price of the asset in USD. Not relevant for perpetuals.
    #[serde(with = "string_to_float")]
    pub usd_index_price: f64,
}

impl SpotTicker {
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

    /// Returns the 24-hour turnover in USD.
    pub fn turnover_24h_usd(&self) -> f64 {
        self.turnover_24h
    }

    /// Returns the 24-hour volume in base asset.
    pub fn volume_24h_base(&self) -> f64 {
        self.volume_24h
    }

    /// Returns the USD value of the 24-hour volume.
    pub fn volume_24h_usd(&self) -> f64 {
        self.volume_24h * self.usd_index_price
    }

    /// Returns the price deviation from USD index.
    pub fn price_deviation_from_usd_index(&self) -> f64 {
        self.last_price - self.usd_index_price
    }

    /// Returns the percentage deviation from USD index.
    pub fn price_deviation_percentage(&self) -> f64 {
        if self.usd_index_price == 0.0 {
            return 0.0;
        }
        (self.last_price - self.usd_index_price) / self.usd_index_price
    }

    /// Returns the absolute daily price change.
    pub fn daily_price_change(&self) -> f64 {
        self.last_price - self.prev_price_24h
    }

    /// Returns the daily price change percentage.
    pub fn daily_change_percentage(&self) -> f64 {
        if self.prev_price_24h == 0.0 {
            return 0.0;
        }
        self.daily_price_change() / self.prev_price_24h
    }

    /// Returns whether the price is currently at 24-hour high.
    pub fn is_at_24h_high(&self) -> bool {
        (self.last_price - self.high_24h).abs() < 0.000001
    }

    /// Returns whether the price is currently at 24-hour low.
    pub fn is_at_24h_low(&self) -> bool {
        (self.last_price - self.low_24h).abs() < 0.000001
    }

    /// Returns the average price (mid price) for the last 24 hours.
    pub fn average_price_24h(&self) -> f64 {
        (self.high_24h + self.low_24h) / 2.0
    }

    /// Returns the price volatility estimate based on 24-hour range.
    pub fn volatility_estimate(&self) -> f64 {
        self.price_range_percentage_24h()
    }

    /// Returns the bid-ask spread as a percentage of mid price.
    pub fn spread_percentage(&self) -> f64 {
        if self.mid_price() == 0.0 {
            return 0.0;
        }
        self.spread() / self.mid_price()
    }

    /// Returns the effective cost of trading (spread percentage).
    pub fn effective_cost(&self) -> f64 {
        self.spread_percentage()
    }

    /// Returns the market efficiency score (lower spread, higher depth = better).
    pub fn market_efficiency_score(&self) -> f64 {
        let spread_score = 1.0 - self.spread_percentage().min(0.01) / 0.01;
        let depth_score = self.total_top_size() / (self.total_top_size() + 1000.0);
        (spread_score + depth_score) / 2.0
    }

    /// Returns the timestamp for when this ticker data was generated.
    pub fn timestamp(&self) -> DateTime<Utc> {
        // Note: Spot ticker doesn't include a timestamp in the response
        // This would need to be added by the caller when receiving the data
        Utc::now()
    }

    /// Returns the symbol's base currency (first part of symbol).
    pub fn base_currency(&self) -> Option<&str> {
        // Simple heuristic: split at "USDT", "USDC", "BTC", etc.
        if self.symbol.ends_with("USDT") {
            Some(&self.symbol[..self.symbol.len() - 4])
        } else if self.symbol.ends_with("USDC") {
            Some(&self.symbol[..self.symbol.len() - 4])
        } else if self.symbol.ends_with("BTC") {
            Some(&self.symbol[..self.symbol.len() - 3])
        } else if self.symbol.ends_with("ETH") {
            Some(&self.symbol[..self.symbol.len() - 3])
        } else {
            None
        }
    }

    /// Returns the symbol's quote currency (second part of symbol).
    pub fn quote_currency(&self) -> Option<&str> {
        if self.symbol.ends_with("USDT") {
            Some("USDT")
        } else if self.symbol.ends_with("USDC") {
            Some("USDC")
        } else if self.symbol.ends_with("BTC") {
            Some("BTC")
        } else if self.symbol.ends_with("ETH") {
            Some("ETH")
        } else {
            None
        }
    }

    /// Returns whether this is a USD-stablecoin pair.
    pub fn is_usd_stablecoin_pair(&self) -> bool {
        self.symbol.ends_with("USDT") || self.symbol.ends_with("USDC")
    }

    /// Returns whether this is a crypto-to-crypto pair.
    pub fn is_crypto_pair(&self) -> bool {
        !self.is_usd_stablecoin_pair()
    }
}
