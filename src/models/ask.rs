use crate::prelude::*;

/// Represents an ask (sell) order in the order book.
///
/// Each ask contains a price and quantity at which sellers are willing to sell. Bots use this to assess selling pressure and optimize order placement.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ask {
    /// The ask price.
    ///
    /// The price at which the seller is offering to sell. Bots use this to determine resistance levels and calculate slippage for buy orders in perpetual futures.
    #[serde(with = "string_to_float")]
    pub price: f64,

    /// The ask quantity.
    ///
    /// The quantity available at the ask price. Bots use this to assess liquidity and estimate the impact of large buy orders on the market.
    #[serde(with = "string_to_float")]
    pub qty: f64,
}

impl Ask {
    /// Constructs a new Ask with specified price and quantity.
    ///
    /// Useful for testing or simulating order book data. Bots typically receive this from the API rather than constructing it manually.
    pub fn new(price: f64, qty: f64) -> Ask {
        Ask { price, qty }
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

    /// Returns whether this ask is better (lower) than another price.
    pub fn is_better_than(&self, other_price: f64) -> bool {
        self.price < other_price
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
        self.price * (1.0 + fee_percentage)
    }

    /// Returns the total cost including fees for this order.
    pub fn total_cost_with_fee(&self, fee_percentage: f64) -> f64 {
        self.notional_value() * (1.0 + fee_percentage)
    }

    /// Returns the slippage if this order were executed against a better price.
    pub fn slippage_vs_better_price(&self, better_price: f64) -> f64 {
        if better_price < self.price {
            self.price - better_price
        } else {
            0.0
        }
    }

    /// Returns the percentage slippage if this order were executed against a better price.
    pub fn slippage_percentage_vs_better_price(&self, better_price: f64) -> f64 {
        if better_price < self.price && better_price > 0.0 {
            (self.price - better_price) / better_price
        } else {
            0.0
        }
    }

    /// Returns whether this ask would be profitable if bought and sold at target price.
    pub fn would_be_profitable(&self, target_sell_price: f64, fee_percentage: f64) -> bool {
        let buy_cost = self.total_cost_with_fee(fee_percentage);
        let sell_revenue = target_sell_price * self.qty * (1.0 - fee_percentage);
        sell_revenue > buy_cost
    }

    /// Returns the profit margin if bought at this price and sold at target price.
    pub fn profit_margin(&self, target_sell_price: f64, fee_percentage: f64) -> f64 {
        let buy_cost = self.total_cost_with_fee(fee_percentage);
        let sell_revenue = target_sell_price * self.qty * (1.0 - fee_percentage);
        if buy_cost == 0.0 {
            return 0.0;
        }
        (sell_revenue - buy_cost) / buy_cost
    }

    /// Returns the break-even sell price needed to cover costs.
    pub fn break_even_price(&self, fee_percentage: f64) -> f64 {
        self.price * (1.0 + fee_percentage) / (1.0 - fee_percentage)
    }

    /// Returns the price impact if this order were added to the order book.
    pub fn price_impact_if_added(&self, current_best_ask: f64, current_ask_qty: f64) -> f64 {
        if self.price < current_best_ask {
            // This would become the new best ask
            (current_best_ask - self.price) / current_best_ask
        } else if self.price == current_best_ask {
            // This adds to existing best ask
            self.qty / (current_ask_qty + self.qty)
        } else {
            0.0
        }
    }

    /// Returns the liquidity contribution of this order.
    pub fn liquidity_contribution(&self, total_ask_liquidity: f64) -> f64 {
        if total_ask_liquidity == 0.0 {
            return 1.0;
        }
        self.qty / total_ask_liquidity
    }

    /// Returns the time-weighted value (assuming constant over time).
    pub fn time_weighted_value(&self, time_weight: f64) -> f64 {
        self.notional_value() * time_weight
    }

    /// Returns a scaled version of this ask (multiplies quantity by factor).
    pub fn scaled(&self, factor: f64) -> Ask {
        Ask {
            price: self.price,
            qty: self.qty * factor,
        }
    }

    /// Returns whether this ask is valid (positive price and quantity).
    pub fn is_valid(&self) -> bool {
        self.price > 0.0 && self.qty > 0.0
    }

    /// Returns whether this ask is within a percentage range of a reference price.
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
