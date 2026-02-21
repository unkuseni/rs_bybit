use crate::prelude::*;

/// Parameters for requesting ADL (Auto-Deleveraging) alert data.
///
/// This struct defines the parameters for querying ADL alerts via the
/// `/v5/market/adlAlert` endpoint. ADL is a risk management mechanism
/// that automatically closes positions when the insurance pool balance
/// reaches certain thresholds to prevent systemic risk.
///
/// # Important Notes
/// - Data update frequency is every 1 minute
/// - Covers: USDT Perpetual, USDT Delivery, USDC Perpetual, USDC Delivery, Inverse Contracts
/// - The `symbol` parameter is optional; if not provided, returns all symbols
#[derive(Clone, Default)]
pub struct ADLAlertRequest<'a> {
    /// The trading symbol (e.g., "BTCUSDT").
    ///
    /// Specifies the contract to filter ADL alerts. This parameter is optional.
    /// If not provided, the endpoint returns ADL alerts for all symbols.
    /// Must be in uppercase.
    pub symbol: Option<Cow<'a, str>>,
}

impl<'a> ADLAlertRequest<'a> {
    /// Creates a default ADLAlert request.
    ///
    /// Returns a request with no symbol filter (returns all symbols).
    /// Suitable for getting a complete overview of ADL alerts across all symbols.
    pub fn default() -> ADLAlertRequest<'a> {
        ADLAlertRequest::new(None)
    }

    /// Constructs a new ADLAlert request with specified parameters.
    ///
    /// Allows customization of the request parameters. Bots should use this to
    /// specify an optional symbol filter for ADL alert queries.
    ///
    /// # Arguments
    ///
    /// * `symbol` - Optional symbol filter (e.g., "BTCUSDT", "ETHUSDT")
    pub fn new(symbol: Option<&'a str>) -> ADLAlertRequest<'a> {
        ADLAlertRequest {
            symbol: symbol.map(Cow::Borrowed),
        }
    }

    /// Constructs a new ADLAlert request for a specific symbol.
    ///
    /// Convenience method for creating requests filtered by a specific symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to filter by (e.g., "BTCUSDT", "ETHUSDT")
    pub fn for_symbol(symbol: &'a str) -> ADLAlertRequest<'a> {
        ADLAlertRequest::new(Some(symbol))
    }

    /// Constructs a new ADLAlert request for all symbols.
    ///
    /// Convenience method for creating requests without symbol filtering.
    pub fn all_symbols() -> ADLAlertRequest<'a> {
        ADLAlertRequest::new(None)
    }

    /// Sets the symbol filter for the request.
    ///
    /// Returns a new request with the specified symbol filter.
    pub fn with_symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(Cow::Borrowed(symbol));
        self
    }

    /// Removes the symbol filter from the request.
    ///
    /// Returns a new request without symbol filtering.
    pub fn without_symbol(mut self) -> Self {
        self.symbol = None;
        self
    }

    /// Creates an ADLAlertRequest for BTCUSDT.
    ///
    /// Convenience method for creating requests for BTCUSDT.
    pub fn btcusdt() -> ADLAlertRequest<'a> {
        ADLAlertRequest::for_symbol("BTCUSDT")
    }

    /// Creates an ADLAlertRequest for ETHUSDT.
    ///
    /// Convenience method for creating requests for ETHUSDT.
    pub fn ethusdt() -> ADLAlertRequest<'a> {
        ADLAlertRequest::for_symbol("ETHUSDT")
    }

    /// Creates an ADLAlertRequest for SOLUSDT.
    ///
    /// Convenience method for creating requests for SOLUSDT.
    pub fn solusdt() -> ADLAlertRequest<'a> {
        ADLAlertRequest::for_symbol("SOLUSDT")
    }

    /// Creates an ADLAlertRequest for XRPUSDT.
    ///
    /// Convenience method for creating requests for XRPUSDT.
    pub fn xrpusdt() -> ADLAlertRequest<'a> {
        ADLAlertRequest::for_symbol("XRPUSDT")
    }

    /// Creates an ADLAlertRequest for ADAUSDT.
    ///
    /// Convenience method for creating requests for ADAUSDT.
    pub fn adausdt() -> ADLAlertRequest<'a> {
        ADLAlertRequest::for_symbol("ADAUSDT")
    }
}
