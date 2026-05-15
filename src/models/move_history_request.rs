use crate::prelude::*;

/// Parameters for requesting position move history.
///
/// Used to construct a request to the `/v5/position/move-history` endpoint to retrieve historical data on position transfers between accounts. Bots use this to audit past transfers, verify portfolio changes, and ensure compliance with trading rules.
#[derive(Serialize, Clone, Default)]
pub struct MoveHistoryRequest<'a> {
    /// The product category (e.g., Linear, Inverse) (optional).
    ///
    /// Optionally filters transfer history by instrument type. If unset, data for all categories is returned. Bots should specify this for targeted analysis.
    pub category: Option<Category>,

    /// The trading pair symbol (e.g., "BTCUSDT") (optional).
    ///
    /// Optionally filters transfer history by symbol. If unset, data for all symbols is returned. Bots should specify this to focus on specific contracts.
    pub symbol: Option<Cow<'a, str>>,

    /// The start time for the transfer history (Unix timestamp in milliseconds) (optional).
    ///
    /// Defines the beginning of the time range. Bots should set this to focus on a specific historical period.
    pub start_time: Option<u64>,

    /// The end time for the transfer history (Unix timestamp in milliseconds) (optional).
    ///
    /// Defines the end of the time range. Bots should set this to limit data to a specific period, optimizing performance.
    pub end_time: Option<u64>,

    /// The status of the transfers (e.g., "Filled", "Rejected") (optional).
    ///
    /// Optionally filters transfers by status. Bots can use this to analyze successful or failed transfers separately.
    pub status: Option<Cow<'a, str>>,

    /// The block trade ID (optional).
    ///
    /// Optionally filters transfers by a specific block trade ID. Bots can use this to retrieve details of a particular transfer event.
    pub block_trade_id: Option<Cow<'a, str>>,

    /// The maximum number of transfer records to return (optional).
    ///
    /// Controls the number of records returned as a string (e.g., "50"). Bots should set a reasonable limit to balance data completeness with performance.
    pub limit: Option<Cow<'a, str>>,
}

impl<'a> MoveHistoryRequest<'a> {
    /// Constructs a new MoveHistory request with specified parameters.
    ///
    /// Allows customization of the transfer history request. Bots should use this to specify the exact category, symbol, time range, and filters to align with their analysis needs.
    pub fn new(
        category: Option<Category>,
        symbol: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        status: Option<&'a str>,
        block_trade_id: Option<&'a str>,
        limit: Option<&'a str>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.map(Cow::Borrowed),
            start_time,
            end_time,
            status: status.map(Cow::Borrowed),
            block_trade_id: block_trade_id.map(Cow::Borrowed),
            limit: limit.map(Cow::Borrowed),
        }
    }
    /// Creates a default MoveHistory request.
    ///
    /// Returns a request with all fields unset. Suitable for broad queries but should be customized for specific analysis needs.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> MoveHistoryRequest<'a> {
        MoveHistoryRequest::new(None, None, None, None, None, None, None)
    }
}
