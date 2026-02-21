use crate::prelude::*;

/// Parameters for requesting ticker information.
///
/// This struct defines the parameters for querying ticker data via the `/v5/market/tickers` endpoint.
/// Ticker data includes the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.
#[derive(Clone, Default)]
pub struct TickerRequest<'a> {
    /// The product category (e.g., Linear, Inverse, Spot, Option).
    ///
    /// Specifies the instrument type. This parameter is required for all ticker requests.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Optionally specifies a single trading pair. If unset, the API returns data for all instruments
    /// in the category. For `option` category, either `symbol` or `base_coin` must be provided.
    pub symbol: Option<Cow<'a, str>>,

    /// Base coin, uppercase only.
    ///
    /// Applies to `option` category only. When querying options, either `symbol` or `base_coin` must be provided.
    /// This parameter is ignored for other categories.
    pub base_coin: Option<Cow<'a, str>>,

    /// Expiry date for options contracts.
    ///
    /// Applies to `option` category only. Format: e.g., "25DEC22". Used to filter options by expiry date.
    /// This parameter is ignored for other categories.
    pub exp_date: Option<Cow<'a, str>>,
}

impl<'a> TickerRequest<'a> {
    /// Creates a default Ticker request.
    ///
    /// Returns a request with `category` set to `Linear` and `symbol` set to `"BTCUSDT"`.
    /// Suitable for testing but should be customized for production to match specific trading needs.
    pub fn default() -> TickerRequest<'a> {
        TickerRequest::new(Category::Linear, Some("BTCUSDT"), None, None)
    }

    /// Constructs a new Ticker request with specified parameters.
    ///
    /// Allows full customization. Bots should use this to tailor requests to their strategy,
    /// ensuring `category` and `symbol` align with the instruments being traded.
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        base_coin: Option<&'a str>,
        exp_date: Option<&'a str>,
    ) -> TickerRequest<'a> {
        TickerRequest {
            category,
            symbol: symbol.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            exp_date: exp_date.map(Cow::Borrowed),
        }
    }

    /// Validates the request parameters according to API constraints.
    ///
    /// Returns `Ok(())` if the request is valid, or `Err(String)` with an error message.
    pub fn validate(&self) -> Result<(), String> {
        // For option category, either symbol or base_coin must be provided
        if self.category == Category::Option && self.symbol.is_none() && self.base_coin.is_none() {
            return Err(
                "Option category requires either symbol or base_coin parameter".to_string(),
            );
        }

        Ok(())
    }

    /// Creates a request for linear perpetual futures.
    ///
    /// Convenience method for creating requests for USDT-margined perpetual futures.
    pub fn linear(symbol: Option<&'a str>) -> TickerRequest<'a> {
        TickerRequest::new(Category::Linear, symbol, None, None)
    }

    /// Creates a request for inverse perpetual futures.
    ///
    /// Convenience method for creating requests for coin-margined perpetual futures.
    pub fn inverse(symbol: Option<&'a str>) -> TickerRequest<'a> {
        TickerRequest::new(Category::Inverse, symbol, None, None)
    }

    /// Creates a request for spot trading pairs.
    ///
    /// Convenience method for creating requests for spot markets.
    pub fn spot(symbol: Option<&'a str>) -> TickerRequest<'a> {
        TickerRequest::new(Category::Spot, symbol, None, None)
    }

    /// Creates a request for options contracts.
    ///
    /// Convenience method for creating requests for options markets.
    pub fn option(
        symbol: Option<&'a str>,
        base_coin: Option<&'a str>,
        exp_date: Option<&'a str>,
    ) -> TickerRequest<'a> {
        TickerRequest::new(Category::Option, symbol, base_coin, exp_date)
    }

    /// Creates a request for options by base coin.
    ///
    /// Convenience method for creating requests for options markets filtered by base coin.
    pub fn option_by_base_coin(base_coin: &'a str, exp_date: Option<&'a str>) -> TickerRequest<'a> {
        TickerRequest::new(Category::Option, None, Some(base_coin), exp_date)
    }
}
