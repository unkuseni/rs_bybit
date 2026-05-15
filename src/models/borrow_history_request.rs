use crate::prelude::*;

/// Parameters for requesting borrow history data.
///
/// Used to construct a request to the `/v5/account/borrow-history` endpoint to retrieve historical borrowing records for an account. Bots use this to audit borrowing activity, calculate interest costs, and manage leverage in perpetual futures trading.
#[derive(Clone, Debug, Default)]
pub struct BorrowHistoryRequest<'a> {
    /// The currency to filter borrow history (e.g., "USDT") (optional).
    ///
    /// Optionally filters borrowing records by currency. If unset, data for all currencies is returned. Bots should specify this to focus on specific margin types.
    pub coin: Option<Cow<'a, str>>,

    /// The start time for the borrow history (Unix timestamp in milliseconds) (optional).
    ///
    /// Defines the beginning of the time range. Bots should set this to focus on a specific historical period.
    pub start_time: Option<u64>,

    /// The end time for the borrow history (Unix timestamp in milliseconds) (optional).
    ///
    /// Defines the end of the time range. Bots should set this to limit data to a specific period, optimizing performance.
    pub end_time: Option<u64>,

    /// The maximum number of borrow records to return (optional).
    ///
    /// Controls the number of records returned as a string (e.g., "50"). Bots should set a reasonable limit to balance data completeness with performance.
    pub limit: Option<Cow<'a, str>>,
}

impl<'a> BorrowHistoryRequest<'a> {
    /// Constructs a new BorrowHistory request with specified parameters.
    ///
    /// Allows customization of the borrow history request. Bots should use this to specify the currency, time range, and limit to align with their analysis needs.
    pub fn new(
        coin: Option<&'a str>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<&'a str>,
    ) -> Self {
        Self {
            coin: coin.map(Cow::Borrowed),
            start_time,
            end_time,
            limit: limit.map(Cow::Borrowed),
        }
    }
    /// Creates a default BorrowHistory request.
    ///
    /// Returns a request with all fields unset. Suitable for broad queries but should be customized for specific analysis needs.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> BorrowHistoryRequest<'a> {
        BorrowHistoryRequest::new(None, None, None, None)
    }
}
