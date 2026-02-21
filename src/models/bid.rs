use crate::prelude::*;

/// Represents a bid (buy) order in the order book.
///
/// Each bid contains a price and quantity at which buyers are willing to buy. Bots use this to assess buying support and optimize order placement.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    /// The bid price.
    ///
    /// The price at which the buyer is offering to buy. Bots use this to determine support levels and calculate slippage for sell orders in perpetual futures.
    #[serde(with = "string_to_float")]
    pub price: f64,

    /// The bid quantity.
    ///
    /// The quantity available at the bid price. Bots use this to assess liquidity and estimate the impact of large sell orders on the market.
    #[serde(with = "string_to_float")]
    pub qty: f64,
}

impl Bid {
    /// Constructs a new Bid with specified price and quantity.
    ///
    /// Useful for testing or simulating order book data. Bots typically receive this from the API rather than constructing it manually.
    pub fn new(price: f64, qty: f64) -> Bid {
        Bid { price, qty }
    }

    /// Returns the notional value (price × quantity).
    pub fn notional_value(&self) -> f64 {
        self.price * self.qty
    }

    /// Returns the inverse price (1/price).
    pub fn inverse_price(&self) -> f64 {
        1.0 / self.price
    }

    /// Returns the log price (natural logarithm of price).
    pub fn log_price(&self) -> f64 {
        self.price.ln()
    }

    /// Returns the price rounded to a given number of decimal places.
    pub fn price_rounded(&self, decimals: usize) -> f64 {
        let factor = 10_f64.powi(decimals as i32);
        (self.price * factor).round() / factor
    }

    /// Returns whether this bid is better (higher) than another price.
    pub fn is_better_than(&self, other_price: f64) -> bool {
        self.price > other_price
    }

    /// Returns the price difference from a reference price.
    pub fn price_difference(&self, reference_price: f64) -> f64 {
        self.price - reference_price
    }

    /// Returns the percentage difference from a reference price.
    pub fn price_percentage_difference(&self, reference_price: f64) -> f64 {
        if reference_price == 0.0 {
            return 0.0;
        }
        (self.price - reference_price) / reference_price
    }

    /// Returns the quantity in quote currency terms.
    pub fn quote_quantity(&self) -> f64 {
        self.notional_value()
    }

    /// Returns the effective price including a fee percentage.
    pub fn effective_price_with_fee(&self, fee_percentage: f64) -> f64 {
        self.price * (1.0 - fee_percentage)
    }

    /// Returns the total cost including fees for this order.
    pub fn total_cost_with_fee(&self, fee_percentage: f64) -> f64 {
        self.notional_value() * (1.0 - fee_percentage)
    }

    /// Returns the slippage if this order were executed against a better price.
    pub fn slippage_vs_better_price(&self, better_price: f64) -> f64 {
        if better_price > self.price {
            better_price - self.price
        } else {
            0.0
        }
    }

    /// Returns the percentage slippage if this order were executed against a better price.
    pub fn slippage_percentage_vs_better_price(&self, better_price: f64) -> f64 {
        if better_price > self.price && self.price > 0.0 {
            (better_price - self.price) / self.price
        } else {
            0.0
        }
    }

    /// Returns whether this bid would be profitable if sold and bought at target price.
    pub fn would_be_profitable(&self, target_buy_price: f64, fee_percentage: f64) -> bool {
        let sell_revenue = self.total_cost_with_fee(fee_percentage);
        let buy_cost = target_buy_price * self.qty * (1.0 + fee_percentage);
        sell_revenue > buy_cost
    }

    /// Returns the profit margin if sold at this price and bought at target price.
    pub fn profit_margin(&self, target_buy_price: f64, fee_percentage: f64) -> f64 {
        let sell_revenue = self.total_cost_with_fee(fee_percentage);
        let buy_cost = target_buy_price * self.qty * (1.0 + fee_percentage);
        if buy_cost == 0.0 {
            return 0.0;
        }
        (sell_revenue - buy_cost) / buy_cost
    }

    /// Returns the break-even buy price needed to cover costs.
    pub fn break_even_price(&self, fee_percentage: f64) -> f64 {
        self.price * (1.0 - fee_percentage) / (1.0 + fee_percentage)
    }

    /// Returns the price impact if this order were added to the order book.
    pub fn price_impact_if_added(&self, current_best_bid: f64, current_bid_qty: f64) -> f64 {
        if self.price > current_best_bid {
            // This would become the new best bid
            (self.price - current_best_bid) / current_best_bid
        } else if self.price == current_best_bid {
            // This adds to existing best bid
            self.qty / (current_bid_qty + self.qty)
        } else {
            0.0
        }
    }

    /// Returns the liquidity contribution of this order.
    pub fn liquidity_contribution(&self, total_bid_liquidity: f64) -> f64 {
        if total_bid_liquidity == 0.0 {
            return 1.0;
        }
        self.qty / total_bid_liquidity
    }

    /// Returns the time-weighted value (assuming constant over time).
    pub fn time_weighted_value(&self, time_weight: f64) -> f64 {
        self.notional_value() * time_weight
    }

    /// Returns a scaled version of this bid (multiplies quantity by factor).
    pub fn scaled(&self, factor: f64) -> Bid {
        Bid {
            price: self.price,
            qty: self.qty * factor,
        }
    }

    /// Returns whether this bid is valid (positive price and quantity).
    pub fn is_valid(&self) -> bool {
        self.price > 0.0 && self.qty > 0.0
    }

    /// Returns whether this bid is within a percentage range of a reference price.
    pub fn is_within_percentage_range(&self, reference_price: f64, percentage: f64) -> bool {
        let diff = self.price_percentage_difference(reference_price).abs();
        diff <= percentage / 100.0
    }

    /// Returns the distance from a reference price in ticks (assuming tick size).
    pub fn distance_in_ticks(&self, reference_price: f64, tick_size: f64) -> i64 {
        if tick_size == 0.0 {
            return 0;
        }
        ((self.price - reference_price) / tick_size).round() as i64
    }
}
