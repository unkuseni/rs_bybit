use crate::prelude::*;

/// Parameters for requesting funding rate history.
///
/// This struct defines the parameters for querying funding rate history via the `/v5/market/funding/history` endpoint. Funding rates are critical in perpetual futures as they balance long and short positions and affect position holding costs.
#[derive(Clone, Default)]
pub struct FundingHistoryRequest<'a> {
    /// The product category (e.g., Linear, Inverse).
    ///
    /// Specifies the instrument type. For perpetual futures, use `Linear` or `Inverse`. Bots must set this correctly.
    pub category: Category,

    /// The trading pair symbol (e.g., "BTCUSDT").
    ///
    /// Identifies the perpetual futures contract. Bots must specify a valid symbol.
    pub symbol: Cow<'a, str>,

    /// The start time for the funding history (Unix timestamp in milliseconds).
    ///
    /// Defines the beginning of the time range. Bots should set this for historical analysis of funding costs.
    pub start_time: Option<u64>,

    /// The end time for the funding history (Unix timestamp in milliseconds).
    ///
    /// Defines the end of the time range. Bots should set this to limit data to a specific period.
    pub end_time: Option<u64>,

    /// The maximum number of records to return (1-200, default: 200).
    ///
    /// Controls the response size. Bots should set a reasonable limit to balance data completeness and performance.
    pub limit: Option<u64>,
}

impl<'a> FundingHistoryRequest<'a> {
    /// Creates a default FundingHistory request.
    ///
    /// Returns a request with `category` set to `Linear` and `symbol` set to `"BTCUSDT"`. Suitable for testing but should be customized for production.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> FundingHistoryRequest<'a> {
        FundingHistoryRequest::new(Category::Linear, "BTCUSDT", None, None, None)
    }
    /// Constructs a new FundingHistory request with specified parameters.
    ///
    /// Allows customization. Bots should use this to specify the exact symbol and time range for funding rate analysis.
    pub fn new(
        category: Category,
        symbol: &'a str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u64>,
    ) -> FundingHistoryRequest<'a> {
        FundingHistoryRequest {
            category,
            symbol: Cow::Borrowed(symbol),
            start_time,
            end_time,
            limit,
        }
    }
}
