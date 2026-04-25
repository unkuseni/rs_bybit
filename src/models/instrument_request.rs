use crate::prelude::*;

/// Parameters for requesting instrument information.
///
/// This struct defines the parameters for querying instrument details via the `/v5/market/instruments-info` endpoint.
/// For perpetual futures, instrument info includes leverage, price filters, and lot size filters, which are critical
/// for configuring trading bot parameters.
#[derive(Clone, Default)]
pub struct InstrumentRequest<'a> {
    /// The product category (e.g., Linear, Inverse, Spot, Option).
    ///
    /// Specifies the instrument type. For perpetual futures, use `Linear` or `Inverse`.
    /// Bots must set this to filter relevant instruments.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Optionally specifies a single trading pair. If unset, the API returns data for all instruments
    /// in the category, which may be voluminous. Bots should set this for specific pairs to reduce
    /// response size and latency.
    pub symbol: Option<Cow<'a, str>>,

    /// Symbol type filter (e.g., "spot", "linear", "inverse").
    ///
    /// The region to which the trading pair belongs. Only applicable for `linear`, `inverse`, and `spot` categories.
    /// This can be used to filter instruments by their regional classification.
    pub symbol_type: Option<Cow<'a, str>>,

    /// Instrument status filter.
    ///
    /// Filters instruments by their trading status:
    /// - `linear` & `inverse` & `spot`: By default returns only `Trading` symbols
    /// - `option`: By default returns `PreLaunch`, `Trading`, and `Delivering`
    /// - Spot has `Trading` only
    /// - `linear` & `inverse`: when status=PreLaunch, it returns Pre-Market contracts
    pub status: Option<Cow<'a, str>>,

    /// The base coin of the instrument (e.g., "BTC").
    ///
    /// Filters instruments by their base asset. For example, setting `base_coin` to `"BTC"` returns
    /// all BTC-based instruments. Applies to `linear`, `inverse`, and `option` categories only.
    /// For `option` category, returns BTC by default if not specified.
    pub base_coin: Option<Cow<'a, str>>,

    /// The maximum number of instruments to return (1-1000, default: 500).
    ///
    /// Controls the response size. Bots should set a reasonable limit to balance data completeness
    /// and performance, especially when querying all instruments in a category.
    /// Note: Spot does not support pagination, so `limit` and `cursor` are invalid for spot category.
    pub limit: Option<u64>,

    /// Cursor for pagination.
    ///
    /// Use the `nextPageCursor` token from the response to retrieve the next page of the result set.
    /// Note: Spot does not support pagination, so `limit` and `cursor` are invalid for spot category.
    pub cursor: Option<Cow<'a, str>>,
}

impl<'a> InstrumentRequest<'a> {
    /// Creates a default Instrument request.
    ///
    /// Returns a request with `category` set to `Linear` and `symbol` set to `"BTCUSDT"`.
    /// Suitable for testing but should be customized for production to match specific trading needs.
    pub fn default() -> InstrumentRequest<'a> {
        InstrumentRequest::new(
            Category::Linear,
            Some("BTCUSDT"),
            None,
            None,
            None,
            None,
            None,
        )
    }

    /// Constructs a new Instrument request with specified parameters.
    ///
    /// Allows full customization. Bots should use this to tailor requests to their strategy,
    /// ensuring `category` and `symbol` align with the instruments being traded.
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        symbol_type: Option<&'a str>,
        status: Option<&'a str>,
        base_coin: Option<&'a str>,
        limit: Option<u64>,
        cursor: Option<&'a str>,
    ) -> InstrumentRequest<'a> {
        InstrumentRequest {
            category,
            symbol: symbol.map(Cow::Borrowed),
            symbol_type: symbol_type.map(Cow::Borrowed),
            status: status.map(Cow::Borrowed),
            base_coin: base_coin.map(Cow::Borrowed),
            limit,
            cursor: cursor.map(Cow::Borrowed),
        }
    }

    /// Validates the request parameters according to API constraints.
    ///
    /// Returns `Ok(())` if the request is valid, or `Err(String)` with an error message.
    pub fn validate(&self) -> Result<(), String> {
        // Validate limit range
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 1000 {
                return Err("Limit must be between 1 and 1000 inclusive".to_string());
            }
        }

        // Validate category-specific constraints
        match self.category {
            Category::Spot
                // Spot does not support pagination
                if (self.limit.is_some() || self.cursor.is_some()) => {
                    return Err(
                        "Spot category does not support limit or cursor parameters".to_string()
                    );
                }
            Category::Option
                // Option requires either symbol or base_coin
                if self.symbol.is_none() && self.base_coin.is_none() => {
                    return Err(
                        "Option category requires either symbol or base_coin parameter".to_string(),
                    );
                }
            _ => {} // Linear and Inverse have no special validation
        }

        Ok(())
    }

    /// Creates a request for linear perpetual futures.
    ///
    /// Convenience method for creating requests for USDT-margined perpetual futures.
    pub fn linear(
        symbol: Option<&'a str>,
        symbol_type: Option<&'a str>,
        status: Option<&'a str>,
        base_coin: Option<&'a str>,
        limit: Option<u64>,
        cursor: Option<&'a str>,
    ) -> InstrumentRequest<'a> {
        InstrumentRequest::new(
            Category::Linear,
            symbol,
            symbol_type,
            status,
            base_coin,
            limit,
            cursor,
        )
    }

    /// Creates a request for inverse perpetual futures.
    ///
    /// Convenience method for creating requests for coin-margined perpetual futures.
    pub fn inverse(
        symbol: Option<&'a str>,
        symbol_type: Option<&'a str>,
        status: Option<&'a str>,
        base_coin: Option<&'a str>,
        limit: Option<u64>,
        cursor: Option<&'a str>,
    ) -> InstrumentRequest<'a> {
        InstrumentRequest::new(
            Category::Inverse,
            symbol,
            symbol_type,
            status,
            base_coin,
            limit,
            cursor,
        )
    }

    /// Creates a request for spot trading pairs.
    ///
    /// Convenience method for creating requests for spot markets.
    pub fn spot(
        symbol: Option<&'a str>,
        symbol_type: Option<&'a str>,
        status: Option<&'a str>,
    ) -> InstrumentRequest<'a> {
        InstrumentRequest::new(
            Category::Spot,
            symbol,
            symbol_type,
            status,
            None, // base_coin not applicable for spot
            None, // limit not supported for spot
            None, // cursor not supported for spot
        )
    }

    /// Creates a request for options contracts.
    ///
    /// Convenience method for creating requests for options markets.
    pub fn option(
        symbol: Option<&'a str>,
        status: Option<&'a str>,
        base_coin: Option<&'a str>,
        limit: Option<u64>,
        cursor: Option<&'a str>,
    ) -> InstrumentRequest<'a> {
        InstrumentRequest::new(
            Category::Option,
            symbol,
            None, // symbol_type not applicable for options
            status,
            base_coin,
            limit,
            cursor,
        )
    }
}
