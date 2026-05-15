use crate::prelude::*;

/// Parameters for requesting closed profit and loss (P&L) data.
///
/// Used to construct a request to the `/v5/position/closed-pnl` endpoint to retrieve historical P&L data for closed positions. Bots use this to analyze trading performance, calculate realized profits, and refine strategies for perpetual futures.
#[derive(Clone, Default)]
pub struct ClosedPnlRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. Bots must set this to fetch P&L data for the correct contract type.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT") (optional).
    ///
    /// Optionally filters P&L data by symbol. If unset, data for all symbols in the category is returned. Bots should specify this for targeted performance analysis.
    pub symbol: Option<Cow<'a, str>>,

    /// The start time for the P&L data (Unix timestamp in milliseconds) (optional).
    ///
    /// Defines the beginning of the time range. Bots should set this to focus on a specific historical period, such as a trading session or month.
    pub start_time: Option<u64>,

    /// The end time for the P&L data (Unix timestamp in milliseconds) (optional).
    ///
    /// Defines the end of the time range. Bots should set this to limit data to a specific period, optimizing performance.
    pub end_time: Option<u64>,

    /// The maximum number of P&L records to return (optional).
    ///
    /// Controls the number of records returned (e.g., max 50). Bots should set a reasonable limit to balance data completeness with performance.
    pub limit: Option<u64>,
}

impl<'a> ClosedPnlRequest<'a> {
    /// Constructs a new ClosedPnl request with specified parameters.
    ///
    /// Allows customization of the P&L request. Bots should use this to specify the exact symbol, category, time range, and limit to align with their analysis needs.
    pub fn new(
        category: Category,
        symbol: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.map(Cow::Borrowed),
            start_time,
            end_time,
            limit,
        }
    }
    /// Creates a default ClosedPnl request.
    ///
    /// Returns a request with `category` set to `Linear` and all other fields unset. Suitable for broad queries but should be customized for specific analysis needs.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> ClosedPnlRequest<'a> {
        ClosedPnlRequest::new(Category::Linear, None, None, None, None)
    }
}
