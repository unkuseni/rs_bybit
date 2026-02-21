use crate::prelude::*;

/// Parameters for requesting order book data.
///
/// This struct defines the parameters for querying the order book via the `/v5/market/orderbook` endpoint.
/// The order book shows current bid and ask prices and quantities, critical for liquidity analysis
/// and order placement in perpetual futures.
#[derive(Clone, Default)]
pub struct OrderbookRequest<'a> {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract. Bots must specify a valid symbol to fetch the correct order book.
    pub symbol: Cow<'a, str>,

    /// The product category (e.g., Linear, Inverse, Spot, Option).
    ///
    /// Specifies the instrument type. For perpetual futures, use `Linear` or `Inverse`.
    /// Bots must set this correctly to avoid errors.
    pub category: Category,

    /// The maximum number of order book levels to return.
    ///
    /// Controls the depth of the order book (number of bid/ask levels). The valid range depends on the category:
    /// - `spot`: [1, 200], default: 1
    /// - `linear` & `inverse`: [1, 500], default: 25
    /// - `option`: [1, 25], default: 1
    ///
    /// Bots should balance depth with performance: deeper books provide more liquidity data
    /// but increase latency and memory usage.
    pub limit: Option<u64>,
}

impl<'a> OrderbookRequest<'a> {
    /// Creates a default Orderbook request.
    ///
    /// Returns a request with `symbol` set to `"BTCUSDT"` and `category` set to `Linear`.
    /// Suitable for testing but should be customized for production.
    pub fn default() -> OrderbookRequest<'a> {
        OrderbookRequest::new("BTCUSDT", Category::Linear, None)
    }

    /// Constructs a new Orderbook request with specified parameters.
    ///
    /// Allows customization. Bots should use this to specify the exact symbol and category
    /// for their perpetual futures strategy.
    pub fn new(symbol: &'a str, category: Category, limit: Option<u64>) -> OrderbookRequest<'a> {
        OrderbookRequest {
            symbol: Cow::Borrowed(symbol),
            category,
            limit,
        }
    }

    /// Validates the request parameters according to API constraints.
    ///
    /// Returns `Ok(())` if the request is valid, or `Err(String)` with an error message.
    pub fn validate(&self) -> Result<(), String> {
        // Validate limit range based on category
        if let Some(limit) = self.limit {
            let (min, max) = match self.category {
                Category::Spot => (1, 200),
                Category::Linear | Category::Inverse => (1, 500),
                Category::Option => (1, 25),
            };

            if limit < min || limit > max {
                return Err(format!(
                    "Limit for category {} must be between {} and {} inclusive",
                    self.category.as_str(),
                    min,
                    max
                ));
            }
        }

        Ok(())
    }

    /// Gets the default limit value for the category.
    ///
    /// Returns the default limit value based on the category:
    /// - `spot`: 1
    /// - `linear` & `inverse`: 25
    /// - `option`: 1
    pub fn default_limit(&self) -> u64 {
        match self.category {
            Category::Spot => 1,
            Category::Linear | Category::Inverse => 25,
            Category::Option => 1,
        }
    }

    /// Gets the effective limit value (either the specified limit or the default).
    ///
    /// Returns the limit value to use for the API request.
    pub fn effective_limit(&self) -> u64 {
        self.limit.unwrap_or_else(|| self.default_limit())
    }

    /// Creates a request for spot trading pairs.
    ///
    /// Convenience method for creating requests for spot markets.
    pub fn spot(symbol: &'a str, limit: Option<u64>) -> OrderbookRequest<'a> {
        OrderbookRequest::new(symbol, Category::Spot, limit)
    }

    /// Creates a request for linear perpetual futures.
    ///
    /// Convenience method for creating requests for USDT-margined perpetual futures.
    pub fn linear(symbol: &'a str, limit: Option<u64>) -> OrderbookRequest<'a> {
        OrderbookRequest::new(symbol, Category::Linear, limit)
    }

    /// Creates a request for inverse perpetual futures.
    ///
    /// Convenience method for creating requests for coin-margined perpetual futures.
    pub fn inverse(symbol: &'a str, limit: Option<u64>) -> OrderbookRequest<'a> {
        OrderbookRequest::new(symbol, Category::Inverse, limit)
    }

    /// Creates a request for options contracts.
    ///
    /// Convenience method for creating requests for options markets.
    pub fn option(symbol: &'a str, limit: Option<u64>) -> OrderbookRequest<'a> {
        OrderbookRequest::new(symbol, Category::Option, limit)
    }
}
