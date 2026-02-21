use crate::prelude::*;

/// Enum representing order book sides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderBookSide {
    /// Bid side (buy orders)
    Bid,
    /// Ask side (sell orders)
    Ask,
}

/// Enum representing trade sides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeSide {
    /// Buy trade
    Buy,
    /// Sell trade
    Sell,
}

/// Structure representing order book depth profile.
#[derive(Debug, Clone)]
pub struct DepthProfile {
    /// Minimum price in the profile
    pub min_price: f64,
    /// Maximum price in the profile
    pub max_price: f64,
    /// Size of each price bucket
    pub bucket_size: f64,
    /// Quantity in each ask bucket
    pub ask_buckets: Vec<f64>,
    /// Quantity in each bid bucket
    pub bid_buckets: Vec<f64>,
}

/// Structure for WebSocket order book data.
///
/// Contains the bids, asks, and sequence numbers for a trading pair’s order book. Bots use this to maintain an up-to-date view of market depth and liquidity.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WsOrderBook {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract for the order book. Bots use this to verify the correct market.
    #[serde(rename = "s")]
    pub symbol: String,

    /// A list of ask prices and quantities.
    ///
    /// Contains the current ask levels in the order book, sorted by price. Bots use this to assess selling pressure and liquidity on the ask side.
    #[serde(rename = "a")]
    pub asks: Vec<Ask>,

    /// A list of bid prices and quantities.
    ///
    /// Contains the current bid levels in the order book, sorted by price. Bots use this to assess buying pressure and liquidity on the bid side.
    #[serde(rename = "b")]
    pub bids: Vec<Bid>,

    /// The update ID for the order book.
    ///
    /// A unique identifier for the order book update. Bots use this to ensure updates are processed in the correct order.
    #[serde(rename = "u")]
    pub update_id: u64,

    /// The sequence number for the update.
    ///
    /// A monotonically increasing number for ordering updates. Bots use this to detect missing or out-of-order updates and maintain order book consistency.
    pub seq: u64,
}

impl WsOrderBook {
    /// Creates a new WsOrderBook instance.
    pub fn new(symbol: &str, asks: Vec<Ask>, bids: Vec<Bid>, update_id: u64, seq: u64) -> Self {
        Self {
            symbol: symbol.to_string(),
            asks,
            bids,
            update_id,
            seq,
        }
    }

    /// Returns the best ask price (lowest ask).
    pub fn best_ask(&self) -> Option<f64> {
        self.asks.first().map(|ask| ask.price)
    }

    /// Returns the best ask quantity.
    pub fn best_ask_quantity(&self) -> Option<f64> {
        self.asks.first().map(|ask| ask.qty)
    }

    /// Returns the best bid price (highest bid).
    pub fn best_bid(&self) -> Option<f64> {
        self.bids.first().map(|bid| bid.price)
    }

    /// Returns the best bid quantity.
    pub fn best_bid_quantity(&self) -> Option<f64> {
        self.bids.first().map(|bid| bid.qty)
    }

    /// Returns the mid price (average of best bid and best ask).
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some((bid + ask) / 2.0),
            _ => None,
        }
    }

    /// Returns the bid-ask spread.
    pub fn spread(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some(ask - bid),
            _ => None,
        }
    }

    /// Returns the spread as a percentage of the mid price.
    pub fn spread_percentage(&self) -> Option<f64> {
        match (self.spread(), self.mid_price()) {
            (Some(spread), Some(mid)) if mid > 0.0 => Some((spread / mid) * 100.0),
            _ => None,
        }
    }

    /// Returns the total quantity available at the ask side.
    pub fn total_ask_quantity(&self) -> f64 {
        self.asks.iter().map(|ask| ask.qty).sum()
    }

    /// Returns the total quantity available at the bid side.
    pub fn total_bid_quantity(&self) -> f64 {
        self.bids.iter().map(|bid| bid.qty).sum()
    }

    /// Returns the total value available at the ask side.
    pub fn total_ask_value(&self) -> f64 {
        self.asks.iter().map(|ask| ask.price * ask.qty).sum()
    }

    /// Returns the total value available at the bid side.
    pub fn total_bid_value(&self) -> f64 {
        self.bids.iter().map(|bid| bid.price * bid.qty).sum()
    }

    /// Returns the order book imbalance.
    /// Positive values indicate more buying pressure, negative values indicate more selling pressure.
    pub fn imbalance(&self) -> Option<f64> {
        let total_bid = self.total_bid_quantity();
        let total_ask = self.total_ask_quantity();
        let total = total_bid + total_ask;

        if total > 0.0 {
            Some((total_bid - total_ask) / total)
        } else {
            None
        }
    }

    /// Returns the volume-weighted average price (VWAP) for asks.
    pub fn ask_vwap(&self) -> Option<f64> {
        let total_value = self.total_ask_value();
        let total_quantity = self.total_ask_quantity();

        if total_quantity > 0.0 {
            Some(total_value / total_quantity)
        } else {
            None
        }
    }

    /// Returns the volume-weighted average price (VWAP) for bids.
    pub fn bid_vwap(&self) -> Option<f64> {
        let total_value = self.total_bid_value();
        let total_quantity = self.total_bid_quantity();

        if total_quantity > 0.0 {
            Some(total_value / total_quantity)
        } else {
            None
        }
    }

    /// Returns the market depth at a given price level.
    /// Returns (bid_quantity, ask_quantity) at the specified price level.
    pub fn depth_at_price(&self, price: f64, tolerance: f64) -> (f64, f64) {
        let bid_quantity = self
            .bids
            .iter()
            .filter(|bid| (bid.price - price).abs() <= tolerance)
            .map(|bid| bid.qty)
            .sum();

        let ask_quantity = self
            .asks
            .iter()
            .filter(|ask| (ask.price - price).abs() <= tolerance)
            .map(|ask| ask.qty)
            .sum();

        (bid_quantity, ask_quantity)
    }

    /// Returns the cumulative order book quantities up to a given price level.
    pub fn cumulative_depth(&self, price_limit: f64, side: OrderBookSide) -> f64 {
        match side {
            OrderBookSide::Bid => self
                .bids
                .iter()
                .filter(|bid| bid.price >= price_limit)
                .map(|bid| bid.qty)
                .sum(),
            OrderBookSide::Ask => self
                .asks
                .iter()
                .filter(|ask| ask.price <= price_limit)
                .map(|ask| ask.qty)
                .sum(),
        }
    }

    /// Returns the price levels within a given percentage range from the mid price.
    pub fn price_levels_in_range(&self, percentage_range: f64) -> (Vec<&Ask>, Vec<&Bid>) {
        if let Some(mid_price) = self.mid_price() {
            let price_range = mid_price * percentage_range / 100.0;
            let min_price = mid_price - price_range;
            let max_price = mid_price + price_range;

            let filtered_asks = self
                .asks
                .iter()
                .filter(|ask| ask.price <= max_price)
                .collect();

            let filtered_bids = self
                .bids
                .iter()
                .filter(|bid| bid.price >= min_price)
                .collect();

            (filtered_asks, filtered_bids)
        } else {
            (vec![], vec![])
        }
    }

    /// Returns the order book liquidity in a given price range.
    pub fn liquidity_in_range(&self, min_price: f64, max_price: f64) -> (f64, f64) {
        let ask_liquidity = self
            .asks
            .iter()
            .filter(|ask| ask.price >= min_price && ask.price <= max_price)
            .map(|ask| ask.qty)
            .sum();

        let bid_liquidity = self
            .bids
            .iter()
            .filter(|bid| bid.price >= min_price && bid.price <= max_price)
            .map(|bid| bid.qty)
            .sum();

        (ask_liquidity, bid_liquidity)
    }

    /// Returns the price impact for a given trade size.
    /// Estimates how much the price would move if a trade of the given size were executed.
    pub fn price_impact(&self, trade_size: f64, side: TradeSide) -> Option<f64> {
        match side {
            TradeSide::Buy => self.price_impact_for_buy(trade_size),
            TradeSide::Sell => self.price_impact_for_sell(trade_size),
        }
    }

    /// Returns the price impact for a buy trade.
    fn price_impact_for_buy(&self, trade_size: f64) -> Option<f64> {
        let reference_price = self.best_ask()?;
        let mut remaining_size = trade_size;
        let mut total_cost = 0.0;
        let mut executed_quantity = 0.0;

        for ask in &self.asks {
            let quantity_to_take = remaining_size.min(ask.qty);
            total_cost += quantity_to_take * ask.price;
            executed_quantity += quantity_to_take;
            remaining_size -= quantity_to_take;

            if remaining_size <= 0.0 {
                break;
            }
        }

        if executed_quantity > 0.0 {
            let average_price = total_cost / executed_quantity;
            Some((average_price - reference_price) / reference_price * 100.0)
        } else {
            None
        }
    }

    /// Returns the price impact for a sell trade.
    fn price_impact_for_sell(&self, trade_size: f64) -> Option<f64> {
        let reference_price = self.best_bid()?;
        let mut remaining_size = trade_size;
        let mut total_cost = 0.0;
        let mut executed_quantity = 0.0;

        for bid in &self.bids {
            let quantity_to_take = remaining_size.min(bid.qty);
            total_cost += quantity_to_take * bid.price;
            executed_quantity += quantity_to_take;
            remaining_size -= quantity_to_take;

            if remaining_size <= 0.0 {
                break;
            }
        }

        if executed_quantity > 0.0 {
            let average_price = total_cost / executed_quantity;
            Some((average_price - reference_price) / reference_price * 100.0)
        } else {
            None
        }
    }

    /// Returns the market depth profile.
    /// Groups order book levels into price buckets for analysis.
    pub fn depth_profile(&self, num_buckets: usize) -> DepthProfile {
        let (min_price, max_price) = self.price_range();
        let price_range = max_price - min_price;
        let bucket_size = price_range / num_buckets as f64;

        let mut ask_buckets = vec![0.0; num_buckets];
        let mut bid_buckets = vec![0.0; num_buckets];

        // Fill ask buckets
        for ask in &self.asks {
            let bucket_index = ((ask.price - min_price) / bucket_size).floor() as usize;
            if bucket_index < num_buckets {
                ask_buckets[bucket_index] += ask.qty;
            }
        }

        // Fill bid buckets
        for bid in &self.bids {
            let bucket_index = ((bid.price - min_price) / bucket_size).floor() as usize;
            if bucket_index < num_buckets {
                bid_buckets[bucket_index] += bid.qty;
            }
        }

        DepthProfile {
            min_price,
            max_price,
            bucket_size,
            ask_buckets,
            bid_buckets,
        }
    }

    /// Returns the order book's price range.
    pub fn price_range(&self) -> (f64, f64) {
        let min_price = self
            .bids
            .last()
            .map(|bid| bid.price)
            .unwrap_or_else(|| self.asks.first().map(|ask| ask.price).unwrap_or(0.0));

        let max_price = self
            .asks
            .last()
            .map(|ask| ask.price)
            .unwrap_or_else(|| self.bids.first().map(|bid| bid.price).unwrap_or(0.0));

        (min_price, max_price)
    }

    /// Returns the order book's quantity-weighted average spread.
    pub fn weighted_spread(&self) -> Option<f64> {
        let best_ask_quantity = self.best_ask_quantity()?;
        let best_bid_quantity = self.best_bid_quantity()?;
        let spread = self.spread()?;

        let total_quantity = best_ask_quantity + best_bid_quantity;
        if total_quantity > 0.0 {
            let ask_weight = best_ask_quantity / total_quantity;
            let bid_weight = best_bid_quantity / total_quantity;

            // Weighted spread gives more weight to the side with more liquidity
            Some(spread * (ask_weight + bid_weight) / 2.0)
        } else {
            None
        }
    }

    /// Returns the order book's resilience.
    /// Measures how quickly the order book recovers after a trade.
    pub fn resilience(&self) -> f64 {
        let depth_ratio = self.total_bid_quantity() / self.total_ask_quantity().max(1.0);
        let spread_ratio = self.spread_percentage().unwrap_or(0.0) / 0.1; // Normalize to 0.1% spread

        // Higher depth ratio and lower spread indicate more resilience
        depth_ratio / (1.0 + spread_ratio)
    }

    /// Returns the order book's toxicity.
    /// Measures the likelihood of adverse selection.
    pub fn toxicity(&self) -> f64 {
        let imbalance = self.imbalance().unwrap_or(0.0).abs();
        let spread = self.spread_percentage().unwrap_or(0.0);

        // Higher imbalance and lower spread indicate higher toxicity
        imbalance / (1.0 + spread)
    }

    /// Returns true if the order book is valid for trading.
    pub fn is_valid(&self) -> bool {
        !self.symbol.is_empty()
            && !self.asks.is_empty()
            && !self.bids.is_empty()
            && self.best_ask().is_some()
            && self.best_bid().is_some()
            && self.best_ask().unwrap_or(0.0) > 0.0
            && self.best_bid().unwrap_or(0.0) > 0.0
            && self.best_ask().unwrap_or(f64::MAX) > self.best_bid().unwrap_or(0.0)
    }

    /// Returns a summary string for this order book.
    pub fn to_summary_string(&self) -> String {
        let best_bid = self.best_bid().unwrap_or(0.0);
        let best_ask = self.best_ask().unwrap_or(0.0);
        let spread = self.spread().unwrap_or(0.0);
        let spread_pct = self.spread_percentage().unwrap_or(0.0);
        let mid_price = self.mid_price().unwrap_or(0.0);
        let imbalance = self.imbalance().unwrap_or(0.0);

        format!(
            "{}: Bid={:.2}, Ask={:.2}, Spread={:.4} ({:.4}%), Mid={:.2}, Imbalance={:.2}, Levels={}/{}",
            self.symbol,
            best_bid,
            best_ask,
            spread,
            spread_pct,
            mid_price,
            imbalance,
            self.bids.len(),
            self.asks.len()
        )
    }

    /// Returns the order book snapshot as a JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Merges this order book with another order book update.
    /// Used for incremental updates.
    pub fn merge(&mut self, update: &WsOrderBook) -> bool {
        if self.symbol != update.symbol {
            return false;
        }

        if update.seq <= self.seq {
            return false; // Old update
        }

        // Update sequence numbers
        self.update_id = update.update_id;
        self.seq = update.seq;

        // For simplicity, replace the entire order book
        // In a real implementation, you would apply incremental updates
        self.asks = update.asks.clone();
        self.bids = update.bids.clone();

        true
    }

    /// Returns the order book age based on sequence numbers.
    pub fn age(&self, current_seq: u64) -> u64 {
        if current_seq >= self.seq {
            current_seq - self.seq
        } else {
            0
        }
    }

    /// Returns true if the order book is stale.
    pub fn is_stale(&self, current_seq: u64, max_age: u64) -> bool {
        self.age(current_seq) > max_age
    }

    /// Returns the order book's market quality score.
    /// Higher scores indicate better market quality (more liquidity, tighter spreads).
    pub fn market_quality_score(&self) -> f64 {
        let mut score = 0.0;
        let mut weight_sum = 0.0;

        // Spread component (tighter spreads are better)
        if let Some(spread_pct) = self.spread_percentage() {
            let spread_score = 1.0 / (1.0 + spread_pct * 10.0); // Normalize
            score += spread_score * 0.4;
            weight_sum += 0.4;
        }

        // Depth component (more depth is better)
        let total_depth = self.total_bid_quantity() + self.total_ask_quantity();
        let depth_score = (total_depth / 1000.0).min(1.0); // Normalize to 1000 units
        score += depth_score * 0.3;
        weight_sum += 0.3;

        // Imbalance component (balanced order book is better)
        if let Some(imbalance) = self.imbalance() {
            let imbalance_score = 1.0 - imbalance.abs();
            score += imbalance_score * 0.2;
            weight_sum += 0.2;
        }

        // Resilience component
        let resilience_score = self.resilience().min(1.0);
        score += resilience_score * 0.1;
        weight_sum += 0.1;

        if weight_sum > 0.0 {
            score / weight_sum * 100.0 // Scale to 0-100
        } else {
            0.0
        }
    }

    /// Returns the order book's estimated transaction cost.
    /// This includes both the spread and potential price impact.
    pub fn estimated_transaction_cost(&self, trade_size: f64) -> Option<f64> {
        let spread_cost = self.spread_percentage()?;
        let buy_impact = self.price_impact(trade_size, TradeSide::Buy).unwrap_or(0.0);
        let sell_impact = self
            .price_impact(trade_size, TradeSide::Sell)
            .unwrap_or(0.0);

        // Average of buy and sell impact plus half the spread (one-way cost)
        Some((buy_impact + sell_impact) / 2.0 + spread_cost / 2.0)
    }

    /// Returns the optimal trade size based on market conditions.
    /// Considers price impact and available liquidity.
    pub fn optimal_trade_size(&self, max_price_impact: f64) -> Option<f64> {
        let mut size = 0.0;
        let mut step = self.total_bid_quantity().min(self.total_ask_quantity()) / 10.0;

        // Binary search for optimal size
        for _ in 0..10 {
            let buy_impact = self
                .price_impact(size + step, TradeSide::Buy)
                .unwrap_or(0.0);
            let sell_impact = self
                .price_impact(size + step, TradeSide::Sell)
                .unwrap_or(0.0);
            let avg_impact = (buy_impact + sell_impact) / 2.0;

            if avg_impact <= max_price_impact {
                size += step;
            } else {
                step /= 2.0;
            }
        }

        if size > 0.0 {
            Some(size)
        } else {
            None
        }
    }

    /// Returns the order book's support and resistance levels.
    /// Based on significant accumulation of orders.
    pub fn support_resistance_levels(&self, threshold_multiplier: f64) -> (Vec<f64>, Vec<f64>) {
        let avg_quantity = (self.total_bid_quantity() + self.total_ask_quantity())
            / (self.bids.len() + self.asks.len()) as f64;
        let threshold = avg_quantity * threshold_multiplier;

        let support_levels: Vec<f64> = self
            .bids
            .iter()
            .filter(|bid| bid.qty >= threshold)
            .map(|bid| bid.price)
            .collect();

        let resistance_levels: Vec<f64> = self
            .asks
            .iter()
            .filter(|ask| ask.qty >= threshold)
            .map(|ask| ask.price)
            .collect();

        (support_levels, resistance_levels)
    }

    /// Returns the order book's momentum indicator.
    /// Positive values indicate buying pressure, negative values indicate selling pressure.
    pub fn momentum_indicator(&self) -> f64 {
        let top_bid_quantity = self.bids.first().map(|b| b.qty).unwrap_or(0.0);
        let top_ask_quantity = self.asks.first().map(|a| a.qty).unwrap_or(0.0);
        let total_top_quantity = top_bid_quantity + top_ask_quantity;

        if total_top_quantity > 0.0 {
            (top_bid_quantity - top_ask_quantity) / total_top_quantity
        } else {
            0.0
        }
    }

    /// Returns the order book's volatility estimate.
    /// Based on the density of orders around the mid price.
    pub fn volatility_estimate(&self) -> Option<f64> {
        let mid_price = self.mid_price()?;
        let price_range = 0.01; // 1% price range

        let (ask_liquidity, bid_liquidity) = self.liquidity_in_range(
            mid_price * (1.0 - price_range),
            mid_price * (1.0 + price_range),
        );

        let total_liquidity = ask_liquidity + bid_liquidity;
        let max_possible_liquidity = self.total_ask_quantity() + self.total_bid_quantity();

        if max_possible_liquidity > 0.0 {
            // Lower concentration around mid price indicates higher volatility
            Some(1.0 - (total_liquidity / max_possible_liquidity))
        } else {
            None
        }
    }

    /// Returns the order book's efficiency metric.
    /// Measures how well the order book facilitates trading.
    pub fn efficiency_metric(&self) -> Option<f64> {
        let spread_pct = self.spread_percentage()?;
        let depth_ratio = self.total_bid_quantity() / self.total_ask_quantity().max(1.0);
        let imbalance = self.imbalance()?.abs();

        // Efficiency increases with lower spread, balanced depth, and lower imbalance
        let spread_component = 1.0 / (1.0 + spread_pct * 100.0);
        let depth_component = 2.0 * depth_ratio.min(1.0) / (1.0 + depth_ratio);
        let imbalance_component = 1.0 - imbalance;

        Some((spread_component + depth_component + imbalance_component) / 3.0 * 100.0)
    }

    /// Returns the order book's fair value estimate.
    /// Based on volume-weighted average prices and order book imbalance.
    pub fn fair_value_estimate(&self) -> Option<f64> {
        let bid_vwap = self.bid_vwap()?;
        let ask_vwap = self.ask_vwap()?;
        let imbalance = self.imbalance()?;

        // Weighted average based on order book imbalance
        let bid_weight = (1.0 + imbalance) / 2.0;
        let ask_weight = (1.0 - imbalance) / 2.0;

        Some(bid_vwap * bid_weight + ask_vwap * ask_weight)
    }

    /// Returns the order book's arbitrage opportunity.
    /// Difference between fair value and mid price as percentage.
    pub fn arbitrage_opportunity(&self) -> Option<f64> {
        let fair_value = self.fair_value_estimate()?;
        let mid_price = self.mid_price()?;

        if mid_price > 0.0 {
            Some((fair_value - mid_price) / mid_price * 100.0)
        } else {
            None
        }
    }

    /// Returns the order book's market impact profile.
    /// Shows how price impact changes with trade size.
    pub fn market_impact_profile(&self, max_trade_size: f64, steps: usize) -> Vec<(f64, f64)> {
        let mut profile = Vec::with_capacity(steps);
        let step_size = max_trade_size / steps as f64;

        for i in 1..=steps {
            let trade_size = step_size * i as f64;
            if let Some(impact) = self.estimated_transaction_cost(trade_size) {
                profile.push((trade_size, impact));
            }
        }

        profile
    }

    /// Returns the order book's snapshot for persistence.
    pub fn snapshot(&self) -> OrderBookSnapshot {
        OrderBookSnapshot {
            symbol: self.symbol.clone(),
            bids: self.bids.clone(),
            asks: self.asks.clone(),
            update_id: self.update_id,
            seq: self.seq,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    /// Returns a comprehensive analysis report.
    pub fn analysis_report(&self) -> String {
        let mut report = String::new();

        report.push_str(&format!("Order Book Analysis: {}\n", self.symbol));
        report.push_str(&format!("================================\n"));

        // Basic metrics
        if let (Some(bid), Some(ask)) = (self.best_bid(), self.best_ask()) {
            report.push_str(&format!("Best Bid: {:.8}\n", bid));
            report.push_str(&format!("Best Ask: {:.8}\n", ask));
        }

        if let Some(spread) = self.spread() {
            report.push_str(&format!("Spread: {:.8}\n", spread));
        }

        if let Some(spread_pct) = self.spread_percentage() {
            report.push_str(&format!("Spread %: {:.4}%\n", spread_pct));
        }

        if let Some(mid) = self.mid_price() {
            report.push_str(&format!("Mid Price: {:.8}\n", mid));
        }

        // Liquidity metrics
        report.push_str(&format!("Bid Levels: {}\n", self.bids.len()));
        report.push_str(&format!("Ask Levels: {}\n", self.asks.len()));
        report.push_str(&format!(
            "Total Bid Qty: {:.8}\n",
            self.total_bid_quantity()
        ));
        report.push_str(&format!(
            "Total Ask Qty: {:.8}\n",
            self.total_ask_quantity()
        ));
        report.push_str(&format!("Total Bid Value: {:.8}\n", self.total_bid_value()));
        report.push_str(&format!("Total Ask Value: {:.8}\n", self.total_ask_value()));

        // Advanced metrics
        if let Some(imbalance) = self.imbalance() {
            report.push_str(&format!("Order Book Imbalance: {:.2}\n", imbalance));
        }

        if let Some(bid_vwap) = self.bid_vwap() {
            report.push_str(&format!("Bid VWAP: {:.8}\n", bid_vwap));
        }

        if let Some(ask_vwap) = self.ask_vwap() {
            report.push_str(&format!("Ask VWAP: {:.8}\n", ask_vwap));
        }

        let momentum = self.momentum_indicator();
        report.push_str(&format!("Momentum Indicator: {:.4}\n", momentum));

        let resilience = self.resilience();
        report.push_str(&format!("Resilience Score: {:.4}\n", resilience));

        let toxicity = self.toxicity();
        report.push_str(&format!("Toxicity Score: {:.4}\n", toxicity));

        let quality_score = self.market_quality_score();
        report.push_str(&format!("Market Quality Score: {:.1}/100\n", quality_score));

        if let Some(efficiency) = self.efficiency_metric() {
            report.push_str(&format!("Efficiency Metric: {:.1}/100\n", efficiency));
        }

        if let Some(fair_value) = self.fair_value_estimate() {
            report.push_str(&format!("Fair Value Estimate: {:.8}\n", fair_value));
        }

        if let Some(arb_opp) = self.arbitrage_opportunity() {
            report.push_str(&format!("Arbitrage Opportunity: {:.4}%\n", arb_opp));
        }

        // Price impact examples
        report.push_str("\nPrice Impact Analysis:\n");
        for &size in &[0.1, 1.0, 10.0, 100.0] {
            if let Some(cost) = self.estimated_transaction_cost(size) {
                report.push_str(&format!("  Size {:.1}: {:.4}% cost\n", size, cost));
            }
        }

        // Support and resistance
        let (support, resistance) = self.support_resistance_levels(2.0);
        if !support.is_empty() {
            report.push_str(&format!("\nSupport Levels ({}):\n", support.len()));
            for &level in &support[..support.len().min(5)] {
                report.push_str(&format!("  {:.8}\n", level));
            }
        }

        if !resistance.is_empty() {
            report.push_str(&format!("\nResistance Levels ({}):\n", resistance.len()));
            for &level in &resistance[..resistance.len().min(5)] {
                report.push_str(&format!("  {:.8}\n", level));
            }
        }

        // Recommendations
        report.push_str("\nRecommendations:\n");
        if quality_score >= 70.0 {
            report.push_str("  ✅ Good market conditions for trading\n");
        } else if quality_score >= 40.0 {
            report.push_str("  ⚠️  Moderate market conditions\n");
        } else {
            report.push_str("  ❌ Poor market conditions\n");
        }

        if let Some(spread_pct) = self.spread_percentage() {
            if spread_pct < 0.1 {
                report.push_str("  ✅ Tight spreads\n");
            } else if spread_pct < 0.5 {
                report.push_str("  ⚠️  Moderate spreads\n");
            } else {
                report.push_str("  ❌ Wide spreads\n");
            }
        }

        let imbalance = self.imbalance().unwrap_or(0.0);
        if imbalance.abs() < 0.1 {
            report.push_str("  ✅ Balanced order book\n");
        } else if imbalance.abs() < 0.3 {
            report.push_str("  ⚠️  Moderate imbalance\n");
        } else {
            report.push_str("  ❌ Significant imbalance\n");
        }

        report
    }
}

/// Structure for persisting order book snapshots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookSnapshot {
    /// Trading symbol
    pub symbol: String,
    /// Bid levels
    pub bids: Vec<Bid>,
    /// Ask levels
    pub asks: Vec<Ask>,
    /// Update ID
    pub update_id: u64,
    /// Sequence number
    pub seq: u64,
    /// Timestamp when snapshot was taken
    pub timestamp: u64,
}

impl OrderBookSnapshot {
    /// Creates a new OrderBookSnapshot from a WsOrderBook.
    pub fn from_order_book(order_book: &WsOrderBook) -> Self {
        Self {
            symbol: order_book.symbol.clone(),
            bids: order_book.bids.clone(),
            asks: order_book.asks.clone(),
            update_id: order_book.update_id,
            seq: order_book.seq,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    /// Returns the age of the snapshot in milliseconds.
    pub fn age_ms(&self) -> u64 {
        let now = chrono::Utc::now().timestamp_millis() as u64;
        if now > self.timestamp {
            now - self.timestamp
        } else {
            0
        }
    }

    /// Returns true if the snapshot is stale.
    pub fn is_stale(&self, max_age_ms: u64) -> bool {
        self.age_ms() > max_age_ms
    }

    /// Converts the snapshot back to a WsOrderBook.
    pub fn to_order_book(&self) -> WsOrderBook {
        WsOrderBook {
            symbol: self.symbol.clone(),
            asks: self.asks.clone(),
            bids: self.bids.clone(),
            update_id: self.update_id,
            seq: self.seq,
        }
    }
}
