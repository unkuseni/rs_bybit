use crate::prelude::*;

/// Parameters for requesting RPI (Real-time Price Improvement) order book data.
///
/// This struct defines the parameters for querying the RPI order book via the `/v5/market/rpi_orderbook` endpoint.
/// The RPI order book shows both regular orders and RPI orders, which can provide price improvement for takers.
/// RPI orders are special orders that can improve prices when they cross with non-RPI orders.
#[derive(Clone, Default)]
pub struct RPIOrderbookRequest<'a> {
    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the trading pair. Bots must specify a valid symbol to fetch the correct RPI order book.
    /// This parameter is required.
    pub symbol: Cow<'a, str>,

    /// The product category (e.g., spot, linear, inverse).
    ///
    /// Specifies the instrument type. According to the Bybit API documentation, this parameter is optional.
    /// If not specified, the API will return data for the appropriate category based on the symbol.
    pub category: Option<Category>,

    /// The maximum number of order book levels to return for each side (1-50).
    ///
    /// Controls the depth of the RPI order book (number of bid/ask levels).
    /// This parameter is required and must be between 1 and 50 inclusive.
    /// RPI order books are limited to 50 levels maximum.
    pub limit: u64,
}

impl<'a> RPIOrderbookRequest<'a> {
    /// Creates a default RPIOrderbook request.
    ///
    /// Returns a request with `symbol` set to `"BTCUSDT"`, no category, and `limit` set to 50.
    /// Suitable for testing but should be customized for production.
    pub fn default() -> RPIOrderbookRequest<'a> {
        RPIOrderbookRequest::new("BTCUSDT", None, 50)
    }

    /// Constructs a new RPIOrderbook request with specified parameters.
    ///
    /// Allows customization. Bots should use this to specify the exact symbol, optional category, and limit
    /// for their RPI order book requests.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSDT")
    /// * `category` - Optional product category (spot, linear, inverse)
    /// * `limit` - Number of order book levels to return (1-50)
    ///
    /// # Panics
    ///
    /// Panics if `limit` is not between 1 and 50 inclusive.
    pub fn new(symbol: &'a str, category: Option<Category>, limit: u64) -> RPIOrderbookRequest<'a> {
        // Validate limit parameter
        if limit == 0 || limit > 50 {
            panic!("RPI orderbook limit must be between 1 and 50 inclusive");
        }

        RPIOrderbookRequest {
            symbol: Cow::Borrowed(symbol),
            category,
            limit,
        }
    }

    /// Constructs a new RPIOrderbook request with specified parameters, returning a Result.
    ///
    /// Similar to `new`, but returns a Result instead of panicking on invalid parameters.
    /// This is the recommended method for production code.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The trading pair symbol (e.g., "BTCUSDT")
    /// * `category` - Optional product category (spot, linear, inverse)
    /// * `limit` - Number of order book levels to return (1-50)
    ///
    /// # Returns
    ///
    /// Returns `Ok(RPIOrderbookRequest)` if parameters are valid, or `Err(String)` with an error message.
    pub fn try_new(
        symbol: &'a str,
        category: Option<Category>,
        limit: u64,
    ) -> Result<RPIOrderbookRequest<'a>, String> {
        // Validate limit parameter
        if limit == 0 || limit > 50 {
            return Err("RPI orderbook limit must be between 1 and 50 inclusive".to_string());
        }

        Ok(RPIOrderbookRequest {
            symbol: Cow::Borrowed(symbol),
            category,
            limit,
        })
    }

    /// Creates an RPIOrderbookRequest for spot trading.
    ///
    /// Convenience method for creating requests for spot markets.
    pub fn spot(symbol: &'a str, limit: u64) -> Result<RPIOrderbookRequest<'a>, String> {
        Self::try_new(symbol, Some(Category::Spot), limit)
    }

    /// Creates an RPIOrderbookRequest for linear perpetual futures.
    ///
    /// Convenience method for creating requests for USDT-margined perpetual futures.
    pub fn linear(symbol: &'a str, limit: u64) -> Result<RPIOrderbookRequest<'a>, String> {
        Self::try_new(symbol, Some(Category::Linear), limit)
    }

    /// Creates an RPIOrderbookRequest for inverse perpetual futures.
    ///
    /// Convenience method for creating requests for coin-margined perpetual futures.
    pub fn inverse(symbol: &'a str, limit: u64) -> Result<RPIOrderbookRequest<'a>, String> {
        Self::try_new(symbol, Some(Category::Inverse), limit)
    }
}
