use crate::prelude::*;

/// Represents the order book for a trading pair.
///
/// Contains the current bid and ask levels, along with metadata like the update ID. Bots use this to analyze market depth and liquidity in perpetual futures.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderBook {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Confirms the trading pair for the order book. Bots should verify this matches the requested symbol.
    #[serde(rename = "s")]
    pub symbol: String,

    /// A list of ask (sell) orders.
    ///
    /// Contains the current ask prices and quantities. Bots use this to assess selling pressure and determine resistance levels in perpetual futures.
    #[serde(rename = "a")]
    pub asks: Vec<Ask>,

    /// A list of bid (buy) orders.
    ///
    /// Contains the current bid prices and quantities. Bots use this to assess buying support and determine support levels in perpetual futures.
    #[serde(rename = "b")]
    pub bids: Vec<Bid>,

    /// The timestamp of the order book snapshot (Unix timestamp in milliseconds).
    ///
    /// Indicates when the order book data was captured. Bots should use this to ensure the data is recent, as stale order book data can lead to poor trading decisions.
    #[serde(rename = "ts")]
    pub timestamp: u64,

    /// The update ID of the order book.
    ///
    /// A unique identifier for the order book snapshot. Bots can use this to track updates and ensure they’re processing the latest data, especially in WebSocket streams.
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

impl OrderBook {
    /// Returns the best ask price (lowest ask).
    pub fn best_ask(&self) -> Option<f64> {
        self.asks.first().map(|ask| ask.price)
    }

    /// Returns the best bid price (highest bid).
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.first().map(|bid| bid.price)
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

    /// Returns the total quantity on the ask side.
    pub fn total_ask_quantity(&self) -> f64 {
        self.asks.iter().map(|ask| ask.qty).sum()
    }

    /// Returns the total quantity on the bid side.
    pub fn total_bid_quantity(&self) -> f64 {
        self.bids.iter().map(|bid| bid.qty).sum()
    }

    /// Returns the total quantity (bids + asks).
    pub fn total_quantity(&self) -> f64 {
        self.total_bid_quantity() + self.total_ask_quantity()
    }

    /// Returns the bid-ask quantity ratio.
    pub fn bid_ask_quantity_ratio(&self) -> f64 {
        let total_bid = self.total_bid_quantity();
        let total_ask = self.total_ask_quantity();
        if total_ask == 0.0 {
            return 0.0;
        }
        total_bid / total_ask
    }

    /// Returns the order book imbalance (bid - ask) / (bid + ask).
    pub fn order_book_imbalance(&self) -> f64 {
        let total_bid = self.total_bid_quantity();
        let total_ask = self.total_ask_quantity();
        let total = total_bid + total_ask;
        if total == 0.0 {
            return 0.0;
        }
        (total_bid - total_ask) / total
    }

    /// Returns the weighted average price for a given quantity on the ask side.
    pub fn weighted_average_ask_price(&self, target_quantity: f64) -> Option<f64> {
        let mut remaining = target_quantity;
        let mut total_value = 0.0;

        for ask in &self.asks {
            let qty_to_take = ask.qty.min(remaining);
            total_value += qty_to_take * ask.price;
            remaining -= qty_to_take;

            if remaining <= 0.0 {
                break;
            }
        }

        if remaining > 0.0 {
            // Not enough liquidity
            None
        } else {
            Some(total_value / target_quantity)
        }
    }

    /// Returns the weighted average price for a given quantity on the bid side.
    pub fn weighted_average_bid_price(&self, target_quantity: f64) -> Option<f64> {
        let mut remaining = target_quantity;
        let mut total_value = 0.0;

        for bid in &self.bids {
            let qty_to_take = bid.qty.min(remaining);
            total_value += qty_to_take * bid.price;
            remaining -= qty_to_take;

            if remaining <= 0.0 {
                break;
            }
        }

        if remaining > 0.0 {
            // Not enough liquidity
            None
        } else {
            Some(total_value / target_quantity)
        }
    }

    /// Returns the price impact for a given quantity on the ask side.
    pub fn ask_price_impact(&self, quantity: f64) -> Option<f64> {
        let wap = self.weighted_average_ask_price(quantity)?;
        let best_ask = self.best_ask()?;
        Some((wap - best_ask) / best_ask)
    }

    /// Returns the price impact for a given quantity on the bid side.
    pub fn bid_price_impact(&self, quantity: f64) -> Option<f64> {
        let wap = self.weighted_average_bid_price(quantity)?;
        let best_bid = self.best_bid()?;
        Some((best_bid - wap) / best_bid)
    }

    /// Returns the cumulative quantity up to a given price level on the ask side.
    pub fn cumulative_ask_quantity_to_price(&self, price: f64) -> f64 {
        self.asks
            .iter()
            .take_while(|ask| ask.price <= price)
            .map(|ask| ask.qty)
            .sum()
    }

    /// Returns the cumulative quantity up to a given price level on the bid side.
    pub fn cumulative_bid_quantity_to_price(&self, price: f64) -> f64 {
        self.bids
            .iter()
            .take_while(|bid| bid.price >= price)
            .map(|bid| bid.qty)
            .sum()
    }

    /// Returns the price level that contains a given cumulative quantity on the ask side.
    pub fn ask_price_for_cumulative_quantity(&self, target_quantity: f64) -> Option<f64> {
        let mut cumulative = 0.0;
        for ask in &self.asks {
            cumulative += ask.qty;
            if cumulative >= target_quantity {
                return Some(ask.price);
            }
        }
        None
    }

    /// Returns the price level that contains a given cumulative quantity on the bid side.
    pub fn bid_price_for_cumulative_quantity(&self, target_quantity: f64) -> Option<f64> {
        let mut cumulative = 0.0;
        for bid in &self.bids {
            cumulative += bid.qty;
            if cumulative >= target_quantity {
                return Some(bid.price);
            }
        }
        None
    }

    /// Returns the market depth (quantity within a percentage range of mid price).
    pub fn market_depth(&self, percentage_range: f64) -> (f64, f64) {
        let mid = match self.mid_price() {
            Some(mid) => mid,
            None => return (0.0, 0.0),
        };

        let lower_bound = mid * (1.0 - percentage_range / 100.0);
        let upper_bound = mid * (1.0 + percentage_range / 100.0);

        let bid_depth = self
            .bids
            .iter()
            .filter(|bid| bid.price >= lower_bound)
            .map(|bid| bid.qty)
            .sum();

        let ask_depth = self
            .asks
            .iter()
            .filter(|ask| ask.price <= upper_bound)
            .map(|ask| ask.qty)
            .sum();

        (bid_depth, ask_depth)
    }

    /// Returns the liquidity score (higher is more liquid).
    pub fn liquidity_score(&self) -> f64 {
        let spread_score = match self.spread_percentage() {
            Some(spread_pct) => 1.0 / (1.0 + spread_pct * 1000.0), // Normalize spread
            None => 0.0,
        };

        let depth_score = {
            let total_qty = self.total_quantity();
            total_qty / (total_qty + 1000.0) // Normalize depth
        };

        let imbalance_score = 1.0 - self.order_book_imbalance().abs();

        spread_score * 0.4 + depth_score * 0.4 + imbalance_score * 0.2
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

    /// Returns the latency between system generation and matching engine.
    pub fn processing_latency_ms(&self) -> i64 {
        if self.matching_engine_timestamp > self.timestamp {
            (self.matching_engine_timestamp - self.timestamp) as i64
        } else {
            (self.timestamp - self.matching_engine_timestamp) as i64
        }
    }

    /// Returns the VWAP (Volume Weighted Average Price) for the visible order book.
    pub fn vwap(&self) -> Option<f64> {
        let total_bid_value: f64 = self.bids.iter().map(|bid| bid.price * bid.qty).sum();
        let total_ask_value: f64 = self.asks.iter().map(|ask| ask.price * ask.qty).sum();
        let total_bid_qty = self.total_bid_quantity();
        let total_ask_qty = self.total_ask_quantity();

        let total_value = total_bid_value + total_ask_value;
        let total_qty = total_bid_qty + total_ask_qty;

        if total_qty == 0.0 {
            None
        } else {
            Some(total_value / total_qty)
        }
    }

    /// Returns the microprice (weighted by inverse of spread).
    pub fn microprice(&self) -> Option<f64> {
        let best_bid = self.best_bid()?;
        let best_ask = self.best_ask()?;
        let bid_qty = self.bids.first().map(|b| b.qty).unwrap_or(0.0);
        let ask_qty = self.asks.first().map(|a| a.qty).unwrap_or(0.0);

        let total_qty = bid_qty + ask_qty;
        if total_qty == 0.0 {
            return None;
        }

        Some((best_bid * ask_qty + best_ask * bid_qty) / total_qty)
    }

    /// Returns the order book slope (price change per unit quantity).
    pub fn order_book_slope(&self, side: OrderBookSide, levels: usize) -> Option<f64> {
        if levels < 2 {
            return None;
        }

        let prices: Vec<f64>;
        let quantities: Vec<f64>;

        match side {
            OrderBookSide::Bid => {
                if self.bids.len() < levels {
                    return None;
                }
                prices = self.bids[..levels].iter().map(|b| b.price).collect();
                quantities = self.bids[..levels].iter().map(|b| b.qty).collect();
            }
            OrderBookSide::Ask => {
                if self.asks.len() < levels {
                    return None;
                }
                prices = self.asks[..levels].iter().map(|a| a.price).collect();
                quantities = self.asks[..levels].iter().map(|a| a.qty).collect();
            }
        }

        // Simple linear regression for slope
        let n = levels as f64;
        let sum_x: f64 = quantities.iter().sum();
        let sum_y: f64 = prices.iter().sum();
        let sum_xy: f64 = quantities
            .iter()
            .zip(prices.iter())
            .map(|(x, y)| x * y)
            .sum();
        let sum_x2: f64 = quantities.iter().map(|x| x * x).sum();

        let denominator = n * sum_x2 - sum_x * sum_x;
        if denominator == 0.0 {
            return None;
        }

        Some((n * sum_xy - sum_x * sum_y) / denominator)
    }

    /// Returns the order book curvature (second derivative).
    pub fn order_book_curvature(&self, side: OrderBookSide, levels: usize) -> Option<f64> {
        if levels < 3 {
            return None;
        }

        // Use finite difference method for curvature
        let slope1 = self.order_book_slope(side, levels - 1)?;
        let slope2 = self.order_book_slope(side, levels)?;

        // Average quantity difference for normalization
        let avg_qty = match side {
            OrderBookSide::Bid => {
                if self.bids.len() < levels {
                    return None;
                }
                self.bids[..levels].iter().map(|b| b.qty).sum::<f64>() / levels as f64
            }
            OrderBookSide::Ask => {
                if self.asks.len() < levels {
                    return None;
                }
                self.asks[..levels].iter().map(|a| a.qty).sum::<f64>() / levels as f64
            }
        };

        if avg_qty == 0.0 {
            return None;
        }

        Some((slope2 - slope1) / avg_qty)
    }

    /// Returns the order book resilience (how quickly liquidity replenishes).
    pub fn order_book_resilience(&self) -> f64 {
        let bid_slope = self.order_book_slope(OrderBookSide::Bid, 5).unwrap_or(0.0);
        let ask_slope = self.order_book_slope(OrderBookSide::Ask, 5).unwrap_or(0.0);

        // Negative slopes indicate decreasing liquidity with price (normal)
        // More negative = less resilient
        let bid_resilience = 1.0 / (1.0 + bid_slope.abs());
        let ask_resilience = 1.0 / (1.0 + ask_slope.abs());

        (bid_resilience + ask_resilience) / 2.0
    }

    /// Returns the order book toxicity (probability of informed trading).
    pub fn order_book_toxicity(&self) -> f64 {
        let imbalance = self.order_book_imbalance().abs();
        let spread_pct = self.spread_percentage().unwrap_or(0.0);

        // Higher imbalance and wider spread indicate higher toxicity
        (imbalance * 0.6 + spread_pct * 100.0 * 0.4).min(1.0)
    }

    /// Returns the effective cost of trading (spread + impact for a given quantity).
    pub fn effective_cost(&self, quantity: f64) -> Option<f64> {
        let spread_pct = self.spread_percentage()?;
        let bid_impact = self.bid_price_impact(quantity).unwrap_or(0.0);
        let ask_impact = self.ask_price_impact(quantity).unwrap_or(0.0);

        Some(spread_pct + (bid_impact + ask_impact) / 2.0)
    }

    /// Returns the market impact cost for a round trip trade.
    pub fn round_trip_cost(&self, quantity: f64) -> Option<f64> {
        let bid_wap = self.weighted_average_bid_price(quantity)?;
        let ask_wap = self.weighted_average_ask_price(quantity)?;

        Some((ask_wap - bid_wap) / bid_wap)
    }

    /// Returns the optimal order size based on market impact.
    pub fn optimal_order_size(&self, max_impact: f64) -> Option<f64> {
        // Binary search for optimal size
        let mut low = 0.0;
        let mut high = self.total_quantity() * 0.1; // Don't exceed 10% of visible liquidity
        let mut best_size = 0.0;

        for _ in 0..20 {
            // 20 iterations should be enough
            let mid = (low + high) / 2.0;
            let impact = self
                .bid_price_impact(mid)
                .unwrap_or(1.0)
                .max(self.ask_price_impact(mid).unwrap_or(1.0));

            if impact <= max_impact {
                best_size = mid;
                low = mid;
            } else {
                high = mid;
            }
        }

        if best_size > 0.0 {
            Some(best_size)
        } else {
            None
        }
    }
}

/// Enum representing order book sides.
#[derive(Debug, Clone, Copy)]
pub enum OrderBookSide {
    Bid,
    Ask,
}
