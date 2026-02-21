use crate::prelude::*;

/// Ticker data for options contracts.
///
/// Contains market data for options contracts, including Greeks (delta, gamma, vega, theta),
/// implied volatility, and other options-specific metrics.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OptionsTicker {
    /// Symbol name.
    pub symbol: String,

    /// Best bid price.
    #[serde(rename = "bid1Price")]
    pub bid1_price: String,

    /// Best bid size.
    #[serde(rename = "bid1Size")]
    pub bid1_size: String,

    /// Best bid implied volatility.
    #[serde(rename = "bid1Iv")]
    pub bid1_iv: String,

    /// Best ask price.
    #[serde(rename = "ask1Price")]
    pub ask1_price: String,

    /// Best ask size.
    #[serde(rename = "ask1Size")]
    pub ask1_size: String,

    /// Best ask implied volatility.
    #[serde(rename = "ask1Iv")]
    pub ask1_iv: String,

    /// Last traded price.
    #[serde(rename = "lastPrice")]
    pub last_price: String,

    /// The highest price in the last 24 hours.
    #[serde(rename = "highPrice24h")]
    pub high_price_24h: String,

    /// The lowest price in the last 24 hours.
    #[serde(rename = "lowPrice24h")]
    pub low_price_24h: String,

    /// Mark price.
    #[serde(rename = "markPrice")]
    pub mark_price: String,

    /// Index price.
    #[serde(rename = "indexPrice")]
    pub index_price: String,

    /// Mark price implied volatility.
    #[serde(rename = "markIv")]
    pub mark_iv: String,

    /// Underlying asset price.
    #[serde(rename = "underlyingPrice")]
    pub underlying_price: String,

    /// Open interest size.
    #[serde(rename = "openInterest")]
    pub open_interest: String,

    /// Turnover for 24h.
    #[serde(rename = "turnover24h")]
    pub turnover_24h: String,

    /// Volume for 24h.
    #[serde(rename = "volume24h")]
    pub volume_24h: String,

    /// Total volume.
    #[serde(rename = "totalVolume")]
    pub total_volume: String,

    /// Total turnover.
    #[serde(rename = "totalTurnover")]
    pub total_turnover: String,

    /// Delta - rate of change of option price with respect to underlying price.
    pub delta: String,

    /// Gamma - rate of change of delta with respect to underlying price.
    pub gamma: String,

    /// Vega - rate of change of option price with respect to implied volatility.
    pub vega: String,

    /// Theta - rate of change of option price with respect to time.
    pub theta: String,

    /// Predicted delivery price. It has a value 30 mins before delivery.
    #[serde(rename = "predictedDeliveryPrice")]
    pub predicted_delivery_price: String,

    /// The change in the last 24 hours.
    #[serde(rename = "change24h")]
    pub change_24h: String,
}

impl OptionsTicker {
    /// Creates a new OptionsTicker with default values.
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            bid1_price: String::new(),
            bid1_size: String::new(),
            bid1_iv: String::new(),
            ask1_price: String::new(),
            ask1_size: String::new(),
            ask1_iv: String::new(),
            last_price: String::new(),
            high_price_24h: String::new(),
            low_price_24h: String::new(),
            mark_price: String::new(),
            index_price: String::new(),
            mark_iv: String::new(),
            underlying_price: String::new(),
            open_interest: String::new(),
            turnover_24h: String::new(),
            volume_24h: String::new(),
            total_volume: String::new(),
            total_turnover: String::new(),
            delta: String::new(),
            gamma: String::new(),
            vega: String::new(),
            theta: String::new(),
            predicted_delivery_price: String::new(),
            change_24h: String::new(),
        }
    }

    /// Returns the best bid price as a floating-point number, if parseable.
    pub fn bid1_price_f64(&self) -> Option<f64> {
        self.bid1_price.parse().ok()
    }

    /// Returns the best ask price as a floating-point number, if parseable.
    pub fn ask1_price_f64(&self) -> Option<f64> {
        self.ask1_price.parse().ok()
    }

    /// Returns the last price as a floating-point number, if parseable.
    pub fn last_price_f64(&self) -> Option<f64> {
        self.last_price.parse().ok()
    }

    /// Returns the mark price as a floating-point number, if parseable.
    pub fn mark_price_f64(&self) -> Option<f64> {
        self.mark_price.parse().ok()
    }

    /// Returns the index price as a floating-point number, if parseable.
    pub fn index_price_f64(&self) -> Option<f64> {
        self.index_price.parse().ok()
    }

    /// Returns the underlying price as a floating-point number, if parseable.
    pub fn underlying_price_f64(&self) -> Option<f64> {
        self.underlying_price.parse().ok()
    }

    /// Returns the bid-ask spread.
    pub fn spread(&self) -> Option<f64> {
        let bid = self.bid1_price_f64()?;
        let ask = self.ask1_price_f64()?;
        Some(ask - bid)
    }

    /// Returns the mid price (average of bid and ask).
    pub fn mid_price(&self) -> Option<f64> {
        let bid = self.bid1_price_f64()?;
        let ask = self.ask1_price_f64()?;
        Some((bid + ask) / 2.0)
    }

    /// Returns the 24-hour change as a floating-point number, if parseable.
    pub fn change_24h_f64(&self) -> Option<f64> {
        self.change_24h.parse().ok()
    }

    /// Returns the 24-hour high price as a floating-point number, if parseable.
    pub fn high_price_24h_f64(&self) -> Option<f64> {
        self.high_price_24h.parse().ok()
    }

    /// Returns the 24-hour low price as a floating-point number, if parseable.
    pub fn low_price_24h_f64(&self) -> Option<f64> {
        self.low_price_24h.parse().ok()
    }

    /// Returns the open interest as a floating-point number, if parseable.
    pub fn open_interest_f64(&self) -> Option<f64> {
        self.open_interest.parse().ok()
    }

    /// Returns the 24-hour volume as a floating-point number, if parseable.
    pub fn volume_24h_f64(&self) -> Option<f64> {
        self.volume_24h.parse().ok()
    }

    /// Returns the 24-hour turnover as a floating-point number, if parseable.
    pub fn turnover_24h_f64(&self) -> Option<f64> {
        self.turnover_24h.parse().ok()
    }

    /// Returns the total volume as a floating-point number, if parseable.
    pub fn total_volume_f64(&self) -> Option<f64> {
        self.total_volume.parse().ok()
    }

    /// Returns the total turnover as a floating-point number, if parseable.
    pub fn total_turnover_f64(&self) -> Option<f64> {
        self.total_turnover.parse().ok()
    }

    /// Returns the delta as a floating-point number, if parseable.
    pub fn delta_f64(&self) -> Option<f64> {
        self.delta.parse().ok()
    }

    /// Returns the gamma as a floating-point number, if parseable.
    pub fn gamma_f64(&self) -> Option<f64> {
        self.gamma.parse().ok()
    }

    /// Returns the vega as a floating-point number, if parseable.
    pub fn vega_f64(&self) -> Option<f64> {
        self.vega.parse().ok()
    }

    /// Returns the theta as a floating-point number, if parseable.
    pub fn theta_f64(&self) -> Option<f64> {
        self.theta.parse().ok()
    }

    /// Returns the bid implied volatility as a floating-point number, if parseable.
    pub fn bid1_iv_f64(&self) -> Option<f64> {
        self.bid1_iv.parse().ok()
    }

    /// Returns the ask implied volatility as a floating-point number, if parseable.
    pub fn ask1_iv_f64(&self) -> Option<f64> {
        self.ask1_iv.parse().ok()
    }

    /// Returns the mark implied volatility as a floating-point number, if parseable.
    pub fn mark_iv_f64(&self) -> Option<f64> {
        self.mark_iv.parse().ok()
    }

    /// Returns the predicted delivery price as a floating-point number, if parseable.
    pub fn predicted_delivery_price_f64(&self) -> Option<f64> {
        self.predicted_delivery_price.parse().ok()
    }

    /// Returns the bid size as a floating-point number, if parseable.
    pub fn bid1_size_f64(&self) -> Option<f64> {
        self.bid1_size.parse().ok()
    }

    /// Returns the ask size as a floating-point number, if parseable.
    pub fn ask1_size_f64(&self) -> Option<f64> {
        self.ask1_size.parse().ok()
    }

    /// Returns the bid-ask size ratio.
    pub fn bid_ask_size_ratio(&self) -> Option<f64> {
        let bid_size = self.bid1_size_f64()?;
        let ask_size = self.ask1_size_f64()?;
        if ask_size == 0.0 {
            return None;
        }
        Some(bid_size / ask_size)
    }

    /// Returns the total size (bid + ask).
    pub fn total_size(&self) -> Option<f64> {
        let bid_size = self.bid1_size_f64()?;
        let ask_size = self.ask1_size_f64()?;
        Some(bid_size + ask_size)
    }

    /// Returns the implied volatility spread (ask IV - bid IV).
    pub fn iv_spread(&self) -> Option<f64> {
        let bid_iv = self.bid1_iv_f64()?;
        let ask_iv = self.ask1_iv_f64()?;
        Some(ask_iv - bid_iv)
    }

    /// Returns the mid implied volatility (average of bid and ask IV).
    pub fn mid_iv(&self) -> Option<f64> {
        let bid_iv = self.bid1_iv_f64()?;
        let ask_iv = self.ask1_iv_f64()?;
        Some((bid_iv + ask_iv) / 2.0)
    }
}
