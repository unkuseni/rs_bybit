use crate::prelude::*;

/// Parameters for requesting order price limits.
///
/// This struct defines the parameters for querying order price limits via Bybit's `/v5/market/price-limit` endpoint.
/// Order price limits define the highest bid price (buyLmt) and lowest ask price (sellLmt) for a given symbol.
#[derive(Clone, Default)]
pub struct OrderPriceLimitRequest<'a> {
    /// The product category (e.g., Spot, Linear, Inverse).
    ///
    /// Specifies the type of instrument for the price limit data.
    /// Valid values: `spot`, `linear`, `inverse`.
    /// If not specified, defaults to `linear`.
    pub category: Option<Category>,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the specific instrument for which to retrieve price limits.
    /// Must be uppercase (e.g., "BTCUSDT", not "btcusdt").
    pub symbol: Cow<'a, str>,
}

impl<'a> OrderPriceLimitRequest<'a> {
    /// Creates a default order price limit request for BTCUSDT.
    ///
    /// Returns an `OrderPriceLimitRequest` with `symbol` set to `"BTCUSDT"` and no category specified.
    /// This will default to `linear` category when sent to the API.
    pub fn default() -> OrderPriceLimitRequest<'a> {
        OrderPriceLimitRequest::new(None, "BTCUSDT")
    }

    /// Constructs a new order price limit request with specified parameters.
    ///
    /// # Arguments
    /// * `category` - Optional product category (spot, linear, inverse)
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSDT")
    pub fn new(category: Option<Category>, symbol: &'a str) -> OrderPriceLimitRequest<'a> {
        OrderPriceLimitRequest {
            category,
            symbol: Cow::Borrowed(symbol),
        }
    }

    /// Constructs a new order price limit request for linear perpetual contracts.
    ///
    /// # Arguments
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSDT")
    pub fn linear(symbol: &'a str) -> OrderPriceLimitRequest<'a> {
        OrderPriceLimitRequest::new(Some(Category::Linear), symbol)
    }

    /// Constructs a new order price limit request for inverse perpetual contracts.
    ///
    /// # Arguments
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSD")
    pub fn inverse(symbol: &'a str) -> OrderPriceLimitRequest<'a> {
        OrderPriceLimitRequest::new(Some(Category::Inverse), symbol)
    }

    /// Constructs a new order price limit request for spot trading.
    ///
    /// # Arguments
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSDT")
    pub fn spot(symbol: &'a str) -> OrderPriceLimitRequest<'a> {
        OrderPriceLimitRequest::new(Some(Category::Spot), symbol)
    }
}
