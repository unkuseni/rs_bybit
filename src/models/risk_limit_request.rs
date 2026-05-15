use crate::prelude::*;

/// Represents a request to fetch risk limit data for a trading category on Bybit.
/// Risk limits define the maximum position size and leverage for a trader, critical for
/// managing exposure in perpetual futures.
#[derive(Clone, Default)]
pub struct RiskLimitRequest<'a> {
    /// The trading category (e.g., Linear for USDT-margined perpetuals).
    /// Determines the type of futures contract. Bots must ensure this matches the trading pair.
    pub category: Category,

    /// The trading symbol (e.g., "BTCUSDT").
    /// Specifies the market for which risk limits are queried. Optional, as some endpoints allow
    /// category-wide queries. Bots should validate symbols to avoid API errors.
    pub symbol: Option<Cow<'a, str>>,
}

impl<'a> RiskLimitRequest<'a> {
    /// Creates a default request for the Linear category.
    /// Simplifies initialization for bots targeting USDT-margined perpetuals.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> RiskLimitRequest<'a> {
        RiskLimitRequest::new(Category::Linear, None)
    }
    /// Constructs a new request with specified category and symbol.
    /// Enables precise queries for risk limits, critical for bots managing leverage and position
    /// sizes in perpetual futures.
    pub fn new(category: Category, symbol: Option<&'a str>) -> RiskLimitRequest<'a> {
        RiskLimitRequest {
            category,
            symbol: symbol.map(Cow::Borrowed),
        }
    }
}
