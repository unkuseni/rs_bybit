use crate::prelude::*;

/// Represents a single RPI (Real-time Price Improvement) order book level.
///
/// Each level contains the price, non-RPI size, and RPI size for either bids or asks.
/// RPI orders are special orders that can improve prices for takers.
#[derive(Clone, Debug)]
pub struct RPIOrderbookLevel {
    /// The price level.
    pub price: f64,

    /// The non-RPI size at this price level.
    ///
    /// This represents the regular order quantity at this price.
    /// When delta data has size=0, it means all quotations for this price have been filled or cancelled.
    pub non_rpi_size: f64,

    /// The RPI size at this price level.
    ///
    /// This represents the RPI (Real-time Price Improvement) order quantity at this price.
    /// When a bid RPI order crosses with a non-RPI ask price, the quantity of the bid RPI becomes invalid and is hidden.
    /// When an ask RPI order crosses with a non-RPI bid price, the quantity of the ask RPI becomes invalid and is hidden.
    pub rpi_size: f64,
}

impl<'de> Deserialize<'de> for RPIOrderbookLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize as an array of 3 strings
        let arr: [String; 3] = Deserialize::deserialize(deserializer)?;

        let price = arr[0].parse::<f64>().map_err(serde::de::Error::custom)?;
        let non_rpi_size = arr[1].parse::<f64>().map_err(serde::de::Error::custom)?;
        let rpi_size = arr[2].parse::<f64>().map_err(serde::de::Error::custom)?;

        Ok(Self {
            price,
            non_rpi_size,
            rpi_size,
        })
    }
}

impl Serialize for RPIOrderbookLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize as an array of 3 strings
        let arr = [
            self.price.to_string(),
            self.non_rpi_size.to_string(),
            self.rpi_size.to_string(),
        ];
        arr.serialize(serializer)
    }
}

impl RPIOrderbookLevel {
    /// Constructs a new RPIOrderbookLevel with specified price, non-RPI size, and RPI size.
    pub fn new(price: f64, non_rpi_size: f64, rpi_size: f64) -> Self {
        Self {
            price,
            non_rpi_size,
            rpi_size,
        }
    }

    /// Returns the total size (non-RPI + RPI) at this price level.
    pub fn total_size(&self) -> f64 {
        self.non_rpi_size + self.rpi_size
    }

    /// Returns true if this level has any RPI size.
    pub fn has_rpi(&self) -> bool {
        self.rpi_size > 0.0
    }

    /// Returns true if this level has any non-RPI size.
    pub fn has_non_rpi(&self) -> bool {
        self.non_rpi_size > 0.0
    }

    /// Returns the notional value (price × total size).
    pub fn notional_value(&self) -> f64 {
        self.price * self.total_size()
    }

    /// Returns the RPI ratio (RPI size / total size).
    pub fn rpi_ratio(&self) -> f64 {
        let total = self.total_size();
        if total == 0.0 {
            return 0.0;
        }
        self.rpi_size / total
    }

    /// Returns the non-RPI ratio (non-RPI size / total size).
    pub fn non_rpi_ratio(&self) -> f64 {
        let total = self.total_size();
        if total == 0.0 {
            return 0.0;
        }
        self.non_rpi_size / total
    }

    /// Returns the effective price for takers (considering RPI improvement).
    pub fn effective_taker_price(&self, is_buy: bool) -> f64 {
        if self.has_rpi() {
            // RPI orders can provide price improvement
            let improvement = if is_buy {
                // For buy orders, RPI ask orders might provide better prices
                -self.price * 0.0001 // 0.01% improvement estimate
            } else {
                // For sell orders, RPI bid orders might provide better prices
                self.price * 0.0001 // 0.01% improvement estimate
            };
            self.price + improvement
        } else {
            self.price
        }
    }

    /// Returns the price impact if this level were consumed.
    pub fn price_impact(&self, reference_price: f64) -> f64 {
        if reference_price == 0.0 {
            return 0.0;
        }
        (self.price - reference_price).abs() / reference_price
    }

    /// Returns whether this level provides price improvement over a reference price.
    pub fn provides_price_improvement(&self, reference_price: f64, is_buy: bool) -> bool {
        if is_buy {
            // For buy orders, lower price is better
            self.price < reference_price
        } else {
            // For sell orders, higher price is better
            self.price > reference_price
        }
    }

    /// Returns the improvement amount over a reference price.
    pub fn improvement_amount(&self, reference_price: f64, is_buy: bool) -> f64 {
        if self.provides_price_improvement(reference_price, is_buy) {
            if is_buy {
                reference_price - self.price
            } else {
                self.price - reference_price
            }
        } else {
            0.0
        }
    }

    /// Returns the improvement percentage over a reference price.
    pub fn improvement_percentage(&self, reference_price: f64, is_buy: bool) -> f64 {
        if reference_price == 0.0 {
            return 0.0;
        }
        self.improvement_amount(reference_price, is_buy) / reference_price
    }

    /// Returns a scaled version of this level.
    pub fn scaled(&self, factor: f64) -> Self {
        Self {
            price: self.price,
            non_rpi_size: self.non_rpi_size * factor,
            rpi_size: self.rpi_size * factor,
        }
    }

    /// Returns whether this level is valid (positive price and at least one size > 0).
    pub fn is_valid(&self) -> bool {
        self.price > 0.0 && (self.non_rpi_size > 0.0 || self.rpi_size > 0.0)
    }

    /// Returns the weighted average price considering RPI probability.
    pub fn weighted_price_with_rpi_probability(&self, rpi_execution_probability: f64) -> f64 {
        let rpi_price = self.effective_taker_price(true); // Using buy side for calculation
        self.price * (1.0 - rpi_execution_probability) + rpi_price * rpi_execution_probability
    }
}

/// Represents the RPI (Real-time Price Improvement) order book for a trading pair.
///
/// Contains the current bid and ask levels with RPI information, along with metadata.
/// RPI order books show both regular orders and RPI orders, which can provide price improvement.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RPIOrderbook {
    /// The trading pair symbol (e.g., "BTCUSDT").
    #[serde(rename = "s")]
    pub symbol: String,

    /// A list of ask (sell) orders with RPI information.
    ///
    /// Each element is an array of [price, non-RPI size, RPI size].
    /// Sorted by price in ascending order.
    #[serde(rename = "a")]
    pub asks: Vec<RPIOrderbookLevel>,

    /// A list of bid (buy) orders with RPI information.
    ///
    /// Each element is an array of [price, non-RPI size, RPI size].
    /// Sorted by price in descending order.
    #[serde(rename = "b")]
    pub bids: Vec<RPIOrderbookLevel>,

    /// The timestamp (ms) that the system generates the data.
    #[serde(rename = "ts")]
    pub timestamp: u64,

    /// Update ID, is always in sequence corresponds to `u` in the 50-level WebSocket RPI orderbook stream.
    #[serde(rename = "u")]
    pub update_id: u64,

    /// Cross sequence.
    ///
    /// You can use this field to compare different levels orderbook data, and for the smaller seq,
    /// then it means the data is generated earlier.
    #[serde(rename = "seq")]
    pub sequence: u64,

    /// The timestamp from the matching engine when this orderbook data is produced.
    /// It can be correlated with `T` from public trade channel.
    #[serde(rename = "cts")]
    pub matching_engine_timestamp: u64,
}

impl RPIOrderbook {
    /// Returns the best ask price (lowest ask).
    pub fn best_ask(&self) -> Option<f64> {
        self.asks.first().map(|ask| ask.price)
    }

    /// Returns the best bid price (highest bid).
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.first().map(|bid| bid.price)
    }

    /// Returns the best ask with RPI information.
    pub fn best_ask_with_rpi(&self) -> Option<&RPIOrderbookLevel> {
        self.asks.first()
    }

    /// Returns the best bid with RPI information.
    pub fn best_bid_with_rpi(&self) -> Option<&RPIOrderbookLevel> {
        self.bids.first()
    }

    /// Returns the bid-ask spread.
    pub fn spread(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Returns the mid price (average of best bid and ask).
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some((bid + ask) / 2.0),
            _ => None,
        }
    }

    /// Returns the spread as a percentage of mid price.
    pub fn spread_percentage(&self) -> Option<f64> {
        match (self.spread(), self.mid_price()) {
            (Some(spread), Some(mid)) if mid != 0.0 => Some(spread / mid),
            _ => None,
        }
    }

    /// Returns the total RPI size on the ask side.
    pub fn total_ask_rpi_size(&self) -> f64 {
        self.asks.iter().map(|ask| ask.rpi_size).sum()
    }

    /// Returns the total non-RPI size on the ask side.
    pub fn total_ask_non_rpi_size(&self) -> f64 {
        self.asks.iter().map(|ask| ask.non_rpi_size).sum()
    }

    /// Returns the total RPI size on the bid side.
    pub fn total_bid_rpi_size(&self) -> f64 {
        self.bids.iter().map(|bid| bid.rpi_size).sum()
    }

    /// Returns the total non-RPI size on the bid side.
    pub fn total_bid_non_rpi_size(&self) -> f64 {
        self.bids.iter().map(|bid| bid.non_rpi_size).sum()
    }

    /// Returns the total size (RPI + non-RPI) on the ask side.
    pub fn total_ask_size(&self) -> f64 {
        self.asks.iter().map(|ask| ask.total_size()).sum()
    }

    /// Returns the total size (RPI + non-RPI) on the bid side.
    pub fn total_bid_size(&self) -> f64 {
        self.bids.iter().map(|bid| bid.total_size()).sum()
    }

    /// Returns the total notional value on the ask side.
    pub fn total_ask_notional(&self) -> f64 {
        self.asks.iter().map(|ask| ask.notional_value()).sum()
    }

    /// Returns the total notional value on the bid side.
    pub fn total_bid_notional(&self) -> f64 {
        self.bids.iter().map(|bid| bid.notional_value()).sum()
    }

    /// Returns the average RPI ratio on the ask side.
    pub fn average_ask_rpi_ratio(&self) -> f64 {
        let total_ask_size = self.total_ask_size();
        if total_ask_size == 0.0 {
            return 0.0;
        }
        self.total_ask_rpi_size() / total_ask_size
    }

    /// Returns the average RPI ratio on the bid side.
    pub fn average_bid_rpi_ratio(&self) -> f64 {
        let total_bid_size = self.total_bid_size();
        if total_bid_size == 0.0 {
            return 0.0;
        }
        self.total_bid_rpi_size() / total_bid_size
    }

    /// Returns the bid-ask RPI ratio difference.
    pub fn rpi_ratio_imbalance(&self) -> f64 {
        self.average_bid_rpi_ratio() - self.average_ask_rpi_ratio()
    }

    /// Returns the order book imbalance considering RPI sizes.
    pub fn order_book_imbalance_with_rpi(&self) -> f64 {
        let total_bid = self.total_bid_size();
        let total_ask = self.total_ask_size();
        let total = total_bid + total_ask;
        if total == 0.0 {
            return 0.0;
        }
        (total_bid - total_ask) / total
    }

    /// Returns the weighted average ask price considering RPI improvement.
    pub fn weighted_average_ask_price_with_rpi(&self, target_quantity: f64) -> Option<f64> {
        let mut remaining = target_quantity;
        let mut total_value = 0.0;

        for ask in &self.asks {
            let qty_to_take = ask.total_size().min(remaining);
            // Use effective price considering RPI improvement for takers
            let effective_price = ask.effective_taker_price(false);
            total_value += qty_to_take * effective_price;
            remaining -= qty_to_take;

            if remaining <= 0.0 {
                break;
            }
        }

        if remaining > 0.0 {
            None
        } else {
            Some(total_value / target_quantity)
        }
    }

    /// Returns the weighted average bid price considering RPI improvement.
    pub fn weighted_average_bid_price_with_rpi(&self, target_quantity: f64) -> Option<f64> {
        let mut remaining = target_quantity;
        let mut total_value = 0.0;

        for bid in &self.bids {
            let qty_to_take = bid.total_size().min(remaining);
            // Use effective price considering RPI improvement for takers
            let effective_price = bid.effective_taker_price(true);
            total_value += qty_to_take * effective_price;
            remaining -= qty_to_take;

            if remaining <= 0.0 {
                break;
            }
        }

        if remaining > 0.0 {
            None
        } else {
            Some(total_value / target_quantity)
        }
    }

    /// Returns the price impact for a given quantity considering RPI.
    pub fn ask_price_impact_with_rpi(&self, quantity: f64) -> Option<f64> {
        let wap = self.weighted_average_ask_price_with_rpi(quantity)?;
        let best_ask = self.best_ask()?;
        Some((wap - best_ask) / best_ask)
    }

    /// Returns the price impact for a given quantity considering RPI.
    pub fn bid_price_impact_with_rpi(&self, quantity: f64) -> Option<f64> {
        let wap = self.weighted_average_bid_price_with_rpi(quantity)?;
        let best_bid = self.best_bid()?;
        Some((best_bid - wap) / best_bid)
    }

    /// Returns the expected price improvement for takers.
    pub fn expected_taker_improvement(&self, is_buy: bool, quantity: f64) -> Option<f64> {
        let (wap_with_rpi, best_price) = if is_buy {
            (
                self.weighted_average_ask_price_with_rpi(quantity)?,
                self.best_ask()?,
            )
        } else {
            (
                self.weighted_average_bid_price_with_rpi(quantity)?,
                self.best_bid()?,
            )
        };

        if is_buy {
            // For buy orders, lower price is better
            Some((best_price - wap_with_rpi) / best_price)
        } else {
            // For sell orders, higher price is better
            Some((wap_with_rpi - best_price) / best_price)
        }
    }

    /// Returns the liquidity score considering RPI availability.
    pub fn liquidity_score_with_rpi(&self) -> f64 {
        let spread_score = match self.spread_percentage() {
            Some(spread_pct) => 1.0 / (1.0 + spread_pct * 1000.0),
            None => 0.0,
        };

        let depth_score = {
            let total_qty = self.total_ask_size() + self.total_bid_size();
            total_qty / (total_qty + 1000.0)
        };

        let rpi_score = {
            let avg_rpi_ratio = (self.average_ask_rpi_ratio() + self.average_bid_rpi_ratio()) / 2.0;
            avg_rpi_ratio
        };

        spread_score * 0.3 + depth_score * 0.3 + rpi_score * 0.4
    }

    /// Returns the timestamp as a DateTime.
    pub fn timestamp_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp((self.timestamp / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now)
    }

    /// Returns the matching engine timestamp as a DateTime.
    pub fn matching_engine_timestamp_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp((self.matching_engine_timestamp / 1000) as i64, 0)
            .unwrap_or_else(chrono::Utc::now)
    }

    /// Returns the processing latency.
    pub fn processing_latency_ms(&self) -> i64 {
        if self.matching_engine_timestamp > self.timestamp {
            (self.matching_engine_timestamp - self.timestamp) as i64
        } else {
            (self.timestamp - self.matching_engine_timestamp) as i64
        }
    }
}
